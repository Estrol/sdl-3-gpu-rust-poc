use std::sync::Arc;

use sdl3_sys::gpu;

use crate::{device::GPUDevice, fence::EstFence};

#[derive(Debug)]
pub struct CommandBuffer {
    pub device: Arc<GPUDevice>,
    pub command_buffer: *mut gpu::SDL_GPUCommandBuffer,
    pub usable: bool,
    pub has_swapchain: bool,
}

#[allow(dead_code)]
impl CommandBuffer {
    pub fn submit(&mut self) -> Result<(), String> {
        if !self.usable {
            return Err("Command buffer is already used!".to_string());
        }

        unsafe {
            gpu::SDL_SubmitGPUCommandBuffer(self.command_buffer);
        }

        self.usable = false;

        Ok(())
    }

    pub fn submit_and_acquire_fence(&mut self) -> Result<Arc<EstFence>, String> {
        if !self.usable {
            return Err("Command buffer is already used!".to_string());
        }

        let fence = unsafe { gpu::SDL_SubmitGPUCommandBufferAndAcquireFence(self.command_buffer) };
        if fence.is_null() {
            return Err("Failed to submit GPU command buffer with fence".to_string());
        }

        self.usable = false;

        Ok(Arc::new(EstFence {
            fence,
            device: Arc::clone(&self.device),
        }))
    }

    pub fn cancel(&mut self) -> Result<(), String> {
        if !self.usable {
            return Err("Command buffer is already used!".to_string());
        }

        unsafe {
            gpu::SDL_CancelGPUCommandBuffer(self.command_buffer);
        }

        self.usable = false;

        Ok(())
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        if self.usable {
            if self.has_swapchain {
                let res = self.submit();
                if res.is_err() {
                    println!("Failed to submit command buffer: {:?}", res.unwrap_err());
                }
            } else {
                let res = self.cancel();
                if res.is_err() {
                    println!("Failed to cancel command buffer: {:?}", res.unwrap_err());
                }
            }
        }
    }
}
