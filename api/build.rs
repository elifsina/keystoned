fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        //.out_dir("./proto/keystoned/v1")
        .compile_protos(
            &[
                "./proto/keystoned/v1/container/service.proto",
                "./proto/keystoned/v1/bridge/service.proto",
            ],
            &["./proto/keystoned/v1/"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile proto file: {:?}", e));
    Ok(())
}
