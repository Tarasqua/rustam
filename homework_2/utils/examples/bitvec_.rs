use bitvec::order::Msb0;

fn main() {
    let mut bits = bitvec::bitvec![u8, Msb0; 0, 0, 1, 0]; // INFO: creating a bit vector with 4 bits, using u8 as the underlying storage type and Msb0 (mos significant bit first) ordering
    println!("{bits:?}");
    bits.push(true);
    println!("{bits:?}");
    println!("{:?}", &bits[..3]);
}
