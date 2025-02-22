use sdl3_sys::gpu;

use crate::texture::{GPUTextureAccess, GPUTextureFormat};

pub fn gpu_texture_format_to_sdl(format: GPUTextureFormat) -> gpu::SDL_GPUTextureFormat {
    match format {
        GPUTextureFormat::R8 => gpu::SDL_GPU_TEXTUREFORMAT_R8_UNORM,
        GPUTextureFormat::R8G8 => gpu::SDL_GPU_TEXTUREFORMAT_R8G8_UNORM,
        GPUTextureFormat::R8G8B8A8 => gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM,
        GPUTextureFormat::B8G8R8A8 => gpu::SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM,
        GPUTextureFormat::A8_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_A8_UNORM,
        GPUTextureFormat::R16_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_R16_UNORM,
        GPUTextureFormat::R16G16_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_R16G16_UNORM,
        GPUTextureFormat::R16G16B16A16_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UNORM,
        GPUTextureFormat::R10G10B10A2_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_R10G10B10A2_UNORM,
        GPUTextureFormat::B5G6R5_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_B5G6R5_UNORM,
        GPUTextureFormat::B5G5R5A1_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_B5G5R5A1_UNORM,
        GPUTextureFormat::B4G4R4A4_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_B4G4R4A4_UNORM,
        GPUTextureFormat::BC1_RGBA_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM,
        GPUTextureFormat::BC2_RGBA_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM,
        GPUTextureFormat::BC3_RGBA_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM,
        GPUTextureFormat::BC4_R_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_BC4_R_UNORM,
        GPUTextureFormat::BC5_RG_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_BC5_RG_UNORM,
        GPUTextureFormat::BC7_RGBA_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM,
        GPUTextureFormat::BC6H_RGB_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_BC6H_RGB_FLOAT,
        GPUTextureFormat::BC6H_RGB_UFLOAT => gpu::SDL_GPU_TEXTUREFORMAT_BC6H_RGB_UFLOAT,
        GPUTextureFormat::R8_SNORM => gpu::SDL_GPU_TEXTUREFORMAT_R8_SNORM,
        GPUTextureFormat::R8G8_SNORM => gpu::SDL_GPU_TEXTUREFORMAT_R8G8_SNORM,
        GPUTextureFormat::R8G8B8A8_SNORM => gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_SNORM,
        GPUTextureFormat::R16_SNORM => gpu::SDL_GPU_TEXTUREFORMAT_R16_SNORM,
        GPUTextureFormat::R16G16_SNORM => gpu::SDL_GPU_TEXTUREFORMAT_R16G16_SNORM,
        GPUTextureFormat::R16G16B16A16_SNORM => gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_SNORM,
        GPUTextureFormat::R16_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_R16_FLOAT,
        GPUTextureFormat::R16G16_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_R16G16_FLOAT,
        GPUTextureFormat::R16G16B16A16_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_FLOAT,
        GPUTextureFormat::R32_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_R32_FLOAT,
        GPUTextureFormat::R32G32_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_R32G32_FLOAT,
        GPUTextureFormat::R32G32B32A32_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_R32G32B32A32_FLOAT,
        GPUTextureFormat::R11G11B10_UFLOAT => gpu::SDL_GPU_TEXTUREFORMAT_R11G11B10_UFLOAT,
        GPUTextureFormat::R8_UINT => gpu::SDL_GPU_TEXTUREFORMAT_R8_UINT,
        GPUTextureFormat::R8G8_UINT => gpu::SDL_GPU_TEXTUREFORMAT_R8G8_UINT,
        GPUTextureFormat::R8G8B8A8_UINT => gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UINT,
        GPUTextureFormat::R16_UINT => gpu::SDL_GPU_TEXTUREFORMAT_R16_UINT,
        GPUTextureFormat::R16G16_UINT => gpu::SDL_GPU_TEXTUREFORMAT_R16G16_UINT,
        GPUTextureFormat::R16G16B16A16_UINT => gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UINT,
        GPUTextureFormat::R32_UINT => gpu::SDL_GPU_TEXTUREFORMAT_R32_UINT,
        GPUTextureFormat::R32G32_UINT => gpu::SDL_GPU_TEXTUREFORMAT_R32G32_UINT,
        GPUTextureFormat::R32G32B32A32_UINT => gpu::SDL_GPU_TEXTUREFORMAT_R32G32B32A32_UINT,
        GPUTextureFormat::R8_INT => gpu::SDL_GPU_TEXTUREFORMAT_R8_INT,
        GPUTextureFormat::R8G8_INT => gpu::SDL_GPU_TEXTUREFORMAT_R8G8_INT,
        GPUTextureFormat::R8G8B8A8_INT => gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_INT,
        GPUTextureFormat::R16_INT => gpu::SDL_GPU_TEXTUREFORMAT_R16_INT,
        GPUTextureFormat::R16G16_INT => gpu::SDL_GPU_TEXTUREFORMAT_R16G16_INT,
        GPUTextureFormat::R16G16B16A16_INT => gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_INT,
        GPUTextureFormat::R32_INT => gpu::SDL_GPU_TEXTUREFORMAT_R32_INT,
        GPUTextureFormat::R32G32_INT => gpu::SDL_GPU_TEXTUREFORMAT_R32G32_INT,
        GPUTextureFormat::R32G32B32A32_INT => gpu::SDL_GPU_TEXTUREFORMAT_R32G32B32A32_INT,
        GPUTextureFormat::R8G8B8A8_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM_SRGB,
        GPUTextureFormat::B8G8R8A8_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM_SRGB,
        GPUTextureFormat::BC1_RGBA_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM_SRGB,
        GPUTextureFormat::BC2_RGBA_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM_SRGB,
        GPUTextureFormat::BC3_RGBA_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM_SRGB,
        GPUTextureFormat::BC7_RGBA_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM_SRGB,
        GPUTextureFormat::D16_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_D16_UNORM,
        GPUTextureFormat::D24_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_D24_UNORM,
        GPUTextureFormat::D32_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_D32_FLOAT,
        GPUTextureFormat::D24_UNORM_S8_UINT => gpu::SDL_GPU_TEXTUREFORMAT_D24_UNORM_S8_UINT,
        GPUTextureFormat::D32_FLOAT_S8_UINT => gpu::SDL_GPU_TEXTUREFORMAT_D32_FLOAT_S8_UINT,
        GPUTextureFormat::ASTC_4x4_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM,
        GPUTextureFormat::ASTC_5x4_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM,
        GPUTextureFormat::ASTC_5x5_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM,
        GPUTextureFormat::ASTC_6x5_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM,
        GPUTextureFormat::ASTC_6x6_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM,
        GPUTextureFormat::ASTC_8x5_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM,
        GPUTextureFormat::ASTC_8x6_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM,
        GPUTextureFormat::ASTC_8x8_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM,
        GPUTextureFormat::ASTC_10x5_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM,
        GPUTextureFormat::ASTC_10x6_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM,
        GPUTextureFormat::ASTC_10x8_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM,
        GPUTextureFormat::ASTC_10x10_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM,
        GPUTextureFormat::ASTC_12x10_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM,
        GPUTextureFormat::ASTC_12x12_UNORM => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM,
        GPUTextureFormat::ASTC_4x4_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM_SRGB,
        GPUTextureFormat::ASTC_5x4_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM_SRGB,
        GPUTextureFormat::ASTC_5x5_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM_SRGB,
        GPUTextureFormat::ASTC_6x5_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM_SRGB,
        GPUTextureFormat::ASTC_6x6_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM_SRGB,
        GPUTextureFormat::ASTC_8x5_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM_SRGB,
        GPUTextureFormat::ASTC_8x6_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM_SRGB,
        GPUTextureFormat::ASTC_8x8_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM_SRGB,
        GPUTextureFormat::ASTC_10x5_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM_SRGB,
        GPUTextureFormat::ASTC_10x6_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM_SRGB,
        GPUTextureFormat::ASTC_10x8_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM_SRGB,
        GPUTextureFormat::ASTC_10x10_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM_SRGB,
        GPUTextureFormat::ASTC_12x10_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM_SRGB,
        GPUTextureFormat::ASTC_12x12_UNORM_SRGB => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM_SRGB,
        GPUTextureFormat::ASTC_4x4_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_4x4_FLOAT,
        GPUTextureFormat::ASTC_5x4_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x4_FLOAT,
        GPUTextureFormat::ASTC_5x5_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x5_FLOAT,
        GPUTextureFormat::ASTC_6x5_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x5_FLOAT,
        GPUTextureFormat::ASTC_6x6_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x6_FLOAT,
        GPUTextureFormat::ASTC_8x5_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x5_FLOAT,
        GPUTextureFormat::ASTC_8x6_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x6_FLOAT,
        GPUTextureFormat::ASTC_8x8_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x8_FLOAT,
        GPUTextureFormat::ASTC_10x5_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x5_FLOAT,
        GPUTextureFormat::ASTC_10x6_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x6_FLOAT,
        GPUTextureFormat::ASTC_10x8_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x8_FLOAT,
        GPUTextureFormat::ASTC_10x10_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x10_FLOAT,
        GPUTextureFormat::ASTC_12x10_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x10_FLOAT,
        GPUTextureFormat::ASTC_12x12_FLOAT => gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x12_FLOAT,
    }
}

