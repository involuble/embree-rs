#[cfg(feature = "bindgen")]
extern crate bindgen;
extern crate pkg_config;

use std::path::PathBuf;
use std::env;
use std::fs;
// use std::error::Error;
use std::io;

#[cfg(feature = "bindgen")]
use bindgen;

#[cfg(feature = "bindgen")]
fn generate_bindings(include_dir: PathBuf) -> Result<(), io::Error> {
    let bindings_gen = bindgen::Builder::default()
        .header(include_dir.join("embree3/rtcore.h"))
        .clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.10240.0/ucrt"))
        .clang_arg(format!("-IC:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/include"));
        // .clang_arg(format!("-IC:/Program Files/LLVM/lib/clang/5.0.0/include"))
        // .clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/8.1/Include/shared"))
        // .clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/8.1/Include/um"));
    let bindings = bindings_gen.generate()?;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings(_: PathBuf) -> Result<(), io::Error> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(crate_path.join("pregenerated_bindings.rs"), out_path.join("bindings.rs"))?;
    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=EMBREE_DIR");

    if let Ok(path) = env::var("EMBREE_DIR") {
        let embree_dir = PathBuf::from(path);

        let include_dir = embree_dir.join("include");
        generate_bindings(include_dir).expect("Could not generate bindings");

        println!("cargo:rustc-link-lib=embree3");

        println!("cargo:rustc-link-search={}", embree_dir.join("lib").display());
        println!("cargo:rustc-link-search={}", embree_dir.join("bin").display());

        println!("cargo:rustc-link-lib=tbb");
        println!("cargo:rustc-link-lib=tbbmalloc");

        return;
    }

    let pkg = pkg_config::Config::new().atleast_version("3.0.0").probe("embree");
    if let Ok(lib) = pkg {
        let include_dir = PathBuf::from(lib.include_paths[0].clone());
        generate_bindings(include_dir).expect("Could not generate bindings");

        // Dunno if this is needed
        println!("cargo:rustc-link-lib=tbb");
        println!("cargo:rustc-link-lib=tbbmalloc");

        return;
    }

    panic!("Couldn't find embree: set environment variable EMBREE_DIR");

    // if !Path::new("embree/.git").exists() {
    //     Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();
    // }

    // //https://github.com/embree/embree/releases
    // let _ = Command::new("curl").args(&["-O", "https://github.com/embree/embree/archive/v3.0.0.zip"]).status();
    // let _ = Command::new("tar").args(&["-xf", "v3.0.0.zip", "-", "-C", "embree"]).status();

    // embree_dir = cmake::Config::new("embree")
    //     .define("EMBREE_ISA_SSE2", "ON")
    //     .define("EMBREE_ISA_SSE42", "ON")
    //     .define("EMBREE_ISA_AVX", "ON")
    //     .define("EMBREE_ISA_AVX2", "ON")
    //     .define("EMBREE_ISA_AVX2", "ON")
    //     .define("EMBREE_ISPC_SUPPORT", "ON")
    //     .define("EMBREE_STATIC_LIB", "ON")
    //     .define("EMBREE_TASKING_SYSTEM", "INTERNAL")
    //     .build();
}