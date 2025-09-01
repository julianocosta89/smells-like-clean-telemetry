use awc::Client;
use log::{error, info};
use opentelemetry::Context;
use opentelemetry::trace::{Span, SpanKind, Status, Tracer, get_active_span};
use opentelemetry_instrumentation_actix_web::ClientExt;
use opentelemetry_semantic_conventions::attribute::{
    DB_OPERATION_NAME, DB_QUERY_TEXT, DB_SYSTEM_NAME, USER_ID,
};
use serde::Deserialize;
use std::{io, vec};
use tokio_postgres::{Client as PgClient, NoTls, Row};
use urlencoding;

#[derive(Debug, Deserialize)]
struct MusicBrainzResponse {
    recordings: Vec<Recording>,
}

#[derive(Debug, Deserialize)]
struct Recording {
    id: String,
    title: String,
    length: Option<u32>,
    releases: Option<Vec<Release>>,
    tags: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize)]
struct Release {
    id: String,
    title: String,
    date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Tag {
    name: String,
}

fn extract_year(date_string: &str) -> Option<i32> {
    if date_string.is_empty() {
        return None;
    }

    // Handle various date formats: YYYY, YYYY-MM, YYYY-MM-DD
    let parts: Vec<&str> = date_string.split('-').collect();
    if !parts.is_empty() && parts[0].len() == 4 {
        parts[0].parse::<i32>().ok()
    } else {
        None
    }
}

async fn get_db_client() -> Result<PgClient, String> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return Err("DATABASE_URL environment variable is not set".to_string()),
    };

    let (client, connection) = match tokio_postgres::connect(&database_url, NoTls).await {
        Ok((client, connection)) => (client, connection),
        Err(_) => return Err("Database connection failed".to_string()),
    };

    // Spawn the connection to run in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn find_by_title_and_artist(title: &str, artist: &str) -> Result<String, String> {
    get_active_span(|span| {
        span.set_attributes(vec![
            opentelemetry::KeyValue::new("media.song.name", title.to_string()),
            opentelemetry::KeyValue::new("media.artist.name", artist.to_string()),
            opentelemetry::KeyValue::new(USER_ID, "12345".to_string()),
        ]);
    });

    match get_song_from_db(title, artist).await {
        Ok(rows) if !rows.is_empty() => {
            let row = &rows[0];
            let title: String = row.get("title");
            let artist: String = row.get("artist");
            let album: String = row.get("album");
            let year: Option<i32> = row.get("year");
            let duration_ms: Option<i32> = row.get("duration_ms");
            let genre: String = row.get("genre");

            return Ok(format!(
                r#"{{"title":"{}","artist":"{}","album":"{}","year":{},"duration_ms":{},"genre":"{}"}}"#,
                title,
                artist,
                album,
                year.map_or("null".to_string(), |y| y.to_string()),
                duration_ms.map_or("null".to_string(), |d| d.to_string()),
                genre
            ));
        }
        Ok(_) => {} // Empty result, continue to external API
        Err(e) => {
            error!("Database query failed: {:?}", e);
        }
    }

    match get_song_from_musicbrainz(title, artist).await {
        Ok(song_data) => {
            // Store the song in database
            match insert_song(
                title,
                artist,
                &song_data.0,
                song_data.1,
                song_data.2,
                &song_data.3,
            )
            .await
            {
                Ok(_) => Ok(format!(
                    r#"{{"title":"{}","artist":"{}","album":"{}","year":{},"duration_ms":{},"genre":"{}"}}"#,
                    title,
                    artist,
                    song_data.0,
                    song_data.1.map_or("null".to_string(), |y| y.to_string()),
                    song_data.2.map_or("null".to_string(), |d| d.to_string()),
                    song_data.3
                )),
                Err(_) => Ok(format!(
                    r#"{{"title":"{}","artist":"{}","album":"{}","year":{},"duration_ms":{},"genre":"{}"}}"#,
                    title,
                    artist,
                    song_data.0,
                    song_data.1.map_or("null".to_string(), |y| y.to_string()),
                    song_data.2.map_or("null".to_string(), |d| d.to_string()),
                    song_data.3
                )),
            }
        }
        Err(_) => Ok(format!(
            r#"{{"error":"Song not found for title: {}, artist: {}"}}"#,
            title, artist
        )),
    }
}

