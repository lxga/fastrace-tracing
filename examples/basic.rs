use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;
use fastrace::prelude::SpanContext;
use fastrace_tracing::FastraceCompatLayer;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    fastrace::set_reporter(ConsoleReporter, Config::default());

    let subscriber = Registry::default().with(FastraceCompatLayer::new());
    tracing::subscriber::set_global_default(subscriber).unwrap();

    {
        let root = fastrace::Span::root("root", SpanContext::random());
        let _guard = root.set_local_parent();

        make_traces();
    }

    fastrace::flush();
}

fn make_traces() {
    let span = tracing::span!(tracing::Level::TRACE, "send request", work_units = 2);
    let _enter = span.enter();

    tracing::error!("This event will be logged in the root span.");
}