pub fn sdl_to_gpu_texture_format(format: gpu::SDL_GPUTextureFormat) -> GPUTextureFormat {
    match format {
        gpu::SDL_GPU_TEXTUREFORMAT_R8_UNORM => GPUTextureFormat::R8,
        gpu::SDL_GPU_TEXTUREFORMAT_R8G8_UNORM => GPUTextureFormat::R8G8,
        gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM => GPUTextureFormat::R8G8B8A8,
        gpu::SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM => GPUTextureFormat::B8G8R8A8,
        gpu::SDL_GPU_TEXTUREFORMAT_A8_UNORM => GPUTextureFormat::A8_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R16_UNORM => GPUTextureFormat::R16_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16_UNORM => GPUTextureFormat::R16G16_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UNORM => GPUTextureFormat::R16G16B16A16_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R10G10B10A2_UNORM => GPUTextureFormat::R10G10B10A2_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_B5G6R5_UNORM => GPUTextureFormat::B5G6R5_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_B5G5R5A1_UNORM => GPUTextureFormat::B5G5R5A1_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_B4G4R4A4_UNORM => GPUTextureFormat::B4G4R4A4_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM => GPUTextureFormat::BC1_RGBA_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM => GPUTextureFormat::BC2_RGBA_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM => GPUTextureFormat::BC3_RGBA_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_BC4_R_UNORM => GPUTextureFormat::BC4_R_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_BC5_RG_UNORM => GPUTextureFormat::BC5_RG_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM => GPUTextureFormat::BC7_RGBA_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_BC6H_RGB_FLOAT => GPUTextureFormat::BC6H_RGB_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_BC6H_RGB_UFLOAT => GPUTextureFormat::BC6H_RGB_UFLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_R8_SNORM => GPUTextureFormat::R8_SNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R8G8_SNORM => GPUTextureFormat::R8G8_SNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_SNORM => GPUTextureFormat::R8G8B8A8_SNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R16_SNORM => GPUTextureFormat::R16_SNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16_SNORM => GPUTextureFormat::R16G16_SNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_SNORM => GPUTextureFormat::R16G16B16A16_SNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_R16_FLOAT => GPUTextureFormat::R16_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16_FLOAT => GPUTextureFormat::R16G16_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_FLOAT => GPUTextureFormat::R16G16B16A16_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_R32_FLOAT => GPUTextureFormat::R32_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_R32G32_FLOAT => GPUTextureFormat::R32G32_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_R32G32B32A32_FLOAT => GPUTextureFormat::R32G32B32A32_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_R11G11B10_UFLOAT => GPUTextureFormat::R11G11B10_UFLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_R8_UINT => GPUTextureFormat::R8_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_R8G8_UINT => GPUTextureFormat::R8G8_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UINT => GPUTextureFormat::R8G8B8A8_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_R16_UINT => GPUTextureFormat::R16_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16_UINT => GPUTextureFormat::R16G16_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_UINT => GPUTextureFormat::R16G16B16A16_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_R32_UINT => GPUTextureFormat::R32_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_R32G32_UINT => GPUTextureFormat::R32G32_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_R32G32B32A32_UINT => GPUTextureFormat::R32G32B32A32_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_R8_INT => GPUTextureFormat::R8_INT,
        gpu::SDL_GPU_TEXTUREFORMAT_R8G8_INT => GPUTextureFormat::R8G8_INT,
        gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_INT => GPUTextureFormat::R8G8B8A8_INT,
        gpu::SDL_GPU_TEXTUREFORMAT_R16_INT => GPUTextureFormat::R16_INT,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16_INT => GPUTextureFormat::R16G16_INT,
        gpu::SDL_GPU_TEXTUREFORMAT_R16G16B16A16_INT => GPUTextureFormat::R16G16B16A16_INT,
        gpu::SDL_GPU_TEXTUREFORMAT_R32_INT => GPUTextureFormat::R32_INT,
        gpu::SDL_GPU_TEXTUREFORMAT_R32G32_INT => GPUTextureFormat::R32G32_INT,
        gpu::SDL_GPU_TEXTUREFORMAT_R32G32B32A32_INT => GPUTextureFormat::R32G32B32A32_INT,
        gpu::SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM_SRGB => GPUTextureFormat::R8G8B8A8_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_B8G8R8A8_UNORM_SRGB => GPUTextureFormat::B8G8R8A8_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_BC1_RGBA_UNORM_SRGB => GPUTextureFormat::BC1_RGBA_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_BC2_RGBA_UNORM_SRGB => GPUTextureFormat::BC2_RGBA_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_BC3_RGBA_UNORM_SRGB => GPUTextureFormat::BC3_RGBA_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_BC7_RGBA_UNORM_SRGB => GPUTextureFormat::BC7_RGBA_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_D16_UNORM => GPUTextureFormat::D16_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_D24_UNORM => GPUTextureFormat::D24_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_D32_FLOAT => GPUTextureFormat::D32_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_D24_UNORM_S8_UINT => GPUTextureFormat::D24_UNORM_S8_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_D32_FLOAT_S8_UINT => GPUTextureFormat::D32_FLOAT_S8_UINT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM => GPUTextureFormat::ASTC_4x4_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM => GPUTextureFormat::ASTC_5x4_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM => GPUTextureFormat::ASTC_5x5_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM => GPUTextureFormat::ASTC_6x5_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM => GPUTextureFormat::ASTC_6x6_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM => GPUTextureFormat::ASTC_8x5_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM => GPUTextureFormat::ASTC_8x6_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM => GPUTextureFormat::ASTC_8x8_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM => GPUTextureFormat::ASTC_10x5_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM => GPUTextureFormat::ASTC_10x6_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM => GPUTextureFormat::ASTC_10x8_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM => GPUTextureFormat::ASTC_10x10_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM => GPUTextureFormat::ASTC_12x10_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM => GPUTextureFormat::ASTC_12x12_UNORM,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_4x4_UNORM_SRGB => GPUTextureFormat::ASTC_4x4_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x4_UNORM_SRGB => GPUTextureFormat::ASTC_5x4_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x5_UNORM_SRGB => GPUTextureFormat::ASTC_5x5_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x5_UNORM_SRGB => GPUTextureFormat::ASTC_6x5_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x6_UNORM_SRGB => GPUTextureFormat::ASTC_6x6_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x5_UNORM_SRGB => GPUTextureFormat::ASTC_8x5_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x6_UNORM_SRGB => GPUTextureFormat::ASTC_8x6_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x8_UNORM_SRGB => GPUTextureFormat::ASTC_8x8_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x5_UNORM_SRGB => GPUTextureFormat::ASTC_10x5_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x6_UNORM_SRGB => GPUTextureFormat::ASTC_10x6_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x8_UNORM_SRGB => GPUTextureFormat::ASTC_10x8_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x10_UNORM_SRGB => GPUTextureFormat::ASTC_10x10_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x10_UNORM_SRGB => GPUTextureFormat::ASTC_12x10_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x12_UNORM_SRGB => GPUTextureFormat::ASTC_12x12_UNORM_SRGB,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_4x4_FLOAT => GPUTextureFormat::ASTC_4x4_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x4_FLOAT => GPUTextureFormat::ASTC_5x4_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_5x5_FLOAT => GPUTextureFormat::ASTC_5x5_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x5_FLOAT => GPUTextureFormat::ASTC_6x5_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_6x6_FLOAT => GPUTextureFormat::ASTC_6x6_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x5_FLOAT => GPUTextureFormat::ASTC_8x5_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x6_FLOAT => GPUTextureFormat::ASTC_8x6_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_8x8_FLOAT => GPUTextureFormat::ASTC_8x8_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x5_FLOAT => GPUTextureFormat::ASTC_10x5_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x6_FLOAT => GPUTextureFormat::ASTC_10x6_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x8_FLOAT => GPUTextureFormat::ASTC_10x8_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_10x10_FLOAT => GPUTextureFormat::ASTC_10x10_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x10_FLOAT => GPUTextureFormat::ASTC_12x10_FLOAT,
        gpu::SDL_GPU_TEXTUREFORMAT_ASTC_12x12_FLOAT => GPUTextureFormat::ASTC_12x12_FLOAT,
        _ => panic!("Unknown texture format"),
    }
}

