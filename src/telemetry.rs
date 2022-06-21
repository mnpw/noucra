use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

// Compose multiple layers onto a `tracing`'s subscriber
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // TODO: Read about higher-ranked trait bounds
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Filter layer discards spans based on their log levels
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    // Formatting layer processes and passes context data in JSON format
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // Attach layers to Registry
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// Register the subscriber as global default
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Direct `log` events to tracing subscriber
    LogTracer::init().expect("Failed to set Logger");

    // Set subscriber for processing spans
    set_global_default(subscriber).expect("Failed to set subscriber");
}
