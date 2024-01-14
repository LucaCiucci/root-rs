use std::path::PathBuf;


fn main() {
    // Builds the project in the directory located in `libfoo`, installing it
    // into $OUT_DIR
    let dst = cmake::build("root-rs-c-bindings");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=lib/root-rs-c-bindings");

    let libs_file = dst.display().to_string() + "/lib/libs.txt";
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
        let lib = lib.file_stem().unwrap();
        let lib = lib.to_str().unwrap();
        println!("cargo:rustc-link-search=native={}", dir.display());
        println!("cargo:rustc-link-lib=static={}", lib);
    }

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