async fn get_song_from_musicbrainz(
    title: &str,
    artist: &str,
) -> io::Result<(String, Option<i32>, Option<i32>, String)> {
    let client = Client::new();

    let music_service_url = match std::env::var("MUSIC_SERVICE_URL") {
        Ok(url) => url,
        Err(_) => "https://musicbrainz.org/ws/2/recording".to_string(),
    };

    let query = format!(
        "recording:\"{}\" AND artist:\"{}\"",
        title.replace("\"", "\\\""),
        artist.replace("\"", "\\\"")
    );
    let encoded_query = urlencoding::encode(&query);
    let url = format!(
        "{}?query={}&fmt=json&limit=20",
        music_service_url, encoded_query
    );

    info!("Requesting song from: {}", url);

    let mut response = client
        .get(&url)
        .insert_header(("User-Agent", "otel-demo/1.0"))
        .trace_request()
        .send()
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;

    let bytes = response
        .body()
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;

    let json_str =
        std::str::from_utf8(&bytes).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    let musicbrainz_response: MusicBrainzResponse =
        serde_json::from_str(json_str).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    if musicbrainz_response.recordings.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No recordings found",
        ));
    }

    let recording = &musicbrainz_response.recordings[0];

    // Extract album and year, prioritizing studio releases over live recordings
    let mut album = "Unknown".to_string();
    let mut year = None;

    if let Some(releases) = &recording.releases {
        let mut best_release = None;
        let mut oldest_year = None;

        // Find the release with the oldest year (any year is better than null)
        for release in releases {
            if let Some(date) = &release.date {
                let release_year = extract_year(date);

                if let Some(year_val) = release_year {
                    let should_update = oldest_year.is_none() || year_val < oldest_year.unwrap();

                    if should_update {
                        oldest_year = Some(year_val);
                        best_release = Some(release);
                    }
                }
            }
        }

        // Use the release with oldest year, or first release if no dates found
        if let Some(release) = best_release.or_else(|| releases.first()) {
            album = release.title.clone();
            year = oldest_year;
        }
    }

    // Extract genre from tags - search all recordings if needed
    let genre = recording
        .tags
        .as_ref()
        .and_then(|tags| tags.first())
        .map(|tag| tag.name.clone())
        .or_else(|| {
            // Look through all recordings for tags
            musicbrainz_response
                .recordings
                .iter()
                .find_map(|r| r.tags.as_ref()?.first())
                .map(|tag| tag.name.clone())
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let duration_ms = recording.length.map(|l| l as i32);

    Ok((album, year, duration_ms, genre))
}

async fn get_song_from_db(title: &str, artist: &str) -> Result<Vec<Row>, String> {
    let tracer = opentelemetry::global::tracer("music_service");
    let parent_cx = Context::current();

    let mut span = tracer
        .span_builder("SELECT songs_db.songs")
        .with_kind(SpanKind::Client)
        .with_attributes(vec![
            opentelemetry::KeyValue::new(DB_SYSTEM_NAME, "postgresql"),
            opentelemetry::KeyValue::new(
                DB_QUERY_TEXT,
                "SELECT title, artist, album, year, duration_ms, genre FROM songs WHERE title ILIKE $1 AND artist ILIKE $2",
            ),
            opentelemetry::KeyValue::new(DB_OPERATION_NAME, "SELECT"),
        ])
        .start_with_context(&tracer, &parent_cx);

    let client = match get_db_client().await {
        Ok(client) => client,
        Err(e) => {
            opentelemetry::trace::Span::set_status(
                &mut span,
                Status::error("Failed to get DB client"),
            );
            opentelemetry::trace::Span::end(&mut span);
            return Err(format!("Database connection failed: {}", e));
        }
    };

    let result = client
        .query(
            "SELECT title, artist, album, year, duration_ms, genre FROM songs WHERE title ILIKE $1 AND artist ILIKE $2",
            &[&title, &artist],
        )
        .await;

    match &result {
        Ok(rows) => {
            span.set_attribute(opentelemetry::KeyValue::new(
                "db.rows_affected",
                rows.len() as i64,
            ));
        }
        Err(e) => {
            opentelemetry::trace::Span::set_status(
                &mut span,
                Status::error(format!("Query failed: {}", e)),
            );
        }
    }

    opentelemetry::trace::Span::end(&mut span);
    result.map_err(|e| format!("Database query failed: {}", e))
}

async fn insert_song(
    title: &str,
    artist: &str,
    album: &str,
    year: Option<i32>,
    duration_ms: Option<i32>,
    genre: &str,
) -> Result<String, String> {
    let tracer = opentelemetry::global::tracer("music_service");
    let parent_cx = Context::current();

    let mut span = tracer.span_builder("INSERT songs_db.songs")
        .with_kind(SpanKind::Client)
        .with_attributes(vec![
            opentelemetry::KeyValue::new(DB_SYSTEM_NAME, "postgresql"),
            opentelemetry::KeyValue::new(
                DB_QUERY_TEXT,
                "INSERT INTO songs (title, artist, album, year, duration_ms, genre) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (title, artist) DO NOTHING",
            ),
            opentelemetry::KeyValue::new(DB_OPERATION_NAME, "INSERT"),
        ])
        .start_with_context(&tracer, &parent_cx);

    let client = match get_db_client().await {
        Ok(client) => client,
        Err(e) => {
            opentelemetry::trace::Span::set_status(
                &mut span,
                Status::error("Failed to get DB client"),
            );
            opentelemetry::trace::Span::end(&mut span);
            return Err(format!("Database connection failed: {}", e));
        }
    };

    match client
        .execute(
            "INSERT INTO songs (title, artist, album, year, duration_ms, genre) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (title, artist) DO NOTHING",
            &[&title, &artist, &album, &year, &duration_ms, &genre],
        )
        .await
    {
        Ok(_) => {
            opentelemetry::trace::Span::end(&mut span);
            Ok(format!(
                "Inserted: Title: {}, Artist: {}, Album: {}, Year: {:?}, Duration: {:?}ms, Genre: {}",
                title, artist, album, year, duration_ms, genre
            ))
        }
        Err(e) => {
            opentelemetry::trace::Span::set_status(
                &mut span,
                Status::error(format!("Insert failed: {}", e)),
            );
            opentelemetry::trace::Span::end(&mut span);
            error!("Insert failed: {:?}", e);
            Err("Insert failed".to_string())
        }
    }
}
