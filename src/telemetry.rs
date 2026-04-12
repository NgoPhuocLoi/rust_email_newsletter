use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

pub fn get_subscriber(name: String, env_filter: EnvFilter) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| env_filter);
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        // Output the formatted spans to stdout.
        std::io::stdout,
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(s: impl Subscriber + Send + Sync) {
    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(s).expect("Failed to set subscriber");
}
