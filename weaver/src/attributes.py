# DO NOT EDIT, this is an auto-generated file

class Attributes:

    CODE_FILE_PATH = "code.file.path"
    """
    The source code file name that identifies the code unit as uniquely as possible (preferably an absolute file path). This attribute MUST NOT be used on the Profile signal since the data is already captured in 'message Function'. This constraint is imposed to prevent redundancy and maintain data integrity.

    ## Examples

    - `"/usr/local/MyApplication/content_root/app/index.php"`
    """

    CODE_FUNCTION_NAME = "code.function.name"
    """
    The method or function fully-qualified name without arguments. The value should fit the natural representation of the language runtime, which is also likely the same used within `code.stacktrace` attribute value. This attribute MUST NOT be used on the Profile signal since the data is already captured in 'message Function'. This constraint is imposed to prevent redundancy and maintain data integrity.
    ```
    Notes:
        Values and format depends on each language runtime, thus it is impossible to provide an exhaustive list of examples.
    ```

    The values are usually the same (or prefixes of) the ones found in native stack trace representation stored in
    `code.stacktrace` without information on arguments.

    Examples:

    - Java method: `com.example.MyHttpService.serveRequest`
    - Java anonymous class method: `com.mycompany.Main$1.myMethod`
    - Java lambda method: `com.mycompany.Main$$Lambda/0x0000748ae4149c00.myMethod`
    - PHP function: `GuzzleHttp\Client::transfer`
    - Go function: `github.com/my/repo/pkg.foo.func5`
    - Elixir: `OpenTelemetry.Ctx.new`
    - Erlang: `opentelemetry_ctx:new`
    - Rust: `playground::my_module::my_cool_func`
    - C function: `fopen`

    ## Examples

    - `"com.example.MyHttpService.serveRequest"`
    - `"GuzzleHttp\\Client::transfer"`
    - `"fopen"`
    """

    CODE_LINE_NUMBER = "code.line.number"
    """
    The line number in `code.file.path` best representing the operation. It SHOULD point within the code unit named in `code.function.name`. This attribute MUST NOT be used on the Profile signal since the data is already captured in 'message Line'. This constraint is imposed to prevent redundancy and maintain data integrity.

    ## Examples

    - `42`
    """

    CODE_STACKTRACE = "code.stacktrace"
    """
    A stacktrace as a string in the natural representation for the language runtime. The representation is identical to [`exception.stacktrace`](/docs/exceptions/exceptions-spans.md#stacktrace-representation). This attribute MUST NOT be used on the Profile signal since the data is already captured in 'message Location'. This constraint is imposed to prevent redundancy and maintain data integrity.

    ## Examples

    - `"at com.example.GenerateTrace.methodB(GenerateTrace.java:13)\\n at com.example.GenerateTrace.methodA(GenerateTrace.java:9)\\n at com.example.GenerateTrace.main(GenerateTrace.java:5)\n"`
    """

    MEDIA_ALBUM_NAME = "media.album.name"
    """
    The name of the album containing the song.

    ## Examples

    - `"Nevermind"`
    - `"In Utero"`
    - `"Bleach"`
    - `"Incesticide"`
    """

    MEDIA_ARTIST_NAME = "media.artist.name"
    """
    The name of the artist performing the song.

    ## Examples

    - `"Nirvana"`
    - `"Foo Fighters"`
    - `"Linkin Park"`
    - `"Slipknot"`
    """

    MEDIA_SONG = "media.song"
    """
    The name/title of the song being queried.
    ```
    Notes:
        This attribute is deprecated and will be removed in future versions. Use `media.song.name` for new implementations.
    ```

    ## Examples

    - `"Smells Like Teen Spirit"`
    - `"Come As You Are"`
    - `"Lithium"`
    - `"Something In The Way"`
    """

    MEDIA_SONG_DURATION_MS = "media.song.duration_ms"
    """
    The duration of the song in milliseconds.

    ## Examples

    - `301000`
    - `355000`
    - `183000`
    - `391000`
    """

    MEDIA_SONG_GENRE = "media.song.genre"
    """
    The musical genre of the song.

    ## Examples

    - `"Rock"`
    - `"Punk"`
    - `"Grunge"`
    - `"Classical"`
    - `"Hip Hop"`
    """

    MEDIA_SONG_NAME = "media.song.name"
    """
    The name/title of the song being queried.

    ## Examples

    - `"Smells Like Teen Spirit"`
    - `"Come As You Are"`
    - `"Lithium"`
    - `"Something In The Way"`
    """

    MEDIA_SONG_YEAR = "media.song.year"
    """
    The release year of the song.

    ## Examples

    - `1991`
    - `1975`
    - `1971`
    - `1976`
    """

    USER_ID = "user.id"
    """
    Unique identifier of the user.

    ## Examples

    - `"S-1-5-21-202424912787-2692429404-2351956786-1000"`
    """

    USER_SUBSCRIPTION_TYPE = "user.subscription.type"
    """
    The type of subscription the user has.

    ## Examples

    - `"Free"`
    - `"Premium"`
    - `"Family"`
    - `"Student"`
    """
