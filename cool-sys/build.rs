

// Example custom build script.
fn main() {
    // Use the `cc` crate to build a C file and statically link it.
    // It now support WASI c++ out of the box (since version 1.0.104)
    // It relies on WASI SDK & clang which is the only supported compiler as of today.
    let mut builder = cc::Build::new();
    builder
        .std("c++17")
        .cpp(true)
        .include("native")
        .flag("-v")
        .file("./native/cool.cpp");

        
    builder.compile("cool");
}