fn main() {
    // Re-run if headers or build script change
    println!("cargo:rerun-if-changed=libomt.h");
    println!("cargo:rerun-if-env-changed=LIBOMT_LIB_DIR");
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");

    // Link search path for native libomt (user-provided)
    if let Ok(dir) = std::env::var("LIBOMT_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", dir);
    }

    // Link against libomt unless explicitly disabled (useful for cargo check)
    let no_link = std::env::var("LIBOMT_NO_LINK").ok().as_deref() == Some("1");
    if !no_link {
        println!("cargo:rustc-link-lib=dylib=libomt");
    }

    // Generate bindings
    let target = std::env::var("TARGET").unwrap();
    let mut builder = bindgen::Builder::default()
        .header("libomt.h")
        .allowlist_function("omt_.*")
        .allowlist_type("OMT.*|omt_.*")
        .allowlist_var("OMT.*")
        .generate_comments(true)
        .derive_default(true)
        .layout_tests(false)
        .clang_arg("-fparse-all-comments")
        .clang_arg(format!("--target={}", target));

    // Propagate any extra clang args provided by the user
    if let Ok(extra) = std::env::var("BINDGEN_EXTRA_CLANG_ARGS") {
        for arg in shell_words::split(&extra).unwrap_or_default() {
            builder = builder.clang_arg(arg);
        }
    }

    // On MSVC, add include paths from the INCLUDE environment variable so headers like stdint.h resolve
    if target.contains("windows-msvc") {
        if let Ok(include_paths) = std::env::var("INCLUDE") {
            for p in include_paths.split(';').filter(|s| !s.is_empty()) {
                builder = builder.clang_arg(format!("-I{}", p));
            }
        }
        // MSVC compatibility
        builder = builder.clang_arg("-fms-compatibility");
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let raw_path = out_path.join("bindings_raw.rs");
    bindings
        .write_to_file(&raw_path)
        .expect("Couldn't write raw bindings!");

    // For Rust 2024, extern blocks must be unsafe. Post-process to add `unsafe`.
    let raw = std::fs::read_to_string(&raw_path).expect("read raw bindings");
    let extern_fixed = raw.replace("extern \"C\" {", "unsafe extern \"C\" {");
    let fixed_path = out_path.join("bindings.rs");
    std::fs::write(&fixed_path, extern_fixed).expect("write fixed bindings");
}


