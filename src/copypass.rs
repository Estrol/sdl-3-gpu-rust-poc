use std::ops::{Deref, DerefMut};

use sdl3_sys::gpu;

#[derive(Debug)]
pub struct GPUCopypass {
    pub copypass: *mut gpu::SDL_GPUCopyPass,
    pub used: bool,
}

#[allow(dead_code)]
impl GPUCopypass {
    pub fn end(&mut self) {
        if self.used {
            return;
        }

        unsafe {
            gpu::SDL_EndGPUCopyPass(self.copypass);
        }

        self.used = true;
    }
}

impl Deref for GPUCopypass {
    type Target = gpu::SDL_GPUCopyPass;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.copypass }
    }
}

impl DerefMut for GPUCopypass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.copypass }
    }
}

impl Drop for GPUCopypass {
    fn drop(&mut self) {
        if !self.used {
            self.end();
        }
    }
}
