fn main() {
    let _ = cxx_build::bridge("src/lib.rs")
        .file("src/helpers.hpp")
        .opt_level(3);

    println!("cargo:rerun-if-changed=src/lib.rs");
}