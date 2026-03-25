#![cfg(feature = "telemetry")]

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const COLLECTOR_ENDPOINT: &str = "http://localhost:4318/v1/traces";

pub struct TelemetryGuard {
    provider: opentelemetry_sdk::trace::TracerProvider,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        // Flush all spans on shutdown
        self.provider.shutdown().ok();
    }
}

pub fn init() -> TelemetryGuard {
    let resource = Resource::new(vec![
        opentelemetry::KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
        opentelemetry::KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
    ]);

    // OTLP exporter over HTTP (works natively and in WASM via fetch)
    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(collector_endpoint())
        .build_span_exporter()
        .expect("Failed to build OTLP exporter");

    let provider = sdktrace::TracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter, runtime::Tokio)
        .build();

    let tracer = provider.tracer(env!("CARGO_PKG_NAME"));

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(otel_layer)
        .init();

    TelemetryGuard { provider }
}

/// Override endpoint per platform
fn collector_endpoint() -> &'static str {
    // On Android/desktop: localhost collector
    // In browser: a proxy you control, or a CORS-enabled collector
    option_env!("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or(COLLECTOR_ENDPOINT)
}
