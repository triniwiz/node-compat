fn main() {
    let _ = cxx_build::bridge("src/lib.rs")
        .include("src/include/helpers.h")
        .opt_level(3);

    println!("cargo:rerun-if-changed=src/lib.rs");
}