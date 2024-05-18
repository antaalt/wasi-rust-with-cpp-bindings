
#[allow(dead_code)]
fn build_with_clang()
{
    use std::{env,path::Path, process::Command};

    //let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo::warning=OUT DIR: {}", out_dir);
    let clang_output = Command::new("clang++").args(&[
            "-c", // required for linking
            "-std=c++17",
            "--target=wasm32-wasi", 
            "-v",
            //"-fPIC",
            "-fno-exceptions", 
            "native/cool.cpp", 
            "-o"
        ])
        .arg(&format!("{}/lib_cool.o", out_dir))
        .output().expect("Failed to compile library");
    println!("cargo::warning=clang stdout: {}", String::from_utf8_lossy(&clang_output.stdout));
    println!("cargo::warning=clang stderr: {}", String::from_utf8_lossy(&clang_output.stderr));
    
    let llvmar_output = Command::new("llvm-ar").args(&[
            "r", 
            "libcool.a", 
            "lib_cool.o"
        ])
        .current_dir(&Path::new(&out_dir))
        .output().expect("Failed to link archive");
    println!("cargo::warning=llvm-ar stdout: {}", String::from_utf8_lossy(&llvmar_output.stdout));
    println!("cargo::warning=llvm-ar stderr: {}", String::from_utf8_lossy(&llvmar_output.stderr));
    
    println!("cargo::rustc-link-search=D:/Bibliotheque/Dev/.third-party/wasi-sdk-22.0.m-mingw64/share/wasi-sysroot/lib/wasm32-wasi");
    println!("cargo::rustc-link-search=native={}", out_dir);
    
    println!("cargo::rustc-link-lib=static=cool");
}
#[allow(dead_code)]
fn build_with_cc()
{
    use std::env;
    //let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    // Use the `cc` crate to build a C file and statically link it.
    let mut builder = cc::Build::new();
    builder.std("c++17")
        .cpp(true)
        .include("native")
        .flag("-v")
        .file("./native/cool.cpp");

    if target == "wasm32-wasi" {
        builder
            .cpp_link_stdlib("c++");
            //.flag("-fno-exceptions");
        println!("cargo:rustc-link-search=D:/Bibliotheque/Dev/.third-party/wasi-sdk-22.0.m-mingw64/share/wasi-sysroot/lib/wasm32-wasi");
    }

    builder.compile("cool");


    println!("cargo::rustc-link-lib=static=cool");
}

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    //println!("cargo::rerun-if-changed=native/cool.cpp");
    //println!("cargo::rerun-if-changed=native/cool.h");
    
    /*let sysroot = var("MYLIB_WASI_SYSROOT")
        .or(var("WASI_SYSROOT"))
        .ok()
        .or_else(|| Some(format!("{}/share/wasi-sysroot", var("WASI_SDK_PATH").ok()?)));
    let sysroot = match sysroot {
        Some(sysroot) => format!("--sysroot={}", sysroot),
        None => {
            eprintln!(
                "Install wasi-sdk or wasi-libc and specify WASI_SYSROOT path in environment!"
            );
            exit(1);
        }
    };*/
    //build_with_cc();
    build_with_clang();
}