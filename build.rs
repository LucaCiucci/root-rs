use std::path::PathBuf;


fn main() {
    let dst = build_and_link_bindings_project();
    generate_bindings(&dst);
}

fn build_and_link_bindings_project() -> PathBuf {
    let mut cfg = cmake::Config::new("root-rs-c-bindings");

    // Additional CMake configuration can be done here

    // build bindings project
    let dst = cfg.build();

    // Link dependencies are written to a file by the cmake script in this files
    let libs_file = dst.display().to_string() + "/lib/root-rs-c-bindings-linked-libs.txt";

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

    dst
}

fn generate_bindings(dst: &PathBuf) {
    let include_dir = dst.display().to_string() + "/include/root-rs-c-bindings";
    let header_path = include_dir.clone() + "/root-rs-c-bindings.h";

    let config_h = std::fs::read_to_string(include_dir.clone() + "/config.h").unwrap();
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
        }
    }

    println!("cargo:rerun-if-changed={}", header_path);

    let bindings = bindgen::Builder::default()
        .header(header_path)
        .clang_arg(format!("-I{}", include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}