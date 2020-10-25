use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-search=native=libwfa");
    println!("cargo:rustc-link-lib=wfa");

    let bindings = bindgen::Builder::default()
        .clang_arg("-IWFA")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_var("stderr")
        .whitelist_var("stdout")
        .whitelist_function("mm_.*")
        .whitelist_type("mm_.*")
        .whitelist_function("affine_.*")
        .whitelist_type("affine_.*")
        .whitelist_function("edit_.*")
        .whitelist_type("edit_.*")
        .whitelist_function("backtrace_.*")
        .whitelist_type("backtrace_.*")
        .whitelist_function("alignment_.*")
        .whitelist_type("alignment_.*")
        .whitelist_function("wavefront_.*")
        .whitelist_type("wavefront_.*")
        .whitelist_function("swg_.*")
        .whitelist_var("METRIC_FACTOR_*")
        .whitelist_var("NUM_LINES_*")
        .whitelist_var("BUFFER_SIZE_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
