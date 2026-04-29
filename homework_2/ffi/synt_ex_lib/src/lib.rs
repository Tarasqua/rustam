// INFO: real function name from static library (.a files, while .so files are dynamic libraries)
// cargo build
// nm  ./target/debug/libsynt_ex_lib.a | grep numbers
// 0000000000000000 T _ZN11synt_ex_lib11add_numbers17h5b15ea86b69cae18E
pub fn add_numbers(left: u64, right: u64) -> u64 {
    left + right
}

#[unsafe(no_mangle)] // prevents name mangling, so the function name is the same as the one in the static library
// nm  ./target/debug/libsynt_ex_lib.a | grep numbers
// 0000000000000000 T add_numbers_c
pub extern "C" fn add_numbers_c(left: u64, right: u64) -> u64 {
    left + right
}

#[repr(C)]
pub struct Params {
    left: u64,
    right: u64,
}

#[unsafe(no_mangle)]
pub extern "C" fn add_numbers_c_with_params(params: &Params) -> u64 {
    params.left + params.right
}
