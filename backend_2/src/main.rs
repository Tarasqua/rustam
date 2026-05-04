// mod client;
// mod data;
// mod server;

// Include the generated proto code
pub mod routeguide {
    tonic::include_proto!("routeguide");
}

use routeguide::Point;

fn main() {
    println!("Hello, world!");

    // Example usage of generated types
    let point = Point {
        latitude: 407838351,
        longitude: -746143763,
    };

    println!("Created point: {:?}", point);
}
