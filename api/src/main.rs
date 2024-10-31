pub mod bridge {
    tonic::include_proto!("bridge");
}

fn main() {
    let bridge = bridge::Bridge::default();
    println!("Hello, {bridge:?}");
}
