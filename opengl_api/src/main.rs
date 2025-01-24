#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[warn(dead_code)]

mod glad_bindings;
mod glfw_bindings;
use std::ffi::c_void;

use glfw_bindings::*;
use glad_bindings::*; //{glad_glBindBuffer, glad_glBufferData, glad_glCreateShader, glad_glGenBuffers, GL_ARRAY_BUFFER, GL_COLOR_BUFFER_BIT, GL_STATIC_DRAW};

fn Cstr(input: &str) -> *const i8 {

    return input.as_ptr() as *const i8;
}

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const TRUE: i32 = 1;
const FALSE: i32 = 0;


fn main() {
    unsafe {
        let _error_code = glfwInit();
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR.try_into().unwrap(), 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR.try_into().unwrap(), 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE.try_into().unwrap(), GLFW_OPENGL_CORE_PROFILE.try_into().unwrap());

        let window: *mut GLFWwindow = glfwCreateWindow(WIDTH, HEIGHT, Cstr("OpenGL window"), 0 as *mut GLFWmonitor, 0 as *mut GLFWwindow);
        
        if window.is_null() {
            println!("failed to create a glfw window");
            glfwTerminate();
            return;
        }
        glfwMakeContextCurrent(window);

        // glViewport(0, 0, WIDTH, HEIGHT);

        unsafe extern "C" fn framebuffer_size_callback(_window: *mut GLFWwindow, width: i32, height: i32)
        {
            glViewport(0, 0, width, height);
        }  

        glfwSetFramebufferSizeCallback(window, Some(framebuffer_size_callback));

        let vertices = [
            -0.5f32, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0,  0.5, 0.0
        ];

        // #[repr(c)]
        let mut VBO: u32 = 0;
        let glad_gl_gen_buffers = glad_glGenBuffers.unwrap()(1, &mut VBO as *mut u32);

        glad_glBindBuffer.unwrap()(GL_ARRAY_BUFFER, VBO);

        glad_glBufferData.unwrap()(GL_ARRAY_BUFFER, vertices.len() as i64, &vertices as *const f32 as *const c_void, GL_STATIC_DRAW);

        let vertexShaderSource  =
        "#version 330 core
        layout (location = 0) in vec3 aPos;

        void main()
        {
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }";

        let mut vertexShader: u32;
        vertexShader = glad_glCreateShader.unwrap()(glad_bindings::GL_VERTEX_SHADER);

        glad_glShaderSource.unwrap()(vertexShader, 1, &Cstr(vertexShaderSource) as *const i8, 0 as *mut i32);
        

        while glfwWindowShouldClose(window) == 0 {
            processInput(window);
            glClearColor(0.9, 0.3, 0.3, 1.0);
            glClear(glad_bindings::GL_COLOR_BUFFER_BIT);
            glfwSwapBuffers(window);
            glfwPollEvents();
        }

        unsafe fn processInput(window: *mut GLFWwindow){
            if glfwGetKey(window, GLFW_KEY_ESCAPE as i32) == GLFW_PRESS as i32 {
                glfwSetWindowShouldClose(window, TRUE);
            }
        }
        glfwTerminate();
    }
    
    
}
