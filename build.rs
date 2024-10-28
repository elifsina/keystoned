fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile_protos(
            &["proto/keystoned/v1/voting.proto"],
            &["proto/keystoned/v1"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile proto file: {:?}", e));
    Ok(())
}
