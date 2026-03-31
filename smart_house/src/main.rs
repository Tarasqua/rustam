use num::Integer;
use smart_house::*;

// trait Trait: Sized {
//     fn is_reference(self) -> bool;
// }

// impl<'a, T> Trait for &'a T {
//     fn is_reference(self) -> bool {
//         true
//     }
// }

struct S;

trait Trait {
    fn f(&self);
}

impl Trait for u32 {
    fn f(&self) {
        print!("1");
    }
}

impl<'a> Trait for &'a i32 {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    // let room = make_room(5);
    // let mut house = House::new(vec![room]);

    // house.make_report();

    // let room = house.get_room_ref_mut(0).unwrap();
    // let device = room.get_device_ref_mut(0).unwrap();
    // let device_type = device.get_device_type();
    // match device_type {
    //     Device::PowerSocket(ps) => {
    //         ps.switch_state();
    //     }
    //     _ => {}
    // }

    // house.make_report();

    // match 0.is_reference() {
    //     true => print!("1"),
    //     false => print!("0"),
    // }

    // match '?'.is_reference() {
    //     true => print!("1"),
    //     false => {
    //         impl Trait for char {
    //             fn is_reference(self) -> bool {
    //                 false
    //             }
    //         }
    //         print!("0")
    //     }
    // }

    // let [x, y] = &mut [S, S];
    // let eq = x as *mut S == y as *mut S;
    // println!("{}", eq as u8);

    let x = &0;
    x.f();
}

fn make_room(num_devices: i32) -> Room {
    let mut devices: Vec<SmartDevice> = vec![];
    for i in 0..num_devices {
        let device_name = if i.is_even() {
            format!("power_socket_{}", i)
        } else {
            format!("thermometer_{}", i)
        };
        let device = SmartDevice::new(device_name);
        devices.push(device);
    }
    Room::new(devices)
}

trait Foo<T> {
    fn foo(&self) -> T;
}
