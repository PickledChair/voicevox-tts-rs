use bindgen::{self, Builder as BindgenBuilder};
use cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = BindgenBuilder::default()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    let mut dst = cmake::build("openjtalk");

    dst.push("build");
    println!("cargo:rustc-link-search=native={}", dst.display());

    dst.push("open_jtalk");
    dst.push("src");
    println!("cargo:rustc-link-search=native={}", dst.display());

    // println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=static=openjtalkutil");
    println!("cargo:rustc-link-lib=static=openjtalk")
}
