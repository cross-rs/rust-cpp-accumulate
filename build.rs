use std::env;

fn main() {
    let mut build = cc::Build::new();

    // msvc doesn't support C++11, only C++14 and higher, but
    // older targets like CentOS have only partial C++11 support.
    let abi = env::var("CARGO_CFG_TARGET_ENV");
    match abi.as_deref() {
        Ok("msvc") => build.flag("-std=c++14"),
        _ => build.flag("-std=c++11"),
    };

    build
        .cpp(true)
        .file("accumulate.cc")
        .compile("libaccumulate.a");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=accumulate.cc");
}
