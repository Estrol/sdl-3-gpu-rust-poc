#![allow(dead_code)]

use std::sync::Arc;

use sdl3_sys::gpu;

use crate::device::GPUDevice;

#[derive(Debug, Clone, Copy)]
pub enum GPUShaderType {
    Vertex,
    Fragment,
}

impl PartialEq for GPUShaderType {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }
}

impl Eq for GPUShaderType {}

pub struct GPUShader {
    pub device: Arc<GPUDevice>,
    pub shader: *mut gpu::SDL_GPUShader,
    pub shader_type: GPUShaderType,
}

impl Drop for GPUShader {
    fn drop(&mut self) {
        unsafe {
            gpu::SDL_ReleaseGPUShader(self.device.device, self.shader);
        }
    }
}
