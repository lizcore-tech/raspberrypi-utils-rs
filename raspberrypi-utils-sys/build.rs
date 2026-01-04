use std::env;
use std::path::PathBuf;

fn main() {
    let piolib_dir = PathBuf::from("piolib-src");
    let include_dir = piolib_dir.join("include");

    println!("cargo:rerun-if-changed=wrapper.h");

    // Build the library using cmake
    let dst = cmake::Config::new(&piolib_dir)
        .define("BUILD_SHARED_LIBS", "OFF")
        // .cflag("-fkeep-inline-functions")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=pio");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let wrapper_c_path = out_path.join("bindgen_wrappers.c");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // https://rust-lang.github.io/rust-bindgen/faq.html#why-isnt-bindgen-generating-bindings-to-inline-functions
        // This tells bindgen to generate the C code for static inline functions
        .wrap_static_fns(true)
        // This tells bindgen WHERE to write that C code
        .wrap_static_fns_path(&wrapper_c_path)
        // // Keep inline functions
        // .clang_arg("-fkeep-inline-functions")
        // Add the main include directory
        .clang_arg(format!("-I{}", include_dir.display()))
        // .clang_arg("-fkeep-inline-functions")
        // .allowlist_recursively(true)
        // // Explicitly allow list the functions/types if they are still missing
        // .allowlist_function("pio_.*")
        // .allowlist_type("pio_.*")
        // .allowlist_var("pio.*")
        // .allowlist_function("PIO_.*")
        // .allowlist_type("PIO_.*")
        // .allowlist_var("PIO.*")
        // .enable_function_attribute_detection()
        // .merge_extern_blocks(true)
        // Handle C inline functions which are common in Pico SDK
        // .generate_inline_functions(true)
        // .generate_pure_virtual_functions(true)
        // .generate_block(true)
        // .generate_private_functions(true)
        // .generate_cstr(true)
        // .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Get the project root directory where wrapper.h is located
    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // NOW: Compile the generated C wrapper file so the __extern symbols exist
    cc::Build::new()
        .file(&wrapper_c_path)
        // Add the directory containing wrapper.h
        .include(&project_root)
        // Add the piolib include directories
        .include(&include_dir)
        .include(piolib_dir.join("include/piolib"))
        .compile("bindgen_wrappers");
}