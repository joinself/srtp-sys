extern crate bindgen;

use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut clang_flags = Vec::<String>::new();

    let srtp_includes = Path::new("vendor/include/");
    let crypto_includes = Path::new("vendor/crypto/include/");

    let mut cmd = cc::Build::new();

    cmd.warnings(false)
        .include(srtp_includes)
        .include(crypto_includes)
        .include("./")
        .define("POSIX", "1")
        .define("HAVE_CONFIG_H", "1")
        // .define("ENABLE_DEBUG_LOGGING", "1")
        .define("ERR_REPORTING_STDOUT", "1")
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
        .file("vendor/srtp/srtp.c");

    // make these per target rather than match on arch, as we
    // may need to enable or disable flags on a per arch/os bases
    if target == "i686-linux-android" {
        cmd.define("CPU_CISC", "1");
        cmd.define("HAVE_X86", "1");
    } else if target == "x86_64-apple-darwin" {
        cmd.define("CPU_CISC", "1");
        cmd.define("HAVE_X86", "1");
    } else if target == "x86_64-apple-ios" {
        cmd.define("CPU_CISC", "1");
        cmd.define("HAVE_X86", "1");
    } else if target == "x86_64-linux-android" {
        cmd.define("CPU_CISC", "1");
        cmd.define("HAVE_X86", "1");
    } else if target == "x86_64-unknown-linux-gnu" {
        cmd.define("CPU_CISC", "1");
        cmd.define("HAVE_X86", "1");
    } else if target == "armv7-linux-androideabi" {
        cmd.define("CPU_RISC", "1");
    } else if target == "aarch64-apple-darwin" {
        cmd.define("CPU_RISC", "1");
    } else if target == "aarch64-apple-ios" {
        cmd.define("CPU_RISC", "1");
    } else if target == "aarch64-apple-ios-sim" {
        cmd.define("CPU_RISC", "1");
    } else if target == "aarch64-linux-android" {
        cmd.define("CPU_RISC", "1");
    } else if target == "aarch64-unknown-linux-gnu" {
        cmd.define("CPU_RISC", "1");
    } else if target == "wasm32-unknown-emscripten" {
        cmd.define("CPU_CISC", "1");
        clang_flags.push(String::from("-fvisibility=default"));
    }

    cmd.compile("srtp");

    // generate the bindings for srtp headers
    let mut builder = bindgen::Builder::default();

    for value in &clang_flags {
        builder = builder.clang_arg(value);
    }

    let bindings = builder
        .clang_arg("-Ivendor/")
        .clang_arg("-Ivendor/include/")
        .allowlist_type(r"srtp.*")
        .allowlist_type(r"SRTP.*")
        .allowlist_function(r"srtp.*")
        .allowlist_function(r"SRTP.*")
        .allowlist_var(r"srtp.*")
        .allowlist_var(r"SRTP.*")
        .header("vendor/include/srtp.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate srtp bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // output the bindings
    bindings
        .write_to_file(out_path.join("srtp.rs"))
        .expect("Couldn't write srtp bindings!");
}
