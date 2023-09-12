extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let p4api_version = env::var("P4API_VERSION").unwrap();
    let openssl_version = env::var("SSL_VERSION").unwrap();

    // perforce api
    println!("cargo:rustc-link-search=./extern/p4api-{}/lib", p4api_version);
    println!("cargo:rustc-link-lib=static=client");
    println!("cargo:rustc-link-lib=static=rpc");
    println!("cargo:rustc-link-lib=static=supp");

    // p4api wrapper
    println!("cargo:rustc-link-search=./lib");
    println!("cargo:rustc-link-lib=static=p4api_wrapper");

    // std c++
    println!("cargo:rustc-link-lib=stdc++");

    // openssl
    println!("cargo:rustc-link-search=./extern/openssl-{}", openssl_version);
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=ssl");

    let bindings = bindgen::Builder::default()
        .header("p4api_wrapper/p4api_wrapper.h")
        // CLANG args
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .clang_arg(format!("-I./extern/p4api-{}/include", p4api_version))
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
