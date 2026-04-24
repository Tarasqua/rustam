use smallvec::{SmallVec, smallvec};

fn main() {
    let mut v: SmallVec<[u8; 4]> = smallvec![0, 1, 2];
    assert!(!v.spilled(), "spilled after initial push"); // INFO: check for data has spilled into a separate heap-allocated buffer
    v.push(3);
    assert!(v.spilled(), "spilled after pushing 3");
    v.push(4);
    assert!(v.spilled(), "spilled after pushing 4");
}
