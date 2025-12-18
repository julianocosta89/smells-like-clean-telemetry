// DO NOT EDIT, this is an auto-generated file

package dev.jcosta.semconv;

public final class Attributes {
    /**
     * <p>The source code file name that identifies the code unit as uniquely as possible (preferably an absolute file path). This attribute MUST NOT be used on the Profile signal since the data is already captured in 'message Function'. This constraint is imposed to prevent redundancy and maintain data integrity.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"/usr/local/MyApplication/content_root/app/index.php"</c></li>
     * </ul>
     */
    public static final String CODE_FILE_PATH = "code.file.path";

    /**
     * <p>The method or function fully-qualified name without arguments. The value should fit the natural representation of the language runtime, which is also likely the same used within <c>code.stacktrace</c> attribute value. This attribute MUST NOT be used on the Profile signal since the data is already captured in 'message Function'. This constraint is imposed to prevent redundancy and maintain data integrity.</p>
     * <h2>Notes</h2><p>Values and format depends on each language runtime, thus it is impossible to provide an exhaustive list of examples.
     * The values are usually the same (or prefixes of) the ones found in native stack trace representation stored in
     * <c>code.stacktrace</c> without information on arguments.</p>
     * <p>Examples:</p>
     * <ul>
     *   <li>Java method: <c>com.example.MyHttpService.serveRequest</c></li>
     *   <li>Java anonymous class method: <c>com.mycompany.Main$1.myMethod</c></li>
     *   <li>Java lambda method: <c>com.mycompany.Main$$Lambda/0x0000748ae4149c00.myMethod</c></li>
     *   <li>PHP function: <c>GuzzleHttp\Client::transfer</c></li>
     *   <li>Go function: <c>github.com/my/repo/pkg.foo.func5</c></li>
     *   <li>Elixir: <c>OpenTelemetry.Ctx.new</c></li>
     *   <li>Erlang: <c>opentelemetry_ctx:new</c></li>
     *   <li>Rust: <c>playground::my_module::my_cool_func</c></li>
     *   <li>C function: <c>fopen</c></li>
     * </ul>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"com.example.MyHttpService.serveRequest"</c></li>
     *   <li><c>"GuzzleHttp\\Client::transfer"</c></li>
     *   <li><c>"fopen"</c></li>
     * </ul>
     */
    public static final String CODE_FUNCTION_NAME = "code.function.name";

    /**
     * <p>The line number in <c>code.file.path</c> best representing the operation. It SHOULD point within the code unit named in <c>code.function.name</c>. This attribute MUST NOT be used on the Profile signal since the data is already captured in 'message Line'. This constraint is imposed to prevent redundancy and maintain data integrity.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>42</c></li>
     * </ul>
     */
    public static final String CODE_LINE_NUMBER = "code.line.number";

    /**
     * <p>A stacktrace as a string in the natural representation for the language runtime. The representation is identical to <a href="/docs/exceptions/exceptions-spans.md#stacktrace-representation"><c>exception.stacktrace</c></a>. This attribute MUST NOT be used on the Profile signal since the data is already captured in 'message Location'. This constraint is imposed to prevent redundancy and maintain data integrity.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"at com.example.GenerateTrace.methodB(GenerateTrace.java:13)\\n at com.example.GenerateTrace.methodA(GenerateTrace.java:9)\\n at com.example.GenerateTrace.main(GenerateTrace.java:5)\n"</c></li>
     * </ul>
     */
    public static final String CODE_STACKTRACE = "code.stacktrace";

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

    /**
     * <p>The type of subscription the user has.</p>
     * <h2>Examples</h2>
     * <ul>
     *   <li><c>"Free"</c></li>
     *   <li><c>"Premium"</c></li>
     *   <li><c>"Family"</c></li>
     *   <li><c>"Student"</c></li>
     * </ul>
     */
    public static final String USER_SUBSCRIPTION_TYPE = "user.subscription.type";
}
