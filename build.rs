use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use ::regex::*;

pub const BUILD_DIR_DEFINE: &'static str = "DRIO_BUILD_DIR";

#[cfg(target_os = "windows")]
pub const PLATFORM: &'static str = "WINDOWS";

#[cfg(target_os = "linux")]
pub const PLATFORM: &'static str = "LINUX";

#[cfg(target_arch = "x86")]
pub const ARCHITECTURE: &'static str = "X86_32";

#[cfg(target_arch = "x86_64")]
pub const ARCHITECTURE: &'static str = "X86_64";

#[cfg(target_arch = "arm")]
pub const ARCHITECTURE: &'static str = "ARM_32";

#[cfg(target_arch = "aarch64")]
pub const ARCHITECTURE: &'static str = "ARM_64";

#[cfg(any(target_arch = "x86", target_arch = "arm"))]
pub const LIB_PATH: &'static str = "lib32/release";

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub const LIB_PATH: &'static str = "lib64/release";

fn version_ok(generated_rs: &Path) -> bool  {
    let mut fin = match File::open(generated_rs) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open generated bindings : {}", e);
            return false
        },
    };

    let mut contents = String::new();
    
    if let Err(e) = fin.read_to_string(&mut contents) {
        eprintln!("Failed to read generated bindings : {}", e);
        return false;
    }
    
    let re = Regex::new(r"_USES_DR_VERSION_.+=\s*(\d+);").unwrap();

    for cap in re.captures_iter(&contents) {
        let version: usize = match cap[1].parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse version '{}' : {}", &cap[1], e);
                return false;
            }
        };

        let major = version / 100;
        let minor = version % 100;

        if major.to_string() != env!("CARGO_PKG_VERSION_MAJOR") {
            panic!("Major version ({}) of {} does not match the current dynamorio-sys crate version ({})...", major, BUILD_DIR_DEFINE, env!("CARGO_PKG_VERSION_MAJOR"));
        }

        if minor.to_string() != env!("CARGO_PKG_VERSION_MINOR") {
            panic!("Minor version ({}) of {} does not match the current dynamorio-sys crate version ({})...", minor, BUILD_DIR_DEFINE, env!("CARGO_PKG_VERSION_MINOR"));
        }
    }
    
    return true;
}


fn main() {
    let build_dir = match env::var(BUILD_DIR_DEFINE) {
        Ok(build_dir) => PathBuf::from(build_dir),
        _ => panic!("Please set the {} to point to the base of the built DynamoRIO source", BUILD_DIR_DEFINE),
    };

    let mut extra_args: Vec<String> = vec![];

    extra_args.push(format!("-D{}", PLATFORM));
    extra_args.push(format!("-D{}", ARCHITECTURE));

    extra_args.push(format!("-I{}", build_dir.join("include").to_string_lossy()));
    extra_args.push(format!("-I{}", build_dir.join("ext/include").to_string_lossy()));

    // Core DynamoRIO lib
    println!("cargo:rustc-link-search={}", build_dir.join(LIB_PATH).to_string_lossy());

    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=static=dynamorio");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=static=dynamorio_static");

    // Extensions
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-cdylib-link-arg=/FORCE:MULTIPLE");

    // Include selected extensions
    if cfg!(feature = "bbdup") {
        extra_args.push("-D __FEATURE_BBDUP".to_string());
        println!("cargo:rustc-link-lib=static=drbbdup");
    }
    if cfg!(feature = "containers") {
        extra_args.push("-D __FEATURE_CONTAINERS".to_string());
        println!("cargo:rustc-link-lib=static=drcontainers");
    }
    if cfg!(feature = "covlib") {
        extra_args.push("-D __FEATURE_COVLIB".to_string());
        println!("cargo:rustc-link-lib=static=drcovlib");
    }
    if cfg!(feature = "mgr") {
        extra_args.push("-D __FEATURE_MGR".to_string());
        println!("cargo:rustc-link-lib=static=drmgr");
    }
    if cfg!(feature = "option") {
        extra_args.push("-D __FEATURE_OPTION".to_string());
    }
    if cfg!(feature = "reg") {
        extra_args.push("-D __FEATURE_REG".to_string());
        println!("cargo:rustc-link-lib=static=drreg");
    }
    if cfg!(feature = "syms") {
        extra_args.push("-D __FEATURE_SYMS".to_string());
        println!("cargo:rustc-link-lib=static=drsyms");
    }
    if cfg!(feature = "util") {
        extra_args.push("-D __FEATURE_UTIL".to_string());
        println!("cargo:rustc-link-lib=static=drutil");
    }
    if cfg!(feature = "wrap") {
        extra_args.push("-D __FEATURE_WRAP".to_string());
        println!("cargo:rustc-link-lib=static=drwrap");
    }
    if cfg!(feature = "x") {
        extra_args.push("-D __FEATURE_X".to_string());
        println!("cargo:rustc-link-lib=static=drx");
    }
    
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.h");

    println!("{:?}", extra_args);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        //.header_contents("_extra.h", extra_defines)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .size_t_is_usize(true)
        .allowlist_function("_?[dD][rR].*")
        .allowlist_type("_?[dD][rR].*")
        .allowlist_var("_?[dD][rR].*")
        .allowlist_var("_USES_DR_VERSION_")
        .clang_args(extra_args)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_path.join("bindings.rs");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(&out_file)
        .expect("Couldn't write bindings!");

    if !version_ok(&out_file) {
        std::fs::remove_file(&out_file).unwrap();
    }
}
