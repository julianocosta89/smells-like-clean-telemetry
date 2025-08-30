from flask import Flask
import logging
from pythonjsonlogger import jsonlogger
from opentelemetry import trace
from opentelemetry._logs import set_logger_provider, get_logger
from opentelemetry.exporter.otlp.proto.grpc._log_exporter import OTLPLogExporter
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk._logs import LoggerProvider, LoggingHandler
from opentelemetry.sdk._logs.export import BatchLogRecordProcessor
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import (
    BatchSpanProcessor,
)

loggerProvider = LoggerProvider()
processor = BatchLogRecordProcessor(OTLPLogExporter())
loggerProvider.add_log_record_processor(processor)
set_logger_provider(loggerProvider)

logger = get_logger(__name__)

handler = LoggingHandler(level=logging.INFO, logger_provider=loggerProvider)
json_formatter = jsonlogger.JsonFormatter()
handler.setFormatter(json_formatter)

logging.basicConfig(handlers=[handler], level=logging.INFO)

provider = TracerProvider()
processor = BatchSpanProcessor(OTLPSpanExporter())
provider.add_span_processor(processor)

app = Flask(__name__)
tracer = provider.get_tracer(__name__)

@app.route('/songs/<title>/<artist>')
@tracer.start_as_current_span("GET /songs/{title}/{artist}", kind=trace.SpanKind.SERVER)
def get_songs(title, artist):
    # This is an example of a non-compliant attribute
    # Created to showcase OTel Weaver's functionality
    span = trace.get_current_span()
    span.set_attribute("media.song", title)
    span.set_attribute("media.artist", artist)
    #logging.info("Get song for title: %s, artist: %s", title, artist)

    # Application logic to retrieve song metadata
    # [...]
    logging.info("Important audit log that must be created")

    #logging.info("Song retrieved for title: %s, artist: %s", title, artist)
    return {
        "title": title, 
        "artist": artist
    }

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)
