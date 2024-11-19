#![allow(dead_code)]
use sdl3_sys::gpu;

use crate::math::Rect;

#[derive(Debug, Clone, Copy)]
pub enum GPUTextureAccess {
    Sampler,
    RenderTarget,
    DepthStencil,
    GraphicsStorage,
    ComputeStorageRead,
    ComputeStorageWrite,
    ComputeStorageReadWrite,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum GPUTextureFormat {
    R8,
    R8G8,
    R8G8B8A8,
    B8G8R8A8,
    A8_UNORM,
    R16_UNORM,
    R16G16_UNORM,
    R16G16B16A16_UNORM,
    R10G10B10A2_UNORM,
    B5G6R5_UNORM,
    B5G5R5A1_UNORM,
    B4G4R4A4_UNORM,
    BC1_RGBA_UNORM,
    BC2_RGBA_UNORM,
    BC3_RGBA_UNORM,
    BC4_R_UNORM,
    BC5_RG_UNORM,
    BC7_RGBA_UNORM,
    BC6H_RGB_FLOAT,
    BC6H_RGB_UFLOAT,
    R8_SNORM,
    R8G8_SNORM,
    R8G8B8A8_SNORM,
    R16_SNORM,
    R16G16_SNORM,
    R16G16B16A16_SNORM,
    R16_FLOAT,
    R16G16_FLOAT,
    R16G16B16A16_FLOAT,
    R32_FLOAT,
    R32G32_FLOAT,
    R32G32B32A32_FLOAT,
    R11G11B10_UFLOAT,
    R8_UINT,
    R8G8_UINT,
    R8G8B8A8_UINT,
    R16_UINT,
    R16G16_UINT,
    R16G16B16A16_UINT,
    R32_UINT,
    R32G32_UINT,
    R32G32B32A32_UINT,
    R8_INT,
    R8G8_INT,
    R8G8B8A8_INT,
    R16_INT,
    R16G16_INT,
    R16G16B16A16_INT,
    R32_INT,
    R32G32_INT,
    R32G32B32A32_INT,
    R8G8B8A8_UNORM_SRGB,
    B8G8R8A8_UNORM_SRGB,
    BC1_RGBA_UNORM_SRGB,
    BC2_RGBA_UNORM_SRGB,
    BC3_RGBA_UNORM_SRGB,
    BC7_RGBA_UNORM_SRGB,
    D16_UNORM,
    D24_UNORM,
    D32_FLOAT,
    D24_UNORM_S8_UINT,
    D32_FLOAT_S8_UINT,
    ASTC_4x4_UNORM,
    ASTC_5x4_UNORM,
    ASTC_5x5_UNORM,
    ASTC_6x5_UNORM,
    ASTC_6x6_UNORM,
    ASTC_8x5_UNORM,
    ASTC_8x6_UNORM,
    ASTC_8x8_UNORM,
    ASTC_10x5_UNORM,
    ASTC_10x6_UNORM,
    ASTC_10x8_UNORM,
    ASTC_10x10_UNORM,
    ASTC_12x10_UNORM,
    ASTC_12x12_UNORM,
    ASTC_4x4_UNORM_SRGB,
    ASTC_5x4_UNORM_SRGB,
    ASTC_5x5_UNORM_SRGB,
    ASTC_6x5_UNORM_SRGB,
    ASTC_6x6_UNORM_SRGB,
    ASTC_8x5_UNORM_SRGB,
    ASTC_8x6_UNORM_SRGB,
    ASTC_8x8_UNORM_SRGB,
    ASTC_10x5_UNORM_SRGB,
    ASTC_10x6_UNORM_SRGB,
    ASTC_10x8_UNORM_SRGB,
    ASTC_10x10_UNORM_SRGB,
    ASTC_12x10_UNORM_SRGB,
    ASTC_12x12_UNORM_SRGB,
    ASTC_4x4_FLOAT,
    ASTC_5x4_FLOAT,
    ASTC_5x5_FLOAT,
    ASTC_6x5_FLOAT,
    ASTC_6x6_FLOAT,
    ASTC_8x5_FLOAT,
    ASTC_8x6_FLOAT,
    ASTC_8x8_FLOAT,
    ASTC_10x5_FLOAT,
    ASTC_10x6_FLOAT,
    ASTC_10x8_FLOAT,
    ASTC_10x10_FLOAT,
    ASTC_12x10_FLOAT,
    ASTC_12x12_FLOAT,
}

#[derive(Debug)]
pub struct GPUTexture {
    pub device: *mut gpu::SDL_GPUDevice,
    pub texture: *mut gpu::SDL_GPUTexture,
    pub sampler: *mut gpu::SDL_GPUSampler,
    pub should_destroy: bool,
    pub size: Rect,
    pub format: GPUTextureFormat,
    pub access: GPUTextureAccess,
}

impl Drop for GPUTexture {
    fn drop(&mut self) {
        if self.should_destroy {
            unsafe {
                gpu::SDL_ReleaseGPUSampler(self.device, self.sampler);
                gpu::SDL_ReleaseGPUTexture(self.device, self.texture);
            }
        }
    }
}

pub struct GPUTextureCreateInfo {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: GPUTextureFormat,
    pub access: GPUTextureAccess,
}
