use std::{error::Error, thread, time::Duration};

use opentelemetry;
use opentelemetry_zipkin;
use reqwest;
use tokio;
use tracing;
use tracing_attributes::instrument;
use tracing_opentelemetry;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[instrument]
#[inline]
fn expensive_work() -> &'static str {
    tracing::span!(tracing::Level::INFO, "expensive_step_1")
        .in_scope(|| thread::sleep(Duration::from_millis(25)));
    tracing::span!(tracing::Level::INFO, "expensive_step_2")
        .in_scope(|| thread::sleep(Duration::from_millis(25)));

    "success"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    // Install an otel pipeline with a simple span processor that exports data one at a time when
    // spans end. See the `install_batch` option on each exporter's pipeline builder to see how to
    // export in batches.
    let tracer = opentelemetry_zipkin::new_pipeline()
        .with_http_client(Some(Box::new(reqwest::Client::new())))
        .with_service_name("report_example")
        .install_batch(opentelemetry::runtime::Tokio)?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(opentelemetry)
        .try_init()?;

    let root = tracing::span!(tracing::Level::INFO, "app_start", work_units = 2);
    let _enter = root.enter();

    let work_result = expensive_work();

    tracing::span!(tracing::Level::INFO, "faster_work")
        .in_scope(|| thread::sleep(Duration::from_millis(10)));

    tracing::warn!("About to exit!");
    tracing::trace!("status: {}", work_result);

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
