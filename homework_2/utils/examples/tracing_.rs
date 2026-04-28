use utils::tracing_;

// NOTE: RUST_LOG=trace,utils::tracing_::inner=trace cargo run --example tracing_
fn main() {
    tracing_subscriber::fmt::init();

    tracing_::foo(42);
    tracing_::inner::bar(42);
}
