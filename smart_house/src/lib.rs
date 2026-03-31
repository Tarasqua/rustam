mod area;
mod closures;
mod point;
mod traits;
mod turbofish;
use rand::RngExt;
use std::mem::size_of;

pub struct Thermometer {
    temperature: f32,
}

impl Thermometer {
    fn new() -> Self {
        Self {
            temperature: rand::rng().random_range(-20.0..=40.0),
        }
    }

    fn current_temperature(&self) -> f32 {
        self.temperature
    }
}

pub struct PowerSocket {
    is_active: bool,
}

impl PowerSocket {
    fn new() -> Self {
        Self { is_active: false }
    }

    pub fn switch_state(&mut self) {
        self.is_active = !self.is_active;
    }

    fn current_state(&self) -> bool {
        self.is_active
    }

    fn current_voltage(&self) -> f32 {
        if !self.is_active {
            0.0
        } else {
            rand::rng().random_range(1.0..=100.0)
        }
    }
}

pub enum Device {
    Thermometer(Thermometer),
    PowerSocket(PowerSocket),
}

pub struct SmartDevice {
    name: String,
    device: Device,
}

impl SmartDevice {
    pub fn new(name: String) -> Self {
        let device = if name.contains("power") {
            Device::PowerSocket(PowerSocket::new())
        } else {
            Device::Thermometer(Thermometer::new())
        };
        Self { name, device }
    }

    pub fn get_device_type(&mut self) -> &mut Device {
        &mut self.device
    }

    pub fn print_state(&self) {
        match &self.device {
            Device::PowerSocket(socket) => {
                println!(
                    "PowerSocket {} is {} and voltage is {}",
                    self.name,
                    if socket.current_state() {
                        "active"
                    } else {
                        "inactive"
                    },
                    socket.current_voltage()
                );
            }
            Device::Thermometer(thermometer) => {
                println!(
                    " Thermometer {}, temperature is {}",
                    self.name,
                    thermometer.current_temperature()
                );
            }
        }
    }
}

pub struct Room {
    devices: Vec<SmartDevice>,
}

impl Room {
    pub fn new(devices: Vec<SmartDevice>) -> Self {
        Self { devices }
    }

    pub fn get_device_ref(&self, index: usize) -> Option<&SmartDevice> {
        self.devices.get(index)
    }

    pub fn get_device_ref_mut(&mut self, index: usize) -> Option<&mut SmartDevice> {
        self.devices.get_mut(index)
    }

    pub fn make_report(&self) {
        for device in self.devices.iter() {
            device.print_state();
        }
    }
}

pub struct House {
    rooms: Vec<Room>,
}

impl House {
    pub fn new(rooms: Vec<Room>) -> Self {
        Self { rooms }
    }

    pub fn get_room_ref(&self, index: usize) -> Option<&Room> {
        self.rooms.get(index)
    }

    pub fn get_room_ref_mut(&mut self, index: usize) -> Option<&mut Room> {
        self.rooms.get_mut(index)
    }

    pub fn make_report(&self) {
        for (i, room) in self.rooms.iter().enumerate() {
            println!();
            println!("Room {}", i);
            room.make_report();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turbofish() {
        assert_eq!(4, size_of::<i32>());
    }
}
