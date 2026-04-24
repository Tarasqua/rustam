fn main() {
    let points = vec![Point(1.0, 2.0); 10];
    write(bytemuck::cast_slice(&points)) // casting to u8 slice for zero-copy writing
}

#[repr(C)] // INFO: demanding C layout for zero-copy casting
#[derive(Copy, Clone)]
struct Point(f32, f32);

unsafe impl bytemuck::Zeroable for Point {} // INFO: asserting that Point can be zero-initialized (all bits zero is a valid Point)
unsafe impl bytemuck::Pod for Point {} // INFO: asserting that Point is Plain Old Data (POD) - it has no padding, no references, and can be safely cast to/from byte slices

fn write(_data: &[u8]) {}
