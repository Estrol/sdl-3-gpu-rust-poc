use std::sync::{Arc, Mutex};

use crate::{
    buffer::{
        gpu_buffer_type_to_sdl, gpu_transfer_usage_to_sdl, GPUBuffer, GPUBufferType,
        GPUTransferBuffer, GPUTransferUsage,
    },
    command_buffer::CommandBuffer,
    copypass::GPUCopypass,
    math::{Color, Rect},
    pipeline::{
        gpu_color_blend_factor_to_sdl, gpu_color_blend_op_to_sdl, gpu_vertex_element_to_sdl,
        GPUColorComponent, GPUGraphicsPipeline, GPUGraphicsPipelineCreateInfo,
    },
    renderpass::GPURenderpass,
    shader::{GPUShader, GPUShaderType},
    texture::{GPUTexture, GPUTextureAccess, GPUTextureCreateInfo, GPUTextureFormat},
    texture_utils::{
        gpu_texture_access_to_sdl, gpu_texture_format_to_sdl, sdl_to_gpu_texture_format,
    },
    window::Window,
};
use image::ImageReader;
use sdl3_sys::{
    error,
    gpu::{
        self, SDL_GPUBufferCreateInfo, SDL_GPUColorComponentFlags, SDL_GPUColorTargetInfo,
        SDL_GPUSamplerCreateInfo, SDL_GPUTextureCreateInfo, SDL_GPUTransferBufferCreateInfo,
    },
    video,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct GPUDevice {
    window: Arc<Window>,
    pub device: *mut gpu::SDL_GPUDevice,
}

#[allow(dead_code)]
impl GPUDevice {
    pub fn new(window: &Arc<Window>) -> Result<Arc<Self>, String> {
        let device = unsafe {
            gpu::SDL_CreateGPUDevice(gpu::SDL_GPU_SHADERFORMAT_SPIRV, true, std::ptr::null())
        };

        if device.is_null() {
            return Err("Failed to create GPU device: ".to_string() + &Self::get_sdl_error_msg());
        }

        let res = unsafe { gpu::SDL_ClaimWindowForGPUDevice(device, window.get_window()) };
        if !res {
            unsafe {
                gpu::SDL_DestroyGPUDevice(device);
            }

            return Err(
                "Failed to claim window for GPU device: ".to_string() + &Self::get_sdl_error_msg()
            );
        }

        Ok(Arc::new(GPUDevice {
            window: Arc::clone(window),
            device,
        }))
    }

    pub fn acquire_command_buffer(self: &Arc<Self>) -> Result<CommandBuffer, String> {
        let command_buffer = unsafe { gpu::SDL_AcquireGPUCommandBuffer(self.device) };
        if command_buffer.is_null() {
            return Err(
                "Failed to acquire command buffer: ".to_string() + &Self::get_sdl_error_msg()
            );
        }

        Ok(CommandBuffer {
            device: Arc::clone(self),
            command_buffer,
            usable: true,
            has_swapchain: false,
        })
    }

    pub fn acquire_swapchain(
        &self,
        command_buffer: &mut CommandBuffer,
    ) -> Result<Option<Arc<GPUTexture>>, String> {
        let mut ptr_to_texture: *mut gpu::SDL_GPUTexture = std::ptr::null_mut();
        let ptr_to_texture_ptr: *mut *mut gpu::SDL_GPUTexture = &mut ptr_to_texture;
        let mut zero: u32 = 0;
        let ptr_to_zero: *mut u32 = &mut zero;
        let res = unsafe {
            gpu::SDL_AcquireGPUSwapchainTexture(
                command_buffer.command_buffer,
                self.window.get_window(),
                ptr_to_texture_ptr,
                ptr_to_zero,
                ptr_to_zero,
            )
        };

        if !res {
            return Err(
                "Failed to acquire swapchain texture: ".to_string() + &Self::get_sdl_error_msg()
            );
        }

        if ptr_to_texture.is_null() {
            return Ok(None);
        }

        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe {
            video::SDL_GetWindowSize(self.window.get_window(), &mut width, &mut height);
        }

        let size = Rect::new(0, 0, width, height);

        command_buffer.has_swapchain = true;
        Ok(Some(Arc::new(GPUTexture {
            device: self.device,
            texture: ptr_to_texture,
            sampler: std::ptr::null_mut(),
            should_destroy: false,
            size,
            format: GPUTextureFormat::R8G8B8A8,
            access: GPUTextureAccess::RenderTarget,
        })))
    }

    pub fn get_swapchain_format(&self) -> GPUTextureFormat {
        let fmt =
            unsafe { gpu::SDL_GetGPUSwapchainTextureFormat(self.device, self.window.get_window()) };

        sdl_to_gpu_texture_format(fmt)
    }

    pub fn begin_renderpass(
        &self,
        command_buffer: &mut CommandBuffer,
        color: &Color,
        texture: &GPUTexture,
    ) -> Result<Arc<Mutex<GPURenderpass>>, String> {
        let mut color_target: SDL_GPUColorTargetInfo = unsafe { std::mem::zeroed() };
        color_target.texture = texture.texture;
        color_target.clear_color.r = color.r;
        color_target.clear_color.g = color.g;
        color_target.clear_color.b = color.b;
        color_target.clear_color.a = color.a;
        color_target.cycle = true;
        color_target.store_op = gpu::SDL_GPU_STOREOP_STORE;
        color_target.load_op = gpu::SDL_GPU_LOADOP_CLEAR;

        let res = unsafe {
            gpu::SDL_BeginGPURenderPass(
                command_buffer.command_buffer,
                &color_target,
                1,
                std::ptr::null(),
            )
        };

        if res.is_null() {
            return Err("Failed to begin renderpass".to_string());
        }

        Ok(Arc::new(Mutex::new(GPURenderpass {
            renderpass: res,
            used: false,
        })))
    }

    pub fn begin_copypass(
        &self,
        command_buffer: &mut CommandBuffer,
    ) -> Result<Arc<Mutex<GPUCopypass>>, String> {
        let copypass = unsafe { gpu::SDL_BeginGPUCopyPass(command_buffer.command_buffer) };

        if copypass.is_null() {
            return Err("Failed to begin copypass".to_string());
        }

        Ok(Arc::new(Mutex::new(GPUCopypass {
            copypass,
            used: false,
        })))
    }

    pub fn create_texture(
        self: &Arc<Self>,
        info: &GPUTextureCreateInfo,
    ) -> Result<Arc<Mutex<GPUTexture>>, String> {
        let mut create_info: SDL_GPUTextureCreateInfo = unsafe { std::mem::zeroed() };
        create_info.format = gpu_texture_format_to_sdl(info.format);
        create_info.usage = gpu_texture_access_to_sdl(info.access);
        create_info.width = info.width;
        create_info.height = info.height;
        create_info.layer_count_or_depth = 1;
        create_info.num_levels = 1;

        let texture = unsafe { gpu::SDL_CreateGPUTexture(self.device, &create_info) };
        if texture.is_null() {
            return Err("Failed to create texture".to_string());
        }

        let transfer_buffer = self.create_transfer_buffer(
            (info.width * info.height * 4) as usize,
            GPUTransferUsage::Upload,
        );

        if let Err(err) = transfer_buffer {
            unsafe {
                gpu::SDL_ReleaseGPUTexture(self.device, texture);
            }

            return Err(err);
        }

        let mut sampler_create_info: SDL_GPUSamplerCreateInfo = unsafe { std::mem::zeroed() };
        sampler_create_info.min_filter = gpu::SDL_GPU_FILTER_LINEAR;
        sampler_create_info.mag_filter = gpu::SDL_GPU_FILTER_LINEAR;
        sampler_create_info.mipmap_mode = gpu::SDL_GPU_SAMPLERMIPMAPMODE_LINEAR;
        sampler_create_info.address_mode_u = gpu::SDL_GPU_SAMPLERADDRESSMODE_REPEAT;
        sampler_create_info.address_mode_v = gpu::SDL_GPU_SAMPLERADDRESSMODE_REPEAT;
        sampler_create_info.address_mode_w = gpu::SDL_GPU_SAMPLERADDRESSMODE_REPEAT;
        sampler_create_info.mip_lod_bias = 0.0;
        sampler_create_info.max_anisotropy = 1.0;
        sampler_create_info.compare_op = gpu::SDL_GPU_COMPAREOP_NEVER;
        sampler_create_info.min_lod = 0.0;
        sampler_create_info.max_lod = 0.0;
        sampler_create_info.enable_anisotropy = false;
        sampler_create_info.enable_compare = false;

        let sampler = unsafe { gpu::SDL_CreateGPUSampler(self.device, &sampler_create_info) };
        if sampler.is_null() {
            unsafe {
                gpu::SDL_ReleaseGPUTexture(self.device, texture);
            }

            return Err("Failed to create sampler".to_string());
        }

        let res = Arc::new(Mutex::new(GPUTexture {
            device: self.device,
            texture,
            sampler,
            should_destroy: true,
            size: Rect::new(0, 0, info.width as i32, info.height as i32),
            format: info.format,
            access: info.access,
        }));

        let transfer_buffer = transfer_buffer.unwrap();
        let transfer_res = transfer_buffer.write_texture(self, &res, &info.data);
        if let Err(_err) = transfer_res {
            unsafe {
                gpu::SDL_ReleaseGPUTexture(self.device, texture);
            }

            return Err("Failed to write texture".to_string());
        }

        Ok(res)
    }

    pub fn create_texture_from_file(
        self: &Arc<Self>,
        path: &str,
    ) -> Result<Arc<Mutex<GPUTexture>>, String> {
        let img = ImageReader::open(path);
        if let Err(err) = img {
            return Err(format!("Failed to open image: {:?}", err));
        }

        let img = img.unwrap().decode();
        if let Err(err) = img {
            return Err(format!("Failed to decode image: {:?}", err));
        }

        let img = img.unwrap();
        let data = img.as_bytes();
        let width = img.width();
        let height = img.height();
        let format = img.color();

        let mut info = GPUTextureCreateInfo {
            data: data.to_vec(),
            width,
            height,
            format: GPUTextureFormat::R8G8B8A8,
            access: GPUTextureAccess::Sampler,
        };

        info.format = match format {
            image::ColorType::Rgb8 => GPUTextureFormat::R8G8B8A8,
            image::ColorType::Rgba8 => GPUTextureFormat::R8G8B8A8,
            _ => return Err("Unsupported texture format".to_string()),
        };

        let texture = self.create_texture(&info);
        if let Err(err) = texture {
            return Err(err);
        }

        let texture = texture.unwrap();

        Ok(texture)
    }

    pub fn create_buffer(
        self: &Arc<Self>,
        size: usize,
        usage: GPUBufferType,
    ) -> Result<Arc<Mutex<GPUBuffer>>, String> {
        let mut info: SDL_GPUBufferCreateInfo = unsafe { std::mem::zeroed() };
        info.size = size as u32;
        info.usage = gpu_buffer_type_to_sdl(usage);
        info.props = 0;

        let buffer = unsafe { gpu::SDL_CreateGPUBuffer(self.device, &info) };
        if buffer.is_null() {
            return Err("Failed to create buffer".to_string());
        }

        Ok(Arc::new(Mutex::new(GPUBuffer {
            device: self.device,
            buffer,
            size: size as u32,
        })))
    }

    pub fn create_transfer_buffer(
        &self,
        size: usize,
        usage: GPUTransferUsage,
    ) -> Result<Arc<GPUTransferBuffer>, String> {
        let mut info: SDL_GPUTransferBufferCreateInfo = unsafe { std::mem::zeroed() };
        info.size = size as u32;
        info.usage = gpu_transfer_usage_to_sdl(usage);
        info.props = 0;

        let buffer = unsafe { gpu::SDL_CreateGPUTransferBuffer(self.device, &info) };
        if buffer.is_null() {
            return Err("Failed to create transfer buffer".to_string());
        }

        Ok(Arc::new(GPUTransferBuffer {
            device: self.device,
            buffer,
            usage,
            size: size as u32,
        }))
    }

    pub fn create_shader(
        self: &Arc<Self>,
        shader_type: GPUShaderType,
        source: &[u8],
        num_samplers: u32,
        num_uniform_buffers: u32,
        num_storage_buffers: u32,
    ) -> Result<Arc<Mutex<GPUShader>>, String> {
        let mut shader_create_info: gpu::SDL_GPUShaderCreateInfo = unsafe { std::mem::zeroed() };
        shader_create_info.code = source.as_ptr() as *const u8;
        shader_create_info.code_size = source.len() as usize;
        shader_create_info.format = gpu::SDL_GPU_SHADERFORMAT_SPIRV;
        shader_create_info.stage = match shader_type {
            GPUShaderType::Vertex => gpu::SDL_GPU_SHADERSTAGE_VERTEX,
            GPUShaderType::Fragment => gpu::SDL_GPU_SHADERSTAGE_FRAGMENT,
        };
        shader_create_info.num_samplers = num_samplers;
        shader_create_info.num_uniform_buffers = num_uniform_buffers;
        shader_create_info.num_storage_buffers = num_storage_buffers;

        let cstring_entry_point = std::ffi::CString::new("main").unwrap();
        shader_create_info.entrypoint = cstring_entry_point.as_ptr();

        let shader = unsafe { gpu::SDL_CreateGPUShader(self.device, &shader_create_info) };
        if shader.is_null() {
            return Err("Failed to create shader".to_string());
        }

        Ok(Arc::new(Mutex::new(GPUShader {
            device: Arc::clone(self),
            shader,
            shader_type,
        })))
    }

    pub fn create_graphics_pipeline(
        self: &Arc<Self>,
        info: &GPUGraphicsPipelineCreateInfo,
    ) -> Result<Arc<Mutex<GPUGraphicsPipeline>>, String> {
        let mut pipeline_create_info: gpu::SDL_GPUGraphicsPipelineCreateInfo =
            unsafe { std::mem::zeroed() };

        let vertex_shader = info.vertex_shader.lock().unwrap();
        let fragment_shader = info.fragment_shader.lock().unwrap();

        pipeline_create_info.vertex_shader = vertex_shader.shader;
        pipeline_create_info.fragment_shader = fragment_shader.shader;

        let mut vertex_attributes: Vec<gpu::SDL_GPUVertexAttribute> = Vec::new();
        for attribute in &info.vertex_attributes {
            let vertex_attribute = gpu::SDL_GPUVertexAttribute {
                location: attribute.location,
                buffer_slot: attribute.buffer_slot,
                format: gpu_vertex_element_to_sdl(attribute.format),
                offset: attribute.offset,
            };

            vertex_attributes.push(vertex_attribute);
        }

        let mut vertex_buffer_description: gpu::SDL_GPUVertexBufferDescription =
            unsafe { std::mem::zeroed() };

        vertex_buffer_description.slot = info.vertex_buffer_desc.slot;
        vertex_buffer_description.pitch = info.vertex_buffer_desc.pitch;
        vertex_buffer_description.input_rate = match info.vertex_buffer_desc.input_rate {
            crate::pipeline::GPUVertexInputRate::Vertex => gpu::SDL_GPU_VERTEXINPUTRATE_VERTEX,
            crate::pipeline::GPUVertexInputRate::Instance => gpu::SDL_GPU_VERTEXINPUTRATE_INSTANCE,
        };

        let mut vertex_input_state: gpu::SDL_GPUVertexInputState = unsafe { std::mem::zeroed() };
        vertex_input_state.num_vertex_buffers = 1;
        vertex_input_state.vertex_buffer_descriptions = &mut vertex_buffer_description;
        vertex_input_state.num_vertex_attributes = vertex_attributes.len() as u32;
        vertex_input_state.vertex_attributes = vertex_attributes.as_ptr();

        pipeline_create_info.vertex_input_state = vertex_input_state;
        pipeline_create_info.primitive_type = match info.primitive_type {
            crate::pipeline::GPUPrimitiveType::PointList => gpu::SDL_GPU_PRIMITIVETYPE_POINTLIST,
            crate::pipeline::GPUPrimitiveType::LineList => gpu::SDL_GPU_PRIMITIVETYPE_LINELIST,
            crate::pipeline::GPUPrimitiveType::LineStrip => gpu::SDL_GPU_PRIMITIVETYPE_LINESTRIP,
            crate::pipeline::GPUPrimitiveType::TriangleList => {
                gpu::SDL_GPU_PRIMITIVETYPE_TRIANGLELIST
            }
            crate::pipeline::GPUPrimitiveType::TriangleStrip => {
                gpu::SDL_GPU_PRIMITIVETYPE_TRIANGLESTRIP
            }
        };

        let mut color_target_desc: gpu::SDL_GPUColorTargetDescription =
            unsafe { std::mem::zeroed() };
        color_target_desc.format = gpu_texture_format_to_sdl(info.texture_format);

        let mut color_blend_desc: gpu::SDL_GPUColorTargetBlendState = unsafe { std::mem::zeroed() };
        color_blend_desc.enable_blend = info.blend_state.enable_blend;
        color_blend_desc.enable_color_write_mask = info.blend_state.enable_color_write_mask;

        let mut color_write_mask: SDL_GPUColorComponentFlags = 0;
        if info.blend_state.color_write_mask & GPUColorComponent::R != GPUColorComponent::None {
            color_write_mask |= gpu::SDL_GPU_COLORCOMPONENT_R;
        }

        if info.blend_state.color_write_mask & GPUColorComponent::G != GPUColorComponent::None {
            color_write_mask |= gpu::SDL_GPU_COLORCOMPONENT_G;
        }

        if info.blend_state.color_write_mask & GPUColorComponent::B != GPUColorComponent::None {
            color_write_mask |= gpu::SDL_GPU_COLORCOMPONENT_B;
        }

        if info.blend_state.color_write_mask & GPUColorComponent::A != GPUColorComponent::None {
            color_write_mask |= gpu::SDL_GPU_COLORCOMPONENT_A;
        }

        color_blend_desc.color_write_mask = color_write_mask;
        color_blend_desc.color_blend_op =
            gpu_color_blend_op_to_sdl(info.blend_state.color_blend_op);
        color_blend_desc.src_color_blendfactor =
            gpu_color_blend_factor_to_sdl(info.blend_state.color_blend_factor_src);
        color_blend_desc.dst_color_blendfactor =
            gpu_color_blend_factor_to_sdl(info.blend_state.color_blend_factor_dst);
        color_blend_desc.alpha_blend_op =
            gpu_color_blend_op_to_sdl(info.blend_state.alpha_blend_op);
        color_blend_desc.src_alpha_blendfactor =
            gpu_color_blend_factor_to_sdl(info.blend_state.alpha_blend_factor_src);
        color_blend_desc.dst_alpha_blendfactor =
            gpu_color_blend_factor_to_sdl(info.blend_state.alpha_blend_factor_dst);

        color_target_desc.blend_state = color_blend_desc;

        let mut target_info: gpu::SDL_GPUGraphicsPipelineTargetInfo = unsafe { std::mem::zeroed() };
        target_info.num_color_targets = 1;
        target_info.color_target_descriptions = &mut color_target_desc;

        pipeline_create_info.target_info = target_info;

        let pipeline =
            unsafe { gpu::SDL_CreateGPUGraphicsPipeline(self.device, &pipeline_create_info) };

        if pipeline.is_null() {
            return Err("Failed to create graphics pipeline".to_string());
        }

        Ok(Arc::new(Mutex::new(GPUGraphicsPipeline {
            device: Arc::clone(self),
            pipeline,
        })))
    }

    fn get_sdl_error_msg() -> String {
        let cstr = unsafe { std::ffi::CStr::from_ptr(error::SDL_GetError()) };
        let str_slice = cstr.to_str().unwrap();

        str_slice.to_string()
    }
}

impl Drop for GPUDevice {
    fn drop(&mut self) {
        unsafe {
            gpu::SDL_DestroyGPUDevice(self.device);
        }
    }
}
