use std::env;
use std::path::PathBuf;

fn main() {
    // 获取当前目录的绝对路径
    let current_dir = env::current_dir().expect("无法获取当前目录");
    let lib_path = current_dir.join("lib");
    let lib_path_str = lib_path.to_str().expect("路径包含无效Unicode");

    // 告诉cargo在指定目录查找共享库
    println!("cargo:rustc-link-search={}", lib_path_str);

    // 设置rpath，确保运行时能找到库文件
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path_str);

    // 告诉cargo链接共享库
    println!("cargo:rustc-link-lib=demo");

    // The bindgen::Builder is the main entry point to bindgen,
    // and lets you build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("./include/demo.h")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./src/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
