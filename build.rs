use core::panic;
use std::path::PathBuf;


fn main() {
    // Builds the project in the directory located in `libfoo`, installing it
    // into $OUT_DIR
    //let mut cfg = cmake::Config::new("root-rs-c-bindings");
    ////panic!("cfg: {:?}", cfg.get_profile());
    //let cfg = cfg.very_verbose(true);
    //let dst = cfg.build();
    //let dst = cmake::build("root-rs-c-bindings");

    let mut cfg = cmake::Config::new("root-rs-c-bindings");

    // if "system-root" feature is enabled, set ROOT_RS_SYSTEM_ROOT to ON, otherwise OFF
    //panic!("system-root is enabled: {}", cfg!(feature = "system-root"));
    if cfg!(feature = "system-root") {
        //panic!("system-root feature is enabled");
        cfg.define("ROOT_RS_SYSTEM_ROOT", "ON");
    } else {
        cfg.define("ROOT_RS_SYSTEM_ROOT", "OFF");
    }

    let dst = cfg.build();

    let libs_file = dst.display().to_string() + "/lib/root-rs-c-bindings-linked-libs.txt";
    //panic!("libs_file: {}", libs_file);

    println!("cargo:rerun-if-changed={}", libs_file);

    let libs = std::fs::read_to_string(libs_file).unwrap();
    let libs = libs.split("\n");
    for lib in libs {
        let lib = lib.trim();
        if lib.len() == 0 {
            continue;
        }
        let lib = PathBuf::from(lib);
        let dir = lib.parent().unwrap();
        let lib = lib.file_stem().unwrap().to_str().unwrap();
        // remove the lib prefix if present
        #[cfg(unix)]
        let lib = lib.strip_prefix("lib").unwrap_or(lib);
        println!("cargo:rustc-link-search=native={}", dir.display());
        println!("cargo:rustc-link-lib={}", lib);
    }

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=root-rs-c-bindings");
    

    // Copy all shared libraries to OUT_DIR
    //let shared_libs_dir_file = dst.display().to_string() + "/lib/root_bin_dir.txt";
    //let shared_libs_dir = std::fs::read_to_string(shared_libs_dir_file).unwrap();
    //let shared_libs_dir = shared_libs_dir.trim();
    //if !shared_libs_dir.is_empty() {
    //    // find all shared libraries in shared_libs_dir
    //    let shared_libs_dir = PathBuf::from(shared_libs_dir);
    //    let shared_libs = std::fs::read_dir(&shared_libs_dir).unwrap();
//
    //    let extensions = [
    //        "so",
    //        "dll",
    //        "dylib",
    //    ];
//
    //    for shared_lib in shared_libs {
    //        //panic!("shared_lib: {:?}", shared_lib);
    //        let shared_lib = shared_lib.unwrap();
    //        let shared_lib = shared_lib.path();
    //        let shared_lib = shared_lib.file_name().unwrap();
    //        let shared_lib = shared_lib.to_str().unwrap();
//
    //        let mut found = false;
    //        for extension in &extensions {
    //            if shared_lib.ends_with(extension) {
    //                found = true;
    //                break;
    //            }
    //        }
    //        if !found {
    //            continue;
    //        }
//
    //        let shared_lib = shared_libs_dir.join(shared_lib);
    //        let shared_lib = shared_lib.display().to_string();
    //        //println!("cargo:rustc-link-search=native={}", shared_libs_dir.display());
    //        //println!("cargo:rustc-link-lib=dylib={}", shared_lib);
//
    //        // copy shared_lib to executable dir, which is <target_dir>\<profile>
    //        let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap();
    //        let profile = std::env::var("PROFILE").unwrap();
    //        let target_dir = PathBuf::from(target_dir).join(profile);
    //        let target_dir = target_dir.display().to_string();
    //        let target_dir = target_dir + "/deps";
    //    }
    //}

    let path = "root-rs-c-bindings/include/root-rs-c-bindings.h";
    let include_dir = dst.display().to_string() + "/include";

    let config_h = std::fs::read_to_string(include_dir.clone() + "/root-rs-c-bindings/config.h").unwrap();
    //panic!("config_h: {}", config_h);
    // REGEX to find all the defines that starts with RRS_ and match everything until the eol.
    let re = regex::Regex::new(r"#define\s+RRS_([^\s]+)\s+([^\s]+)").unwrap();
    for line in config_h.lines() {
        if let Some(caps) = re.captures(line) {
            let name = "RRS_".to_string() + caps.get(1).unwrap().as_str();
            let value = caps.get(2).unwrap().as_str().to_string();
            println!("cargo:rustc-env={}={}", name, value);
            if value.starts_with("\"") && value.ends_with("\"") {
                let value = value[1..value.len()-1].to_string();
                println!("cargo:rustc-env={}_STR={}", name, value);
            }
            // set an env var for each define
        }
    }
    //panic!("defines: {:?}", defines);

    println!("cargo:rerun-if-changed={}", path);

    let bindings = bindgen::Builder::default()
        .header(path)
        .clang_arg(format!("-I{}", include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}