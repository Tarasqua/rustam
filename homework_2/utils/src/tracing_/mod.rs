use tracing::instrument;

#[instrument] // -> TRACE foo{param=42}: utils::tracing_: tracing: 42
pub fn foo(param: i32) {
    tracing::trace!("tracing: {param}");
    tracing::debug!("debug");
    tracing::info!("info");
    tracing::warn!("warn");
    tracing::error!("error");
}

pub mod inner {
    use tracing::instrument;

    #[instrument(name = "inner::bar", skip(param))] // -> TRACE inner::bar: utils::tracing_::inner: tracing: 42
    pub fn bar(param: i32) {
        tracing::trace!("tracing: {param}");
        tracing::debug!("debug");
        tracing::info!("info");

        // INFO: all logs below will be logged with the "main_inner" span
        let main_inner_span = tracing::info_span!("main_inner");
        let _s = main_inner_span.enter();

        tracing::warn!("warn");
        tracing::error!("error");

        {
            let another_span = tracing::info_span!("another_span");
            let _s = another_span.enter();

            tracing::warn!("another warn") // -> WARN main_inner:another_span: another warn
        }
    }
}
