#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[warn(dead_code)]

mod glad_bindings;
mod glfw_bindings;
// use glad_bindings::*;
use glfw_bindings::*;

fn Cstr(input: &str) -> *const i8 {

    return input.as_ptr() as *const i8;
}



fn main() {
    unsafe {
        let _error_code = glfwInit();
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR.try_into().unwrap(), 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR.try_into().unwrap(), 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE.try_into().unwrap(), GLFW_OPENGL_CORE_PROFILE.try_into().unwrap());

        let window: *mut GLFWwindow = glfwCreateWindow(800, 600, Cstr("OpenGL window"), 0 as *mut GLFWmonitor, 0 as *mut GLFWwindow);
        
        if window == 0 as *mut GLFWwindow {
            println!("failed to create a glfw window");
            glfwTerminate();
            return;
        }
        glfwMakeContextCurrent(window);


    }
    

    // println!("Hello, world!");
}
