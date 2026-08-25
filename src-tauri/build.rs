fn main() {
    tauri_build::build();
    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rustc-env=OUT_DIR={}",
        std::env::var("OUT_DIR").unwrap()
    );
}
