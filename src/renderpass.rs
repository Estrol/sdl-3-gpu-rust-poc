use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use sdl3_sys::{gpu, rect};

use crate::{buffer::GPUBuffer, pipeline::GPUGraphicsPipeline, texture::GPUTexture};

pub struct GPURenderpass {
    pub renderpass: *mut gpu::SDL_GPURenderPass,
    pub used: bool,
}

impl Deref for GPURenderpass {
    type Target = gpu::SDL_GPURenderPass;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.renderpass }
    }
}

impl DerefMut for GPURenderpass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.renderpass }
    }
}

#[allow(dead_code)]
impl GPURenderpass {
    pub fn end(&mut self) -> Result<(), String> {
        if self.used {
            return Err("Renderpass is already ended!".to_string());
        }

        unsafe {
            gpu::SDL_EndGPURenderPass(self.renderpass);
        }

        self.used = true;

        Ok(())
    }

    pub fn draw_indexed(&self, index_count: u32, index_offset: u32, vertex_offset: i32) {
        unsafe {
            gpu::SDL_DrawGPUIndexedPrimitives(
                self.renderpass,
                index_count,
                1,
                index_offset,
                vertex_offset,
                0,
            );
        }
    }

    pub fn bind_vertex_buffer(&self, buffer: &Arc<Mutex<GPUBuffer>>, offset: u32) {
        let buffer = buffer.lock().unwrap();

        let binding = gpu::SDL_GPUBufferBinding {
            buffer: buffer.buffer,
            offset,
        };

        unsafe {
            gpu::SDL_BindGPUVertexBuffers(self.renderpass, 0, &binding, 1);
        }
    }

    pub fn bind_index_buffer(&self, buffer: &Arc<Mutex<GPUBuffer>>, offset: u32) {
        let buffer = buffer.lock().unwrap();

        let binding = gpu::SDL_GPUBufferBinding {
            buffer: buffer.buffer,
            offset,
        };

        unsafe {
            gpu::SDL_BindGPUIndexBuffer(
                self.renderpass,
                &binding,
                gpu::SDL_GPU_INDEXELEMENTSIZE_32BIT,
            );
        }
    }

    pub fn bind_graphics_pipeline(&self, pipeline: &GPUGraphicsPipeline) {
        unsafe {
            gpu::SDL_BindGPUGraphicsPipeline(self.renderpass, pipeline.pipeline);
        }
    }

    pub fn bind_fragment_samplers(&self, binding: &GPUTexture) {
        let binding = gpu::SDL_GPUTextureSamplerBinding {
            texture: binding.texture,
            sampler: binding.sampler,
        };

        unsafe {
            gpu::SDL_BindGPUFragmentSamplers(self.renderpass, 0, &binding, 1);
        }
    }

    pub fn bind_vertex_samplers(&self, binding: &GPUTexture) {
        let binding = gpu::SDL_GPUTextureSamplerBinding {
            texture: binding.texture,
            sampler: binding.sampler,
        };

        unsafe {
            gpu::SDL_BindGPUVertexSamplers(self.renderpass, 0, &binding, 1);
        }
    }

    pub fn set_viewport(&self, x: i32, y: i32, width: u32, height: u32, depth: f32) {
        let viewport = gpu::SDL_GPUViewport {
            x: x as f32,
            y: y as f32,
            w: width as f32,
            h: height as f32,
            min_depth: 0.0,
            max_depth: depth,
        };

        unsafe {
            gpu::SDL_SetGPUViewport(self.renderpass, &viewport);
        }
    }

    pub fn set_scissor(&self, x: i32, y: i32, width: u32, height: u32) {
        let scissor = rect::SDL_Rect {
            x,
            y,
            w: width as i32,
            h: height as i32,
        };

        unsafe {
            gpu::SDL_SetGPUScissor(self.renderpass, &scissor);
        }
    }
}

impl Drop for GPURenderpass {
    fn drop(&mut self) {
        if !self.used {
            let res = self.end();
            if let Err(err) = res {
                println!("Failed to end renderpass: {:?}", err);
            }
        }
    }
}
