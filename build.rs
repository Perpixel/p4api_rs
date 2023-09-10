extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    println!("cargo:rustc-link-search=./extern/lib");
    println!("cargo:rustc-link-search=./lib/");

    // perforce api
    println!("cargo:rustc-link-lib=static=client");
    println!("cargo:rustc-link-lib=static=rpc");
    println!("cargo:rustc-link-lib=static=supp");
    println!("cargo:rustc-link-lib=static=p4api_wrapper");

    // std c++
    println!("cargo:rustc-link-lib=stdc++");

    // openssl
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=ssl");

    let bindings = bindgen::Builder::default()
        .header("p4api_wrapper/p4api_wrapper.h")
        // CLANG args
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .clang_arg("-I./extern/p4api-2023.1.2468153_linux_x64/include")
        .clang_arg("-I./p4api_wrapper")
        // Allow list
        .allowlist_type("ClientUser")
        .allowlist_type("ClientApi")
        .allowlist_type("ErrorPivate")
        .allowlist_type("ErrorSeverity")
        .allowlist_type("P4.*")
        //
        .opaque_type("std::.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate p4api binding");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
