use opentelemetry::global;

use log::Level;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_resource_detectors::{OsResourceDetector, ProcessResourceDetector};
use opentelemetry_sdk::{
    Resource, propagation::TraceContextPropagator, resource::ResourceDetector,
};
use std::env;
use std::str::FromStr;

fn get_resource() -> Resource {
    let detectors: Vec<Box<dyn ResourceDetector>> = vec![
        Box::new(OsResourceDetector),
        Box::new(ProcessResourceDetector),
    ];

    Resource::builder().with_detectors(&detectors).build()
}

fn init_tracer_provider() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_resource(get_resource())
        .with_batch_exporter(
            opentelemetry_otlp::SpanExporter::builder()
                .with_http()
                .build()
                .expect("Failed to initialize tracing provider"),
        )
        .build();

    global::set_tracer_provider(tracer_provider);
}

fn init_logger_provider() {
    let logger_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_resource(get_resource())
        .with_batch_exporter(
            opentelemetry_otlp::LogExporter::builder()
                .with_http()
                .build()
                .expect("Failed to initialize logger provider"),
        )
        .build();

    // Setup Log Appender for the log crate
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();

    // Read maximum log level from the enironment, using INFO if it's missing or
    // we can't parse it.
    let max_level = env::var("LOG_LEVEL")
        .ok()
        .and_then(|l| Level::from_str(l.to_lowercase().as_str()).ok())
        .unwrap_or(Level::Info);
    log::set_max_level(max_level.to_level_filter());
}

pub fn init_otel() -> Result<(), Box<dyn std::error::Error>> {
    init_logger_provider();
    init_tracer_provider();
    Ok(())
}
