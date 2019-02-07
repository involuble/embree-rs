extern crate pkg_config;
// #[cfg(all(feature = "vcpkg"), target_env = "msvc"))]
// extern crate vcpkg;

#[cfg(feature = "bindgen")]
extern crate bindgen;

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

fn try_load_from_directory(dir: PathBuf) -> Result<(), io::Error> {
    if !dir.is_dir() {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    let include_dir = dir.join("include");
    generate_bindings(include_dir)?;
    
    let bin_dir = dir.join("bin");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    for dll in ["embree3.dll", "tbb.dll", "tbbmalloc.dll"].iter() {
        fs::copy(bin_dir.join(dll), out_path.join(dll))?;
    }

    println!("cargo:rustc-link-search={}", dir.join("lib").display());

    println!("cargo:rustc-link-search=native={}", out_path.display());
    //println!("cargo:rustc-link-search=native={}", bin_dir.display());
    
    println!("cargo:rustc-link-lib=embree3");

    println!("cargo:rustc-link-lib=tbb");
    println!("cargo:rustc-link-lib=tbbmalloc");

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=EMBREE_DIR");

    if let Ok(path) = env::var("EMBREE_DIR") {
        let embree_dir = PathBuf::from(path);

        let r = try_load_from_directory(embree_dir);

        if r.is_ok() {
            return;
        }
    }

    // #[cfg(all(feature = "vcpkg"), target_env = "msvc"))]
    // let vc_pkg = vcpkg::Config::new()
    //     .emit_includes(true)
    //     .find_package("embree3");
    // if let Ok(lib) = vc_pkg {
    //     let include_dir = lib.include_paths[0].clone();
    //     generate_bindings(include_dir).expect("Could not generate bindings");
    //     return;
    // }

    let pkg = pkg_config::Config::new().atleast_version("3.0.0").probe("embree");
    if let Ok(lib) = pkg {
        let include_dir = lib.include_paths[0].clone();
        generate_bindings(include_dir).expect("Could not generate bindings");

        // Dunno if this is needed
        println!("cargo:rustc-link-lib=tbb");
        println!("cargo:rustc-link-lib=tbbmalloc");

        return;
    }

    // Default install location
    if let Ok(_) = try_load_from_directory(PathBuf::from("C:\\Program Files\\Intel\\Embree3 x64")) {
        return;
    }

    panic!("Couldn't find Embree: set environment variable EMBREE_DIR");

    // if !Path::new("embree/.git").exists() {
    //     Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();
    // }

    // let _ = Command::new("curl").args(&["-O", "https://github.com/embree/embree/archive/v3.0.0.zip"]).status();
    // let _ = Command::new("tar").args(&["-xf", "v3.0.0.zip", "-", "-C", "embree"]).status();

    // embree_dir = cmake::Config::new("embree")
    // //     .define("EMBREE_ISA_SSE2", "ON")
    // //     .define("EMBREE_ISA_SSE42", "ON")
    // //     .define("EMBREE_ISA_AVX", "ON")
    // //     .define("EMBREE_ISA_AVX2", "ON")
    //     .define("EMBREE_MAX_ISA", "AVX2")
    //     .define("EMBREE_ISPC_SUPPORT", "OFF")
    //     .define("EMBREE_STATIC_LIB", "ON")
    //     .define("EMBREE_TASKING_SYSTEM", "INTERNAL")
    //     .define("EMBREE_TUTORIALS", "OFF")
    //     .build();
}