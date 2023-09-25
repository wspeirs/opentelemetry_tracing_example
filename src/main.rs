use opentelemetry::global;
use opentelemetry_stdout as stdout;
use opentelemetry::sdk::trace::TracerProvider;
use opentelemetry::trace::TracerProvider as _;
use tracing::{error, event, Level, span, Subscriber};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;


#[tracing::instrument]
fn another() {
    println!("HELLO");
}

fn setup_tracing() {
    // create a TracerProvider that uses STDOUT
    let provider = TracerProvider::builder()
        .with_simple_exporter(stdout::SpanExporter::default())
        .build();
    // create the tracer... what name should be used here?!?
    let tracer = provider.tracer("tracing_test");

    // create an OpenTelemetryLayer using this tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // add the OpenTelemetryLayer to a default registry
    let subscriber = Registry::default().with(telemetry);

    // set our subscriber as the default
    tracing::subscriber::set_global_default(subscriber).expect("Error setting global subscriber");
}

fn main() {
    setup_tracing();

    another();

    error!("This is an error!!!");

    {
        let span = span!(Level::TRACE, "my_span");
        let _guard = span.enter();

        event!(Level::INFO, foo = 5, bar = "hello");
    }

    global::shutdown_tracer_provider();
}
