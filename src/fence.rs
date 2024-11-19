use std::sync::Arc;

use sdl3_sys::gpu;

use crate::device::GPUDevice;

#[derive(Debug)]
pub struct EstFence {
    pub fence: *mut gpu::SDL_GPUFence,
    pub device: Arc<GPUDevice>,
}

#[allow(dead_code)]
impl EstFence {
    pub fn wait(&self) {
        unsafe { gpu::SDL_WaitForGPUFences(self.device.device, false, &self.fence, 0) };
    }
}

impl Drop for EstFence {
    fn drop(&mut self) {
        unsafe {
            gpu::SDL_ReleaseGPUFence(self.device.device, self.fence);
        }
    }
}
