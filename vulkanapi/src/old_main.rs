#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod vulkan_bindings;
mod glfw_bindings;


use std::ffi::CStr;
use std::ptr;
use std::os::raw::c_char;

// For convenience, create type aliases
type GLFWwindow = *mut glfw_bindings::GLFWwindow;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const APP_NAME: &str = "Vulkan Window";

struct VulkanApp {
    instance: vulkan_bindings::VkInstance,
    window: GLFWwindow,
}

impl VulkanApp {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            // Initialize GLFW
            if glfw_bindings::glfwInit() == 0 {
                return Err("Failed to initialize GLFW".into());
            }

            // Tell GLFW not to create an OpenGL context
            glfw_bindings::glfwWindowHint(glfw_bindings::GLFW_CLIENT_API as i32, glfw_bindings::GLFW_NO_API as i32);
            glfw_bindings::glfwWindowHint(glfw_bindings::GLFW_RESIZABLE as i32, glfw_bindings::GLFW_FALSE as i32);

            // Create window
            let window = glfw_bindings::glfwCreateWindow(
                WIDTH as i32,
                HEIGHT as i32,
                "Vulkan Window\0".as_ptr() as *const i8,
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if window.is_null() {
                glfw_bindings::glfwTerminate();
                return Err("Failed to create GLFW window".into());
            }

            // Create Vulkan instance
            let app_info = vulkan_bindings::VkApplicationInfo {
                sType: vulkan_bindings::VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: ptr::null(),
                pApplicationName: APP_NAME.as_ptr() as *const c_char,
                applicationVersion: 1,
                pEngineName: APP_NAME.as_ptr() as *const c_char,
                engineVersion: 1,
                apiVersion: 1,
            };

            // Get required extensions from GLFW
            let mut glfw_extension_count = 0u32;
            let glfw_extensions = glfw_bindings::glfwGetRequiredInstanceExtensions(&mut glfw_extension_count);
            let mut extensions = Vec::new();
            for i in 0..glfw_extension_count {
                let ext = CStr::from_ptr(*glfw_extensions.offset(i as isize));
                extensions.push(ext.as_ptr());
            }

            let create_info = vulkan_bindings::VkInstanceCreateInfo {
                sType: vulkan_bindings::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                pApplicationInfo: &app_info,
                enabledLayerCount: 0,
                ppEnabledLayerNames: ptr::null(),
                enabledExtensionCount: extensions.len() as u32,
                ppEnabledExtensionNames: extensions.as_ptr(),
            };

            let mut instance: vulkan_bindings::VkInstance = ptr::null_mut();
            let result = vulkan_bindings::vkCreateInstance(&create_info, ptr::null(), &mut instance);

            if result != vulkan_bindings::VkResult_VK_SUCCESS {
                glfw_bindings::glfwDestroyWindow(window);
                glfw_bindings::glfwTerminate();
                return Err("Failed to create Vulkan instance".into());
            }

            Ok(VulkanApp { instance, window })
        }
    }

    fn main_loop(&self) {
        unsafe {
            while glfw_bindings::glfwWindowShouldClose(self.window) == 0 {
                glfw_bindings::glfwPollEvents();
            }
        }
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            vulkan_bindings::vkDestroyInstance(self.instance, ptr::null());
            glfw_bindings::glfwDestroyWindow(self.window);
            glfw_bindings::glfwTerminate();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = VulkanApp::new()?;
    app.main_loop();
    Ok(())
}