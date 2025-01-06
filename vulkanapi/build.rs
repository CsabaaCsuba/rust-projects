use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to rerun this script if the Vulkan or GLFW headers change
    println!("cargo:rerun-if-changed=C:\\VulkanSDK\\1.3.296.0\\Include");
    println!("cargo:rerun-if-changed=C:\\glfw-3.4-bin\\include");

    // Vulkan bindings
    let vulkan_bindings = bindgen::Builder::default()
        .header("C:\\VulkanSDK\\1.3.296.0\\Include\\vulkan\\vulkan.h")
        .generate()
        .expect("Unable to generate Vulkan bindings")
        .write_to_file(PathBuf::from("src/vulkan_bindings.rs"))
        .expect("Couldn't write Vulkan bindings");

    // GLFW bindings
    let glfw_bindings = bindgen::Builder::default()
        .header("C:\\glfw-3.4-bin\\include\\GLFW\\glfw3.h")
        .generate()
        .expect("Unable to generate GLFW bindings")
        .write_to_file(PathBuf::from("src/glfw_bindings.rs"))
        .expect("Couldn't write GLFW bindings");

    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-link-search=native=C:\\VulkanSDK\\1.3.296.0\\Lib");
    println!("cargo:rustc-link-search=native=C:\\glfw-3.4-bin\\lib-static-ucrt");

    println!("cargo:rustc-link-lib=dylib=vulkan-1");

    // Link GLFW (make sure GLFW is installed and in the library path)
    println!("cargo:rustc-link-lib=dylib=glfw3dll");
}
