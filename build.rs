use std::env;
use std::path::PathBuf;

fn main() {
    // 获取LAPACK路径
    let lapack_path =
        env::var("LAPACK_PATH").unwrap_or_else(|_| "/opt/homebrew/opt/lapack".to_string());
    let include_path = format!("{}/include", lapack_path);
    let lib_path = format!("{}/lib", lapack_path);

    // 告诉cargo链接LAPACK相关库
    println!("cargo:rustc-link-lib=lapacke");
    println!("cargo:rustc-link-lib=lapack");

    // 设置rpath，确保运行时能找到库文件
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path);

    // 添加include路径
    println!("cargo:include={}", include_path);

    // 设置重新构建的条件
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    // The bindgen::Builder is the main entry point to bindgen,
    // and lets you build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // 添加clang参数，指定include路径
        .clang_arg(format!("-I{}", include_path))
        // 保留LAPACK常量的白名单
        .allowlist_var("LAPACK_ROW_MAJOR")
        .allowlist_var("LAPACK_COL_MAJOR")
        // 允许所有以LAPACKE_开头的函数
        .allowlist_function("LAPACKE_.*")
        // // Tell cargo to invalidate the built crate whenever any of the included header files changed
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // 将绑定写入到src/bindings.rs文件中
    let out_path = PathBuf::from("src");
    let out_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