pub fn gpu_texture_access_to_sdl(access: GPUTextureAccess) -> gpu::SDL_GPUTextureUsageFlags {
    match access {
        GPUTextureAccess::Sampler => gpu::SDL_GPU_TEXTUREUSAGE_SAMPLER,
        GPUTextureAccess::RenderTarget => gpu::SDL_GPU_TEXTUREUSAGE_COLOR_TARGET,
        GPUTextureAccess::DepthStencil => gpu::SDL_GPU_TEXTUREUSAGE_DEPTH_STENCIL_TARGET,
        GPUTextureAccess::GraphicsStorage => gpu::SDL_GPU_TEXTUREUSAGE_GRAPHICS_STORAGE_READ,
        GPUTextureAccess::ComputeStorageRead => gpu::SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_READ,
        GPUTextureAccess::ComputeStorageWrite => gpu::SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_WRITE,
        GPUTextureAccess::ComputeStorageReadWrite => {
            gpu::SDL_GPU_TEXTUREUSAGE_COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE
        }
    }
}

pub fn gpu_texture_format_to_byte_size(format: GPUTextureFormat) -> u32 {
    match format {
        GPUTextureFormat::R8 => 1,
        GPUTextureFormat::R8G8 => 2,
        GPUTextureFormat::R8G8B8A8 => 4,
        GPUTextureFormat::B8G8R8A8 => 4,
        GPUTextureFormat::A8_UNORM => 1,
        GPUTextureFormat::R16_UNORM => 2,
        GPUTextureFormat::R16G16_UNORM => 4,
        GPUTextureFormat::R16G16B16A16_UNORM => 8,
        GPUTextureFormat::R10G10B10A2_UNORM => 4,
        GPUTextureFormat::B5G6R5_UNORM => 2,
        GPUTextureFormat::B5G5R5A1_UNORM => 2,
        GPUTextureFormat::B4G4R4A4_UNORM => 2,
        GPUTextureFormat::BC1_RGBA_UNORM => 8,
        GPUTextureFormat::BC2_RGBA_UNORM => 16,
        GPUTextureFormat::BC3_RGBA_UNORM => 16,
        GPUTextureFormat::BC4_R_UNORM => 8,
        GPUTextureFormat::BC5_RG_UNORM => 16,
        GPUTextureFormat::BC7_RGBA_UNORM => 16,
        GPUTextureFormat::BC6H_RGB_FLOAT => 16,
        GPUTextureFormat::BC6H_RGB_UFLOAT => 16,
        GPUTextureFormat::R8_SNORM => 1,
        GPUTextureFormat::R8G8_SNORM => 2,
        GPUTextureFormat::R8G8B8A8_SNORM => 4,
        GPUTextureFormat::R16_SNORM => 2,
        GPUTextureFormat::R16G16_SNORM => 4,
        GPUTextureFormat::R16G16B16A16_SNORM => 8,
        GPUTextureFormat::R16_FLOAT => 2,
        GPUTextureFormat::R16G16_FLOAT => 4,
        GPUTextureFormat::R16G16B16A16_FLOAT => 8,
        GPUTextureFormat::R32_FLOAT => 4,
        GPUTextureFormat::R32G32_FLOAT => 8,
        GPUTextureFormat::R32G32B32A32_FLOAT => 16,
        GPUTextureFormat::R11G11B10_UFLOAT => 4,
        GPUTextureFormat::R8_UINT => 1,
        GPUTextureFormat::R8G8_UINT => 2,
        GPUTextureFormat::R8G8B8A8_UINT => 4,
        GPUTextureFormat::R16_UINT => 2,
        GPUTextureFormat::R16G16_UINT => 4,
        GPUTextureFormat::R16G16B16A16_UINT => 8,
        GPUTextureFormat::R32_UINT => 4,
        GPUTextureFormat::R32G32_UINT => 8,
        GPUTextureFormat::R32G32B32A32_UINT => 16,
        GPUTextureFormat::R8_INT => 1,
        GPUTextureFormat::R8G8_INT => 2,
        GPUTextureFormat::R8G8B8A8_INT => 4,
        GPUTextureFormat::R16_INT => 2,
        GPUTextureFormat::R16G16_INT => 4,
        GPUTextureFormat::R16G16B16A16_INT => 8,
        GPUTextureFormat::R32_INT => 4,
        GPUTextureFormat::R32G32_INT => 8,
        GPUTextureFormat::R32G32B32A32_INT => 16,
        GPUTextureFormat::R8G8B8A8_UNORM_SRGB => 4,
        GPUTextureFormat::B8G8R8A8_UNORM_SRGB => 4,
        GPUTextureFormat::BC1_RGBA_UNORM_SRGB => 8,
        GPUTextureFormat::BC2_RGBA_UNORM_SRGB => 16,
        GPUTextureFormat::BC3_RGBA_UNORM_SRGB => 16,
        GPUTextureFormat::BC7_RGBA_UNORM_SRGB => 16,
        GPUTextureFormat::D16_UNORM => 2,
        GPUTextureFormat::D24_UNORM => 3,
        GPUTextureFormat::D32_FLOAT => 4,
        GPUTextureFormat::D24_UNORM_S8_UINT => 4,
        GPUTextureFormat::D32_FLOAT_S8_UINT => 5,
        GPUTextureFormat::ASTC_4x4_UNORM => 16,
        GPUTextureFormat::ASTC_5x4_UNORM => 16,
        GPUTextureFormat::ASTC_5x5_UNORM => 16,
        GPUTextureFormat::ASTC_6x5_UNORM => 16,
        GPUTextureFormat::ASTC_6x6_UNORM => 16,
        GPUTextureFormat::ASTC_8x5_UNORM => 16,
        GPUTextureFormat::ASTC_8x6_UNORM => 16,
        GPUTextureFormat::ASTC_8x8_UNORM => 16,
        GPUTextureFormat::ASTC_10x5_UNORM => 16,
        GPUTextureFormat::ASTC_10x6_UNORM => 16,
        GPUTextureFormat::ASTC_10x8_UNORM => 16,
        GPUTextureFormat::ASTC_10x10_UNORM => 16,
        GPUTextureFormat::ASTC_12x10_UNORM => 16,
        GPUTextureFormat::ASTC_12x12_UNORM => 16,
        GPUTextureFormat::ASTC_4x4_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_5x4_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_5x5_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_6x5_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_6x6_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_8x5_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_8x6_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_8x8_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_10x5_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_10x6_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_10x8_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_10x10_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_12x10_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_12x12_UNORM_SRGB => 16,
        GPUTextureFormat::ASTC_4x4_FLOAT => 16,
        GPUTextureFormat::ASTC_5x4_FLOAT => 16,
        GPUTextureFormat::ASTC_5x5_FLOAT => 16,
        GPUTextureFormat::ASTC_6x5_FLOAT => 16,
        GPUTextureFormat::ASTC_6x6_FLOAT => 16,
        GPUTextureFormat::ASTC_8x5_FLOAT => 16,
        GPUTextureFormat::ASTC_8x6_FLOAT => 16,
        GPUTextureFormat::ASTC_8x8_FLOAT => 16,
        GPUTextureFormat::ASTC_10x5_FLOAT => 16,
        GPUTextureFormat::ASTC_10x6_FLOAT => 16,
        GPUTextureFormat::ASTC_10x8_FLOAT => 16,
        GPUTextureFormat::ASTC_10x10_FLOAT => 16,
        GPUTextureFormat::ASTC_12x10_FLOAT => 16,
        GPUTextureFormat::ASTC_12x12_FLOAT => 16,
    }
}
