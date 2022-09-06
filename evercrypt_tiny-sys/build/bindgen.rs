//! Generates the bindings

use crate::files::{DIST_C89, DIST_KARAMEL_INCLUDE, DIST_KARAMEL_MINIMAL_INCLUDE};

/// Generates the bindings
pub fn generate() {
    // Generate bindings
    let bindings = bindgen::builder()
        // Allow EverCrypt symbols
        .allowlist_function("EverCrypt_.*")
        .allowlist_type("EverCrypt_.*")
        .allowlist_var("EverCrypt_.*")
        // Allow Spec symbols
        .allowlist_function("Spec_.*")
        .allowlist_type("Spec_.*")
        .allowlist_var("Spec_.*")
        // Allow Hacl symbols
        .allowlist_function("Hacl_.*")
        .allowlist_type("Hacl_.*")
        .allowlist_var("Hacl_.*")
        // Block functions that use non-FFI-safe u128 within their signature
        .blocklist_function("Hacl_Blake2b_32_blake2b_update_multi")
        .blocklist_function("Hacl_Blake2b_32_blake2b_update_last")
        .blocklist_function("Hacl_Hash_SHA2_update_last_384")
        .blocklist_function("Hacl_Hash_SHA2_update_last_512")
        .blocklist_function("Hacl_Blake2b_256_blake2b_update_multi")
        .blocklist_function("Hacl_Blake2b_256_blake2b_update_last")
        // Add include paths
        .clang_arg(format!("-I{DIST_C89}"))
        .clang_arg(format!("-I{DIST_KARAMEL_INCLUDE}"))
        .clang_arg(format!("-I{DIST_KARAMEL_MINIMAL_INCLUDE}"))
        // Generate bindings
        .header("src/bindgen/bindgen.h")
        .generate()
        .expect("Failed to create bindings");

    // Write bindings
    bindings.write_to_file("src/bindgen/bindgen.rs").expect("Failed to write bindings");
}
