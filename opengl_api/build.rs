use bindgen;
// use std::env;
use std::path::PathBuf;

fn main(){
    // Tell Cargo to rerun this script if the Vulkan or GLFW headers change
    println!("cargo:rerun-if-changed=glad/include");
    println!("cargo:rerun-if-changed=glfw_3.4/include");

    

    // GLAD bindings
    bindgen::Builder::default()
        .header("glad/include/glad/glad.h")
        //.allowlist_recursively(false) //can help generate code only for used dependencies
        .layout_tests(false) //makes code faster
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(unused_imports)]")
        .generate()
        .expect("Unable to generate Vulkan bindings")
        .write_to_file(PathBuf::from("src/glad_bindings.rs"))
        .expect("Couldn't write Vulkan bindings");

    // GLFW bindings
    bindgen::Builder::default()
        .header("glfw_3.4/include/GLFW/glfw3.h")
        //.allowlist_recursively(false) //can help generate code only for used dependencies
        .layout_tests(false) //makes code faster
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(unused_imports)]")
        .generate()
        .expect("Unable to generate GLFW bindings")
        .write_to_file(PathBuf::from("src/glfw_bindings.rs"))
        .expect("Couldn't write GLFW bindings");

    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-link-search=native=glad/src");
    println!("cargo:rustc-link-search=native=glfw_3.4/lib_mingw64");

    // println!("cargo:rustc-link-lib=dylib=vulkan-1");

    // Link GLFW (make sure GLFW is installed and in the library path)
    println!("cargo:rustc-link-lib=dylib=glfw3");

    println!("cargo:rustc-link-lib=dylib=glad");

    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=shell32");
}