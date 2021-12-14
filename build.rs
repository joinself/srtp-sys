extern crate bindgen;

use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let srtp_includes = Path::new("vendor/include/");
    let crypto_includes = Path::new("vendor/crypto/include/");

    cc::Build::new()
        .warnings(false)
        .include(srtp_includes)
        .include(crypto_includes)
        .include("./")
        .define("POSIX", "1")
        .define("HAVE_CONFIG_H", "1")
        .file("vendor/crypto/cipher/aes.c")
        .file("vendor/crypto/cipher/aes_icm.c")
        .file("vendor/crypto/cipher/cipher.c")
        .file("vendor/crypto/cipher/null_cipher.c")
        .file("vendor/crypto/cipher/cipher_test_cases.c")
        .file("vendor/crypto/hash/auth.c")
        .file("vendor/crypto/hash/hmac.c")
        .file("vendor/crypto/hash/null_auth.c")
        .file("vendor/crypto/hash/sha1.c")
        .file("vendor/crypto/kernel/alloc.c")
        .file("vendor/crypto/hash/auth_test_cases.c")
        .file("vendor/crypto/kernel/crypto_kernel.c")
        .file("vendor/crypto/kernel/err.c")
        .file("vendor/crypto/kernel/key.c")
        .file("vendor/crypto/math/datatypes.c")
        .file("vendor/crypto/replay/rdb.c")
        .file("vendor/crypto/replay/rdbx.c")
        .file("vendor/srtp/srtp.c")
        .compile("srtp");


    println!("cargo:rerun-if-changed=srtp.h");

    // generate the bindings for srtp headers
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/")
        .clang_arg("-Ivendor/include/")
        .allowlist_type(r"srtp.*")
        .allowlist_type(r"SRTP.*")
        .allowlist_function(r"srtp.*")
        .allowlist_function(r"SRTP.*")
        .allowlist_var(r"srtp.*")
        .allowlist_var(r"SRTP.*")
        .header("vendor/include/srtp.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate srtp bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // output the bindings
    bindings
        .write_to_file(out_path.join("srtp.rs"))
        .expect("Couldn't write srtp bindings!");
}