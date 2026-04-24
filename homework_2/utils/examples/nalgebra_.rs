fn main() {
    let point = nalgebra::Vector2::new(1f32, 2f32);
    let rotation = nalgebra::Rotation2::new(std::f32::consts::FRAC_PI_2); // rotate 90 degrees
    let rotated = rotation * point;
    println!("Rotated coordinates: {}", rotated);

    let point = nalgebra::Point4::new(0f32, 0f32, 0f32, 1f32);
    let pos = nalgebra::Point3::new(0.5, 0.5, -1.0);
    let target = nalgebra::Point3::new(0.5, 0.5, 0.0);
    let up = nalgebra::Vector3::new(0f32, 1f32, 0f32);
    let transform: nalgebra::Matrix4<f32> = nalgebra::Matrix4::look_at_lh(&pos, &target, &up); // INFO: left-handed coordinate system
    let transformed_point = transform * point;
    println!("Transformed coordinates: {}", transformed_point);
}
