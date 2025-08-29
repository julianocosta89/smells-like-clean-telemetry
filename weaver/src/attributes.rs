// DO NOT EDIT, this is an auto-generated file

/// The name of the album containing the song.
///
/// ## Examples
///
/// - `"Nevermind"`
/// - `"In Utero"`
/// - `"Bleach"`
/// - `"Incesticide"`
pub const MEDIA_ALBUM_NAME: &str = "media.album.name";

/// The name of the artist performing the song.
///
/// ## Examples
///
/// - `"Nirvana"`
/// - `"Foo Fighters"`
/// - `"Linkin Park"`
/// - `"Slipknot"`
pub const MEDIA_ARTIST_NAME: &str = "media.artist.name";

/// The name/title of the song being queried.
///
/// ## Notes
///
/// This attribute is deprecated and will be removed in future versions. Use `media.song.name` for new implementations.
///
/// ## Examples
///
/// - `"Smells Like Teen Spirit"`
/// - `"Come As You Are"`
/// - `"Lithium"`
/// - `"Something In The Way"`
#[deprecated(note="{note: Replaced by `media.song.name`., reason: renamed, renamed_to: media.song.name}")]
pub const MEDIA_SONG: &str = "media.song";

/// The duration of the song in milliseconds.
///
/// ## Examples
///
/// - `301000`
/// - `355000`
/// - `183000`
/// - `391000`
pub const MEDIA_SONG_DURATION_MS: &str = "media.song.duration_ms";

/// The musical genre of the song.
///
/// ## Examples
///
/// - `"Rock"`
/// - `"Punk"`
/// - `"Grunge"`
/// - `"Classical"`
/// - `"Hip Hop"`
pub const MEDIA_SONG_GENRE: &str = "media.song.genre";

/// The name/title of the song being queried.
///
/// ## Examples
///
/// - `"Smells Like Teen Spirit"`
/// - `"Come As You Are"`
/// - `"Lithium"`
/// - `"Something In The Way"`
pub const MEDIA_SONG_NAME: &str = "media.song.name";

/// The release year of the song.
///
/// ## Examples
///
/// - `1991`
/// - `1975`
/// - `1971`
/// - `1976`
pub const MEDIA_SONG_YEAR: &str = "media.song.year";    


/// Unique identifier of the user.
///
/// ## Examples
///
/// - `"S-1-5-21-202424912787-2692429404-2351956786-1000"`
pub const USER_ID: &str = "user.id";

/// The type of subscription the user has.
///
/// ## Examples
///
/// - `"Free"`
/// - `"Premium"`
/// - `"Family"`
/// - `"Student"`
pub const USER_SUBSCRIPTION_TYPE: &str = "user.subscription.type";    
