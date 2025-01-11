#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[warn(dead_code)]

mod vulkan_bindings;
mod glfw_bindings;


use std::ffi::CStr;
use std::ptr;
use std::os::raw::c_char;

use vulkan_bindings::{vkGetInstanceProcAddr, PFN_vkCreateDebugUtilsMessengerEXT};

// For convenience, create type aliases
type GLFWwindow = *mut glfw_bindings::GLFWwindow;
type VkInstance = vulkan_bindings::VkInstance;
type VkResult = vulkan_bindings::VkResult;
type VkDebugUtilsMessengerEXT = vulkan_bindings::VkDebugUtilsMessengerEXT;
type VkPhysicalDevice = vulkan_bindings::VkPhysicalDevice;
type VkDevice = vulkan_bindings::VkDevice;
type VkQueue = vulkan_bindings::VkQueue;
type VkAllocationCallbacks = vulkan_bindings::VkAllocationCallbacks;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const APP_NAME: &str = "Vulkan Window";
const ValidationLayers: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];

fn CreateDebugUtilsMessengerExt(instance: VkInstance, 
    pCreateInfo: VkDebugUtilsMessengerEXT, 
    pAllocator: VkAllocationCallbacks,
    pDebugMessenger: VkDebugUtilsMessengerEXT
) -> VkResult {
    unsafe {
        let func = vkGetInstanceProcAddr(instance, "vkCreateDebugUtilsMessengerEXT".as_bytes()[0] as *const i8).unwrap() as *const unsafe extern "C" fn()->i32;
        *func(instance, pCreateInfo, pAllocator, pDebugMessenger)
    }
    
    

}

fn main(){


}