fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut package_metadata =
        rust_executable_metadata::PackageMetadataBuilder::new_from_cargo("svix")
            .unwrap()
            .license("MIT")
            .maintainer("support@svix.com")
            .copyright("Svix Inc");
    if let Ok(sha) = std::env::var("GITHUB_SHA") {
        package_metadata = package_metadata.hash(sha);
    }
    package_metadata
        .build()
        .unwrap()
        .write_linker_script_and_inject_argument()
        .unwrap();
}
