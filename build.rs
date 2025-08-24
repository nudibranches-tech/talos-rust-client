fn main() {
    // The proto files are already compiled and checked into src/generated/
    // This is done to avoid requiring protoc at build time for users

    // If you need to regenerate the proto files:
    // 1. Uncomment the code below
    // 2. Run `cargo build`
    // 3. Move the generated files from target/debug/build/.../out/ to src/generated/
    // 4. Comment out the code again

    /*
    let proto_files = &[
        "vendor/talos/api/common/common.proto",
        "vendor/talos/api/machine/machine.proto",
        "vendor/talos/api/cluster/cluster.proto",
        "vendor/talos/api/inspect/inspect.proto",
        "vendor/talos/api/security/security.proto",
        "vendor/talos/api/storage/storage.proto",
        "vendor/talos/api/time/time.proto",
    ];

    let includes = &[
        "vendor/talos/api",
        "vendor/talos/api/vendor",
    ];

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_well_known_types(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile_protos(proto_files, includes)
        .expect("Failed to compile protos");
    */

    // Rerun if any proto files change
    println!("cargo:rerun-if-changed=vendor/talos/api");
}
