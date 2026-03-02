use std::{
    env,
    path::{Path, PathBuf},
};

fn resolve_core_c_root(manifest_dir: &Path) -> PathBuf {
    if let Some(path) = env::var_os("OPENDRONEID_CORE_C_DIR") {
        let root = PathBuf::from(path);
        if root.join("libopendroneid/opendroneid.h").exists() {
            return root;
        }
        panic!(
            "OPENDRONEID_CORE_C_DIR is set but does not contain libopendroneid/opendroneid.h: {}",
            root.display()
        );
    }

    let bundled = manifest_dir.join("vendor/opendroneid-core-c");
    if bundled.join("libopendroneid/opendroneid.h").exists() {
        return bundled;
    }

    panic!(
        "Bundled opendroneid-core-c sources not found under {}/vendor/opendroneid-core-c. Set OPENDRONEID_CORE_C_DIR to a local checkout.",
        manifest_dir.display()
    );
}

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set"));
    let core_c_root = resolve_core_c_root(&manifest_dir);
    let lib_dir = core_c_root.join("libopendroneid");

    let wifi_enabled = env::var_os("CARGO_FEATURE_WIFI").is_some();

    println!(
        "cargo:rerun-if-changed={}",
        lib_dir.join("opendroneid.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        lib_dir.join("opendroneid.c").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        lib_dir.join("odid_wifi.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        lib_dir.join("wifi.c").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("compat/byteswap.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        core_c_root.join("LICENSE").display()
    );
    println!("cargo:rerun-if-env-changed=OPENDRONEID_CORE_C_DIR");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_WIFI");

    let mut c_build = cc::Build::new();
    c_build
        .include(manifest_dir.join("compat"))
        .include(&lib_dir)
        .file(lib_dir.join("opendroneid.c"))
        .warnings(false);
    if wifi_enabled {
        c_build.file(lib_dir.join("wifi.c"));
    }
    c_build.compile("opendroneid");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "linux" {
        println!("cargo:rustc-link-lib=m");
    }

    let bindings = bindgen::Builder::default()
        .header_contents("opendroneid_wrapper.h", "#include <opendroneid.h>\n")
        .clang_arg(format!("-I{}", lib_dir.display()))
        .allowlist_type("ODID_.*")
        .allowlist_type("FRDID_.*")
        .allowlist_function("odid_.*")
        .allowlist_function("encode.*")
        .allowlist_function("decode.*")
        .allowlist_function("get.*")
        .allowlist_function("createEnum.*")
        .allowlist_function("drone_export_gps_data")
        .allowlist_function("frdid_.*")
        .allowlist_var("ODID_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true)
        .generate_comments(true)
        .layout_tests(false)
        .generate()
        .expect("Failed to generate libopendroneid bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write generated bindings");
}
