#[inline(never)] // INFO: prevent inlining
pub fn product(n: u64) -> u64 {
    let mut result = 1u64;
    for i in 2..=n {
        result = result.wrapping_mul(i) + 1;
    }
    result
}

#[inline(never)] // INFO: prevent inlining
pub fn product_long(n: u64) -> u64 {
    let mut result = 2u64;
    for i in 2..=n {
        result = result.wrapping_mul(i) + 1;
    }
    result
}
