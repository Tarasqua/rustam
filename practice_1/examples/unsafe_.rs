#![allow(unused)]

fn main() {
    let mut arr = vec![1, 2, 3, 4, 5];
    let (left, right) = split_at_mut(&mut arr, 3);

    assert_eq!(left, &[1, 2, 3]);
    assert_eq!(right, &[4, 5]);

    unsafe {
        let _second = arr.get_unchecked(1); // INFO: no bounds check
    }

    let _my_arr = MyType {
        ptr: arr.as_mut_ptr(),
        len: arr.len(),
        cap: arr.capacity(),
    };
}

fn split_at_mut<T>(slice: &mut [T], index: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    assert!(index <= len);
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, index),
            std::slice::from_raw_parts_mut(ptr.add(index), len - index),
        )
    }
}

struct MyType<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyType<T> {
    pub fn good(&mut self) {
        self.len += 2;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn feature_test() {}
}
