// DO NOT EDIT, this is an auto-generated file

package dev.jcosta.semconv;

public final class Attributes {
    /**
     * <p>The name of the album containing the song.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"Nevermind"</c></li>
     *   <li><c>"In Utero"</c></li>
     *   <li><c>"Bleach"</c></li>
     *   <li><c>"Incesticide"</c></li>
     * </ul>
     */
    public static final String MEDIA_ALBUM_NAME = "media.album.name";

    /**
     * <p>The name of the artist performing the song.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"Nirvana"</c></li>
     *   <li><c>"Foo Fighters"</c></li>
     *   <li><c>"Linkin Park"</c></li>
     *   <li><c>"Slipknot"</c></li>
     * </ul>
     */
    public static final String MEDIA_ARTIST_NAME = "media.artist.name";

    /**
     * <p>The name/title of the song being queried.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"Smells Like Teen Spirit"</c></li>
     *   <li><c>"Come As You Are"</c></li>
     *   <li><c>"Lithium"</c></li>
     *   <li><c>"Something In The Way"</c></li>
     * </ul>
     */
    public static final String MEDIA_SONG_NAME = "media.song.name";

    /**
     * <p>The name/title of the song being queried.</p>
     * <h2>Notes</h2><p>This attribute is deprecated and will be removed in future versions. Use <c>media.song.name</c> for new implementations.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"Smells Like Teen Spirit"</c></li>
     *   <li><c>"Come As You Are"</c></li>
     *   <li><c>"Lithium"</c></li>
     *   <li><c>"Something In The Way"</c></li>
     * </ul>
     */
    @Deprecated
    public static final String MEDIA_SONG = "media.song";

    /**
     * <p>The duration of the song in milliseconds.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>301000</c></li>
     *   <li><c>355000</c></li>
     *   <li><c>183000</c></li>
     *   <li><c>391000</c></li>
     * </ul>
     */
    public static final String MEDIA_SONG_DURATION_MS = "media.song.duration_ms";

    /**
     * <p>The musical genre of the song.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"Rock"</c></li>
     *   <li><c>"Punk"</c></li>
     *   <li><c>"Grunge"</c></li>
     *   <li><c>"Classical"</c></li>
     *   <li><c>"Hip Hop"</c></li>
     * </ul>
     */
    public static final String MEDIA_SONG_GENRE = "media.song.genre";

    /**
     * <p>The release year of the song.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>1991</c></li>
     *   <li><c>1975</c></li>
     *   <li><c>1971</c></li>
     *   <li><c>1976</c></li>
     * </ul>
     */
    public static final String MEDIA_SONG_YEAR = "media.song.year";

    /**
     * <p>Unique identifier of the user.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"S-1-5-21-202424912787-2692429404-2351956786-1000"</c></li>
     * </ul>
     */
    public static final String USER_ID = "user.id";
}
