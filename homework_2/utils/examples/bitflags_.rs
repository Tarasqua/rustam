fn main() {
    let green = Color::GREEN;
    let yellow = green | Color::BLUE;
    assert_eq!(yellow, Color::YELLOW);
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq)]
    struct Color: u8 {
        const BLACK = 0;
        const RED = 0b00000010;
        const GREEN = 0b00000001;
        const BLUE = 0b00000100;
        const YELLOW = Self::GREEN.bits() | Self::BLUE.bits();
    }
}
