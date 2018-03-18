#[cfg(feature = "bindgen")]
extern crate bindgen;

use std::io::prelude::*;
use std::env;

fn main() {
    let mut embree_dir;
    if let Ok(path) = env::var("EMBREE_LIBRARY") {
        embree_dir = path;
    }
    else {
        panic!("Couldn't find embree: environment variable EMBREE_LIBRARY isn't set");
    }

    // {
    //     //https://github.com/embree/embree/releases
    //     let _ = Command::new("curl").args(&["-O", "https://github.com/embree/embree/archive/v3.0.0.zip"]).status();
    //     let _ = Command::new("tar").args(&["-xf", "v3.0.0.zip", "-", "-C", "embree-3.0.0"]).status();
    //     embree_dir = cmake::Config::new("embree-3.0.0")
    //         .define("EMBREE_ISA_SSE2", "ON")
    //         .define("EMBREE_ISA_SSE42", "ON")
    //         .define("EMBREE_ISA_AVX", "ON")
    //         .define("EMBREE_ISA_AVX2", "ON")
    //         .define("EMBREE_ISA_AVX2", "ON")
    //         .define("EMBREE_ISPC_SUPPORT", "ON")
    //         .define("EMBREE_STATIC_LIB", "ON")
    //         .define("EMBREE_TASKING_SYSTEM", "INTERNAL")
    //         .build();
    // }

    if cfg!(feature = "bindgen") {
        let bindings_gen = bindgen::Builder::default()
            .header(embree_dir.join("include/embree3/rtcore.h"));
            // .clang_arg("-I\"C:\\Program Files (x86)\\Windows Kits\\10\\Include\\10.0.10240.0\\ucrt\\"");
        let bindings = bindings_gen.generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Writing bindings to file failed");
    }
    else {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        fs::copy(crate_path.join("pregenerated_bindings.rs"), out_path.join("bindings.rs"))
            .expect("Couldn't find pregenerated bindings!");
    }

    println!("cargo:rustc-link-lib=embree3");
    println!("cargo:rustc-link-search={}", embree_dir.join("lib"));

    println!("cargo:rustc-link-lib=tbb");
    println!("cargo:rustc-link-lib=tbbmalloc");
    println!("cargo:rustc-link-search=dylib={}", embree_dir.join("bin"));

    println!("cargo:rerun-if-changed=build.rs");
}