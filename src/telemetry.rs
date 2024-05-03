use std::env;

use gethostname::gethostname;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use rustc_version::version;
use tonic::metadata::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry() {
    let mut map = MetadataMap::with_capacity(1);

    if env::var("OTEL_HONEYCOMB_KEY").is_ok() {
        map.insert(
            "x-honeycomb-team",
            env::var("OTEL_HEADER_VALUE")
                .expect("OTEL_HEADER_VALUE must be set if OTEL_HEADER_KEY is")
                .parse()
                .expect("OTEL_HEADER_VALUE to be parseable"),
        );
    }
    let tracer = tracing_opentelemetry::layer().with_tracer(
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(env::var("OTEL_ENDPOINT").unwrap_or_else(|_| {
                        return "http://localhost:4317".to_owned();
                    }))
                    .with_metadata(map),
            )
            .with_trace_config(sdktrace::config().with_resource(Resource::new(vec![
                            KeyValue::new("service.name", "wantjob"),
                            KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                            KeyValue::new("process.runtime.name", "rustc"),
                            KeyValue::new(
                                "process.runtime.version",
                                version().expect("rustc version to exist").to_string(),
                            ),
                            KeyValue::new(
                                "process.command",
                                std::env::args().next().expect("executable name to exist"),
                            ),
                            KeyValue::new(
                                "process.command_line",
                                std::env::args().collect::<Vec<_>>().join(" "),
                            ),
                            KeyValue::new(
                                "process.executable.name",
                                std::env::current_exe()
                                    .expect("current executable details to exist")
                                    .file_name()
                                    .expect("executable name to exist")
                                    .to_string_lossy()
                                    .into_owned(),
                            ),
                            KeyValue::new(
                                "process.executable.path",
                                std::env::current_exe()
                                    .expect("current executable details to exist")
                                    .display()
                                    .to_string(),
                            ),
                            KeyValue::new("process.pid", std::process::id() as i64),
                            KeyValue::new("host.arch", std::env::consts::ARCH),
                            KeyValue::new(
                                "host.name",
                                gethostname().into_string().expect("hostname to exist"),
                            ),
                        ])))
            .install_batch(runtime::Tokio)
            .expect("otel exporter to start"),
    );

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| return "wantjob=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracer)
        .try_init()
        .expect("Tracing to start");
}
