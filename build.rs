extern crate cc;

use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .file("accumulate.cc")
        .compile("libaccumulate.a");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=accumulate.cc");
}
