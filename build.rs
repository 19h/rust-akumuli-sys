use bindgen::builder;

use std::env;
use std::path::PathBuf;
use std::collections::HashSet;

fn main() {
    println!("cargo:rustc-link-lib=apr");

    let bindings =
        builder()
            .header("./wrapper.h")
            .generate()
            .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
