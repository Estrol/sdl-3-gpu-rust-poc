#![allow(dead_code)]

// TODO: Implement other pipeline (compute)

use std::{
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
    sync::{Arc, Mutex},
};

use sdl3_sys::gpu;

use crate::{device::GPUDevice, shader::GPUShader, texture::GPUTextureFormat};

#[derive(Debug, Clone, Copy)]
pub enum GPUVertexElement {
    Int,
    Int2,
    Int3,
    Int4,
    Uint,
    Uint2,
    Uint3,
    Uint4,
    Float,
    Float2,
    Float3,
    Float4,
    Byte2,
    Byte4,
    Ubyte2,
    Ubyte4,
    Byte2Norm,
    Byte4Norm,
    Ubyte2Norm,
    Ubyte4Norm,
    Short2,
    Short4,
    Ushort2,
    Ushort4,
    Short2Norm,
    Short4Norm,
    Ushort2Norm,
    Ushort4Norm,
    Half2,
    Half4,
}

pub fn gpu_vertex_element_to_sdl(format: GPUVertexElement) -> gpu::SDL_GPUVertexElementFormat {
    match format {
        GPUVertexElement::Int => gpu::SDL_GPU_VERTEXELEMENTFORMAT_INT,
        GPUVertexElement::Int2 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_INT2,
        GPUVertexElement::Int3 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_INT3,
        GPUVertexElement::Int4 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_INT4,
        GPUVertexElement::Uint => gpu::SDL_GPU_VERTEXELEMENTFORMAT_UINT,
        GPUVertexElement::Uint2 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_UINT2,
        GPUVertexElement::Uint3 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_UINT3,
        GPUVertexElement::Uint4 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_UINT4,
        GPUVertexElement::Float => gpu::SDL_GPU_VERTEXELEMENTFORMAT_FLOAT,
        GPUVertexElement::Float2 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_FLOAT2,
        GPUVertexElement::Float3 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3,
        GPUVertexElement::Float4 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_FLOAT4,
        GPUVertexElement::Byte2 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_BYTE2,
        GPUVertexElement::Byte4 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_BYTE4,
        GPUVertexElement::Ubyte2 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2,
        GPUVertexElement::Ubyte4 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4,
        GPUVertexElement::Byte2Norm => gpu::SDL_GPU_VERTEXELEMENTFORMAT_BYTE2_NORM,
        GPUVertexElement::Byte4Norm => gpu::SDL_GPU_VERTEXELEMENTFORMAT_BYTE4_NORM,
        GPUVertexElement::Ubyte2Norm => gpu::SDL_GPU_VERTEXELEMENTFORMAT_UBYTE2_NORM,
        GPUVertexElement::Ubyte4Norm => gpu::SDL_GPU_VERTEXELEMENTFORMAT_UBYTE4_NORM,
        GPUVertexElement::Short2 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_SHORT2,
        GPUVertexElement::Short4 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_SHORT4,
        GPUVertexElement::Ushort2 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_USHORT2,
        GPUVertexElement::Ushort4 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_USHORT4,
        GPUVertexElement::Short2Norm => gpu::SDL_GPU_VERTEXELEMENTFORMAT_SHORT2_NORM,
        GPUVertexElement::Short4Norm => gpu::SDL_GPU_VERTEXELEMENTFORMAT_SHORT4_NORM,
        GPUVertexElement::Ushort2Norm => gpu::SDL_GPU_VERTEXELEMENTFORMAT_USHORT2_NORM,
        GPUVertexElement::Ushort4Norm => gpu::SDL_GPU_VERTEXELEMENTFORMAT_USHORT4_NORM,
        GPUVertexElement::Half2 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_HALF2,
        GPUVertexElement::Half4 => gpu::SDL_GPU_VERTEXELEMENTFORMAT_HALF4,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GPUVertexAttribute {
    pub location: u32,
    pub buffer_slot: u32,
    pub format: GPUVertexElement,
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum GPUColorBlendOp {
    Invalid,
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

#[derive(Debug, Clone, Copy)]
pub enum GPUColorBlendFactor {
    Invalid,
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    SrcAlphaSaturate,
}

pub fn gpu_color_blend_factor_to_sdl(color_factor: GPUColorBlendFactor) -> gpu::SDL_GPUBlendFactor {
    match color_factor {
        GPUColorBlendFactor::Invalid => gpu::SDL_GPU_BLENDFACTOR_INVALID,
        GPUColorBlendFactor::Zero => gpu::SDL_GPU_BLENDFACTOR_ZERO,
        GPUColorBlendFactor::One => gpu::SDL_GPU_BLENDFACTOR_ONE,
        GPUColorBlendFactor::SrcColor => gpu::SDL_GPU_BLENDFACTOR_SRC_COLOR,
        GPUColorBlendFactor::OneMinusSrcColor => gpu::SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_COLOR,
        GPUColorBlendFactor::DstColor => gpu::SDL_GPU_BLENDFACTOR_DST_COLOR,
        GPUColorBlendFactor::OneMinusDstColor => gpu::SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_COLOR,
        GPUColorBlendFactor::SrcAlpha => gpu::SDL_GPU_BLENDFACTOR_SRC_ALPHA,
        GPUColorBlendFactor::OneMinusSrcAlpha => gpu::SDL_GPU_BLENDFACTOR_ONE_MINUS_SRC_ALPHA,
        GPUColorBlendFactor::DstAlpha => gpu::SDL_GPU_BLENDFACTOR_DST_ALPHA,
        GPUColorBlendFactor::OneMinusDstAlpha => gpu::SDL_GPU_BLENDFACTOR_ONE_MINUS_DST_ALPHA,
        GPUColorBlendFactor::ConstantColor => gpu::SDL_GPU_BLENDFACTOR_CONSTANT_COLOR,
        GPUColorBlendFactor::OneMinusConstantColor => {
            gpu::SDL_GPU_BLENDFACTOR_ONE_MINUS_CONSTANT_COLOR
        }
        GPUColorBlendFactor::SrcAlphaSaturate => gpu::SDL_GPU_BLENDFACTOR_SRC_ALPHA_SATURATE,
    }
}

pub fn gpu_color_blend_op_to_sdl(color_op: GPUColorBlendOp) -> gpu::SDL_GPUBlendOp {
    match color_op {
        GPUColorBlendOp::Add => gpu::SDL_GPU_BLENDOP_ADD,
        GPUColorBlendOp::Subtract => gpu::SDL_GPU_BLENDOP_SUBTRACT,
        GPUColorBlendOp::ReverseSubtract => gpu::SDL_GPU_BLENDOP_REVERSE_SUBTRACT,
        GPUColorBlendOp::Min => gpu::SDL_GPU_BLENDOP_MIN,
        GPUColorBlendOp::Max => gpu::SDL_GPU_BLENDOP_MAX,
        _ => gpu::SDL_GPU_BLENDOP_INVALID,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GPUColorComponent {
    None = 0b0000,
    R = 0b0001,
    G = 0b0010,
    B = 0b0100,
    A = 0b1000,
}
impl PartialEq for GPUColorComponent {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

impl Eq for GPUColorComponent {}

impl BitOr for GPUColorComponent {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        unsafe { std::mem::transmute(self as u8 | rhs as u8) }
    }
}

impl BitAnd for GPUColorComponent {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        unsafe { std::mem::transmute(self as u8 & rhs as u8) }
    }
}

impl BitAndAssign for GPUColorComponent {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOrAssign for GPUColorComponent {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

pub struct GPUColorTargetBlendState {
    pub enable_blend: bool,
    pub enable_color_write_mask: bool,
    pub color_write_mask: GPUColorComponent,
    pub color_blend_op: GPUColorBlendOp,
    pub color_blend_factor_src: GPUColorBlendFactor,
    pub color_blend_factor_dst: GPUColorBlendFactor,
    pub alpha_blend_op: GPUColorBlendOp,
    pub alpha_blend_factor_src: GPUColorBlendFactor,
    pub alpha_blend_factor_dst: GPUColorBlendFactor,
}

pub enum GPUVertexInputRate {
    Vertex,
    Instance,
}

pub struct GPUVertexBufferDescription {
    pub slot: u32,
    pub pitch: u32,
    pub input_rate: GPUVertexInputRate,
}

pub enum GPUPrimitiveType {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

pub struct GPUGraphicsPipelineCreateInfo {
    pub texture_format: GPUTextureFormat,
    pub vertex_shader: Arc<Mutex<GPUShader>>,
    pub fragment_shader: Arc<Mutex<GPUShader>>,
    pub vertex_attributes: Vec<GPUVertexAttribute>,
    pub vertex_buffer_desc: GPUVertexBufferDescription,
    pub primitive_type: GPUPrimitiveType,
    pub blend_state: GPUColorTargetBlendState,
}

pub struct GPUGraphicsPipeline {
    pub pipeline: *mut gpu::SDL_GPUGraphicsPipeline,
    pub device: Arc<GPUDevice>,
}

impl Drop for GPUGraphicsPipeline {
    fn drop(&mut self) {
        unsafe {
            gpu::SDL_ReleaseGPUGraphicsPipeline(self.device.device, self.pipeline);
        }
    }
}

pub struct GPUComputePipeline {
    pub pipeline: *mut gpu::SDL_GPUComputePipeline,
    pub device: Arc<GPUDevice>,
}

impl Drop for GPUComputePipeline {
    fn drop(&mut self) {
        unsafe {
            gpu::SDL_ReleaseGPUComputePipeline(self.device.device, self.pipeline);
        }
    }
}
