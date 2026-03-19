fn irrefutable() {
    // INFO: Patterns that will match for any possible value passed are irrefutable
    let x = 5;
}

fn refutable() {
    // INFO: Patterns that can fail to match for some possible value are refutable.
    let some_option_value = Some(5);
    let Some(x) = some_option_value else {
        return;
    };
}
