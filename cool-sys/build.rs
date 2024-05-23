

// Example custom build script.
fn main() {
    use std::env;

    let wasi_sdk = env::var("WASI_SDK").unwrap();
    let wasi_sysroot = format!("{}/share/wasi-sysroot/", wasi_sdk);
    // Retrieve target at runtime, do not use cfg as the target is the host here.
    let target = env::var("TARGET").unwrap();

    // Use the `cc` crate to build a C file and statically link it.
    let mut builder = cc::Build::new();
    builder
        .compiler("clang++")
        .std("c++17")
        .cpp_link_stdlib("c++")
        .cpp(true)
        .include("native")
        .flag("-v")
        .file("./native/cool.cpp");

    if target == "wasm32-wasi" {
        // No support for exceptions in WASI
        builder.flag("-fno-exceptions");
        println!("cargo:rustc-link-search=native={}/lib/wasm32-wasi", wasi_sysroot);
        println!("cargo:rustc-flags=-L {}/lib/wasm32-wasi -lstatic=c++ -lstatic=c++abi", wasi_sysroot);
    }

    builder.compile("cool");
}