use utils::logging_;

// NOTE: RUST_LOG=info,utils::logging_::inner=trace cargo run --example logging_
fn main() {
    env_logger::init();
    logging_::foo();
    logging_::inner::bar();
}
