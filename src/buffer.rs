#![allow(dead_code)]

use sdl3_sys::gpu::{
    self, SDL_GPUBufferRegion, SDL_GPUTextureRegion, SDL_GPUTextureTransferInfo,
    SDL_GPUTransferBufferLocation, SDL_MapGPUTransferBuffer, SDL_UnmapGPUTransferBuffer,
};
use std::{
    cmp::PartialEq,
    sync::{Arc, Mutex},
};

use crate::{
    command_buffer::CommandBuffer, device::GPUDevice, texture::GPUTexture,
    texture_utils::gpu_texture_format_to_byte_size,
};

pub enum GPUBufferType {
    Vertex,
    Index,
    Uniform,
}

pub fn gpu_buffer_type_to_sdl(buffer_type: GPUBufferType) -> gpu::SDL_GPUBufferUsageFlags {
    match buffer_type {
        GPUBufferType::Vertex => gpu::SDL_GPU_BUFFERUSAGE_VERTEX,
        GPUBufferType::Index => gpu::SDL_GPU_BUFFERUSAGE_INDEX,
        GPUBufferType::Uniform => gpu::SDL_GPU_BUFFERUSAGE_GRAPHICS_STORAGE_READ,
    }
}

#[derive(Debug)]
pub struct GPUBuffer {
    pub buffer: *mut gpu::SDL_GPUBuffer,
    pub device: *mut gpu::SDL_GPUDevice,
    pub size: u32,
}

impl Drop for GPUBuffer {
    fn drop(&mut self) {
        unsafe {
            gpu::SDL_ReleaseGPUBuffer(self.device, self.buffer);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GPUTransferUsage {
    Upload,
    Download,
}
impl PartialEq for GPUTransferUsage {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (GPUTransferUsage::Upload, GPUTransferUsage::Upload)
                | (GPUTransferUsage::Download, GPUTransferUsage::Download)
        )
    }
}

pub fn gpu_transfer_usage_to_sdl(usage: GPUTransferUsage) -> gpu::SDL_GPUTransferBufferUsage {
    match usage {
        GPUTransferUsage::Upload => gpu::SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD,
        GPUTransferUsage::Download => gpu::SDL_GPU_TRANSFERBUFFERUSAGE_DOWNLOAD,
    }
}

#[derive(Debug)]
pub struct GPUTransferBuffer {
    pub buffer: *mut gpu::SDL_GPUTransferBuffer,
    pub device: *mut gpu::SDL_GPUDevice,
    pub usage: GPUTransferUsage,
    pub size: u32,
}

impl GPUTransferBuffer {
    pub fn read_texture(
        &self,
        device: &Arc<GPUDevice>,
        texture: &Arc<Mutex<GPUTexture>>,
    ) -> Result<Vec<u8>, String> {
        if self.usage != GPUTransferUsage::Download {
            return Err("Transfer buffer is not set to download".to_string());
        }

        let texture = texture.lock().unwrap();

        let byte_size = gpu_texture_format_to_byte_size(texture.format) as usize;
        let mut data = vec![0; texture.size.w as usize * texture.size.h as usize * byte_size];

        let mut region: SDL_GPUTextureRegion = unsafe { std::mem::zeroed() };
        region.texture = texture.texture;
        region.x = 0;
        region.y = 0;
        region.w = texture.size.w as u32;
        region.h = texture.size.h as u32;
        region.z = 0;
        region.d = 1;

        let mut location: SDL_GPUTextureTransferInfo = unsafe { std::mem::zeroed() };
        location.transfer_buffer = self.buffer;
        location.offset = 0;
        location.pixels_per_row = texture.size.w as u32;
        location.rows_per_layer = texture.size.h as u32;

        let mut command_buffer = device.acquire_command_buffer().unwrap();
        let copypass = device.begin_copypass(&mut command_buffer).unwrap();
        let mut copypass = copypass.lock().unwrap();

        unsafe { gpu::SDL_DownloadFromGPUTexture(copypass.copypass, &region, &location) };

        copypass.end();
        let res = command_buffer.submit();
        if let Err(e) = res {
            return Err(e);
        }

        let mapped_data = unsafe { SDL_MapGPUTransferBuffer(self.device, self.buffer, true) };

        if mapped_data.is_null() {
            return Err("Failed to map transfer buffer".to_string());
        }

        let data_ptr = mapped_data as *const u8;
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data.len()) };
        data.copy_from_slice(data_slice);

        unsafe {
            SDL_UnmapGPUTransferBuffer(self.device, self.buffer);
        };

