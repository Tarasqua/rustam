use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

fn main() {
    // INFO: both send and sync (inner types are send and sync)
    assert_send::<BothStruct>();
    assert_sync::<BothStruct>();

    // INFO: RefCell: Send+!Sync
    assert_send::<RefCell<()>>();
    assert_sync::<RefCell<()>>();

    // INFO: &RefCell: !Send+!Sync
    assert_send::<&RefCell<()>>();
    assert_sync::<&RefCell<()>>();

    // INFO: Rc: !Send+!Sync
    assert_send::<Rc<()>>();
    assert_sync::<Rc<()>>();

    // INFO: PhantomData: !Send+!Sync
    assert_send::<NeitherStruct>();
    assert_sync::<NeitherStruct>();
}

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[derive(Default)]
struct BothStruct {
    _number: u32,
    _array: [u8; 10],
    _string: String,
    _map: HashMap<u64, Vec<u8>>,
}

#[derive(Default)]
struct NeitherStruct {
    _p: PhantomData<Rc<()>>,
}
