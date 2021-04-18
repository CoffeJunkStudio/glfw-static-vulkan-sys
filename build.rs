extern crate cmake;
use cmake::Config;

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    let vulkan_static = {
        if target_os == "macos" || target_os == "ios" {
            "ON"
        } else {
            "OFF"
        }
    };

    let dst = Config::new(".")
        .define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .define("GLFW_VULKAN_STATIC", vulkan_static)
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .build();
    
    let lib_dir = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("lib");
    // Ignore existing symlink error
    let _ = std::os::unix::fs::symlink(lib_dir.join("libglfw3.a"), lib_dir.join("libglfw.a"));
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
}