        Ok(data)
    }

    pub fn write_texture(
        &self,
        device: &Arc<GPUDevice>,
        texture: &Arc<Mutex<GPUTexture>>,
        data: &[u8],
    ) -> Result<(), String> {
        if self.usage != GPUTransferUsage::Upload {
            return Err("Transfer buffer is not set to upload".to_string());
        }

        let texture = texture.lock().unwrap();

        let byte_size = gpu_texture_format_to_byte_size(texture.format) as usize;
        if data.len() != texture.size.w as usize * texture.size.h as usize * byte_size {
            return Err("Data size does not match texture size".to_string());
        }

        let mut region: SDL_GPUTextureRegion = unsafe { std::mem::zeroed() };
        region.texture = texture.texture;
        region.x = 0;
        region.y = 0;
        region.w = texture.size.w as u32;
        region.h = texture.size.h as u32;
        region.z = 0;
        region.d = 1;

        let mut location: SDL_GPUTextureTransferInfo = unsafe { std::mem::zeroed() };
        location.transfer_buffer = self.buffer;
        location.offset = 0;
        location.pixels_per_row = texture.size.w as u32;
        location.rows_per_layer = texture.size.h as u32;

        let mapped_data = unsafe { SDL_MapGPUTransferBuffer(self.device, self.buffer, false) };

        if mapped_data.is_null() {
            return Err("Failed to map transfer buffer".to_string());
        }

        let data_ptr = mapped_data as *mut u8;
        let data_slice = unsafe { std::slice::from_raw_parts_mut(data_ptr, data.len()) };
        data_slice.copy_from_slice(data);

        unsafe {
            SDL_UnmapGPUTransferBuffer(self.device, self.buffer);
        };

        let mut command_buffer = device.acquire_command_buffer().unwrap();
        let copypass = device.begin_copypass(&mut command_buffer).unwrap();
        let mut copypass = copypass.lock().unwrap();

        unsafe { gpu::SDL_UploadToGPUTexture(copypass.copypass, &location, &region, true) };

        copypass.end();
        let res = command_buffer.submit();
        if let Err(e) = res {
            return Err(e);
        }

        Ok(())
    }

    pub fn read_buffer(
        &self,
        device: &Arc<GPUDevice>,
        buffer: &Arc<Mutex<GPUBuffer>>,
    ) -> Result<Vec<u8>, String> {
        if self.usage != GPUTransferUsage::Download {
            return Err("Transfer buffer is not set to download".to_string());
        }

        let buffer = buffer.lock().unwrap();

        if buffer.size > self.size {
            return Err("Buffer size is larger than transfer buffer size".to_string());
        }

        let mut data = vec![0; buffer.size as usize];

        let mut location: SDL_GPUTransferBufferLocation = unsafe { std::mem::zeroed() };
        location.transfer_buffer = self.buffer;
        location.offset = 0;

        let mut region: SDL_GPUBufferRegion = unsafe { std::mem::zeroed() };
        region.buffer = buffer.buffer;
        region.offset = 0;
        region.size = buffer.size;

        let mut command_buffer = device.acquire_command_buffer().unwrap();
        let copypass = device.begin_copypass(&mut command_buffer).unwrap();
        let mut copypass = copypass.lock().unwrap();

        unsafe { gpu::SDL_DownloadFromGPUBuffer(copypass.copypass, &region, &location) };

        copypass.end();
        let res = command_buffer.submit();
        if let Err(e) = res {
            return Err(e);
        }

        let mapped_data = unsafe { SDL_MapGPUTransferBuffer(self.device, self.buffer, true) };

        if mapped_data.is_null() {
            return Err("Failed to map transfer buffer".to_string());
        }

        let data_ptr = mapped_data as *const u8;
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data.len()) };
        data.copy_from_slice(data_slice);

        unsafe {
            SDL_UnmapGPUTransferBuffer(self.device, self.buffer);
        };

        Ok(data)
    }

    /// Write buffer data to the transfer buffer and then upload it to the GPU buffer.
    /// This function version used a command buffer to submit the upload operation.
    /// This is useful when you want to batch multiple operations together.
    pub fn write_buffer_cmd(
        &self,
        device: &Arc<GPUDevice>,
        buffer: &Arc<Mutex<GPUBuffer>>,
        command_buffer: &mut CommandBuffer,
        data: &[u8],
    ) -> Result<(), String> {
        if self.usage != GPUTransferUsage::Upload {
            return Err("Transfer buffer is not set to upload".to_string());
        }

        let buffer = buffer.lock().unwrap();

        if data.len() > buffer.size as usize {
            return Err("Data size is larger than buffer size".to_string());
        }

        let mapped_data = unsafe { SDL_MapGPUTransferBuffer(self.device, self.buffer, false) };

        if mapped_data.is_null() {
            return Err("Failed to map transfer buffer".to_string());
        }

        let data_ptr = mapped_data as *mut u8;
        let data_slice = unsafe { std::slice::from_raw_parts_mut(data_ptr, data.len()) };
        data_slice.copy_from_slice(data);

        unsafe {
            SDL_UnmapGPUTransferBuffer(self.device, self.buffer);
        };

        let copypass = device.begin_copypass(command_buffer).unwrap();
        let mut copypass = copypass.lock().unwrap();

        let mut location: SDL_GPUTransferBufferLocation = unsafe { std::mem::zeroed() };
        location.transfer_buffer = self.buffer;
        location.offset = 0;

        let mut region: SDL_GPUBufferRegion = unsafe { std::mem::zeroed() };
        region.buffer = buffer.buffer;
        region.offset = 0;
        region.size = buffer.size;

        unsafe { gpu::SDL_UploadToGPUBuffer(copypass.copypass, &location, &region, true) };

        copypass.end();

        Ok(())
    }

    /// Write buffer data to the transfer buffer and then upload it to the GPU buffer.
    /// This function version used a command buffer to submit the upload operation.
    /// This is useful when you want to batch multiple operations together.
    ///
    /// This function is a generic version of write_buffer_cmd that allows you to write
    /// data of any type to the buffer.
    pub fn write_buffer_t_cmd<T>(
        &self,
        device: &Arc<GPUDevice>,
        buffer: &Arc<Mutex<GPUBuffer>>,
        command_buffer: &mut CommandBuffer,
        data: &[T],
    ) -> Result<(), String> {
        if self.usage != GPUTransferUsage::Upload {
            return Err("Transfer buffer is not set to upload".to_string());
        }

        let data_size = std::mem::size_of::<T>() * data.len();
        {
            let buffer = buffer.lock().unwrap();

            if data_size > buffer.size as usize {
                return Err("Data size is larger than buffer size".to_string());
            }
        }

        let new_data = unsafe {
            let data_ptr = data.as_ptr() as *const u8;
            std::slice::from_raw_parts(data_ptr, data_size)
        };

        let res = self.write_buffer_cmd(device, buffer, command_buffer, new_data);

        if let Err(e) = res {
            return Err(e);
        }

        Ok(())
    }

    /// Write buffer data to the transfer buffer and then upload it to the GPU buffer.
    /// This function version is a shortcut that does not require a command buffer.
    pub fn write_buffer(
        &self,
        device: &Arc<GPUDevice>,
        buffer: &Arc<Mutex<GPUBuffer>>,
        data: &[u8],
    ) -> Result<(), String> {
        if self.usage != GPUTransferUsage::Upload {
            return Err("Transfer buffer is not set to upload".to_string());
        }

        {
            let buffer = buffer.lock().unwrap();

            if data.len() > buffer.size as usize {
                return Err("Data size is larger than buffer size".to_string());
            }
        }

        let command_buffer = device.acquire_command_buffer();
        if let Err(_err) = command_buffer {
            return Err("Failed to acquire command buffer".to_string());
        }

        let mut command_buffer = command_buffer.unwrap();
        let res = self.write_buffer_cmd(device, buffer, &mut command_buffer, data);

        if let Err(e) = res {
            return Err(e);
        }

        Ok(())
    }

    /// Write buffer data to the transfer buffer and then upload it to the GPU buffer.
    /// This function version is a shortcut that does not require a command buffer.
    ///
    /// This function is a generic version of write_buffer that allows you to write
    /// data of any type to the buffer.
    pub fn write_buffer_t<T>(
        &self,
        device: &Arc<GPUDevice>,
        buffer: &Arc<Mutex<GPUBuffer>>,
        data: &[T],
    ) -> Result<(), String> {
        if self.usage != GPUTransferUsage::Upload {
            return Err("Transfer buffer is not set to upload".to_string());
        }

        {
            let buffer = buffer.lock().unwrap();
            let data_size = std::mem::size_of::<T>() * data.len();
            if data_size > buffer.size as usize {
                return Err("Data size is larger than buffer size".to_string());
            }
        }

        let command_buffer = device.acquire_command_buffer();
        if let Err(_err) = command_buffer {
            return Err("Failed to acquire command buffer".to_string());
        }

        let mut command_buffer = command_buffer.unwrap();
        let res = self.write_buffer_t_cmd(device, buffer, &mut command_buffer, data);

        if let Err(e) = res {
            return Err(e);
        }

        let res = command_buffer.submit();
        if let Err(e) = res {
            return Err(e);
        }

        Ok(())
    }
}

impl Drop for GPUTransferBuffer {
    fn drop(&mut self) {
        unsafe {
            gpu::SDL_ReleaseGPUTransferBuffer(self.device, self.buffer);
        }
    }
}
