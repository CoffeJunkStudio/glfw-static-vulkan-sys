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

    // Remove existing 'version-less' file, ignore if it didn't exist
    let _ = std::fs::remove_file(lib_dir.join("libglfw.a"));

    // Create a 'version-less' glfw file from our freshly build glfw3 library.
    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            std::os::unix::fs::symlink(lib_dir.join("libglfw3.a"), lib_dir.join("libglfw.a")).unwrap();
        } else if #[cfg(windows)] {
            std::os::windows::fs::symlink_file(lib_dir.join("libglfw3.a"), lib_dir.join("libglfw.a")).unwrap();
        } else {
            std::fs::copy(lib_dir.join("libglfw3.a"), lib_dir.join("libglfw.a")).unwrap();
        }
    }

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
}
