use std::env;
use std::path::PathBuf;

fn main() {
    // 获取当前 build.rs 文件所在目录
    let build_dir = std::env::current_dir().unwrap();

    // 配置 include 目录
    let include_dir = build_dir.join(r"EasyX_26.1.1/include");
    let lib_dir = build_dir.join(r"EasyX_26.1.1/lib/VC2015");

    // 确定目标架构
    let target = env::var("TARGET").unwrap();
    let arch = if target.contains("x86_64") {
        "x64"
    } else {
        "x86"
    };

    // 编译 C++ 包装层
    cc::Build::new()
        .cpp(true) // 使用
        .define("UNICODE", None) // C++ 编译器
        .include(&include_dir)
        .file(build_dir.join("cpp/easyx_wrapper.cpp"))
        .compile("easyx_wrapper");

    // 设置库目录（只用于查找 EasyXw.lib）
    let lib_arch_dir = lib_dir.join(arch);
    println!("cargo:rustc-link-search={}", lib_arch_dir.display());

    // 链接 Windows 库
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=msimg32");
    println!("cargo:rustc-link-lib=shell32");

    // 生成绑定
    let bindings = bindgen::Builder::default()
        .header("cpp/easyx_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(false)
        .clang_arg("-fms-extensions")
        .generate()
        .expect("无法生成绑定");

    // 输出绑定文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("无法写入绑定文件");
}
