use std::f32::consts::FRAC_PI_2;

fn main() {
    let point = glam::vec2(1f32, 1f32);
    let rotation = glam::Mat2::from_angle(FRAC_PI_2);
    let rotated = rotation * point;
    eprintln!("rotated = {:?}", rotated);
}
