use std::mem::offset_of;
use std::sync::Arc;

mod buffer;
mod command_buffer;
mod copypass;
mod device;
mod fence;
mod math;
mod pipeline;
mod renderpass;
mod shader;
mod texture;
mod texture_utils;
mod window;

use math::Color;
use math::Vertex;
use pipeline::GPUColorBlendFactor;
use pipeline::GPUColorBlendOp;
use pipeline::GPUColorComponent;
use pipeline::GPUColorTargetBlendState;
use pipeline::GPUGraphicsPipelineCreateInfo;
use pipeline::GPUPrimitiveType;
use pipeline::GPUVertexAttribute;
use pipeline::GPUVertexBufferDescription;
use pipeline::GPUVertexElement;
use pipeline::GPUVertexInputRate;
use shader::GPUShaderType;
use window::Window;

fn main() {
    let window = Window::new("Hello, World!", 800, 600);
    let gpu = window.create_gpu_device();

    if let Err(err) = gpu {
        panic!("Failed to create GPU device: {:?}", err);
    }

    let gpu = gpu.unwrap();

    // Vertex: { position: [x, y, z], color: [r, g, b, a], texcoord: [u, v] };
    let mut vertices = vec![
        Vertex {
            position: math::Vector3::new(-1.0, -1.0, 0.0), // Bottom-left corner
            color: Color::from_rgba(255, 255, 255, 255),
            texcoord: math::Vector2::new(0.0, 0.0),
        },
        Vertex {
            position: math::Vector3::new(1.0, -1.0, 0.0), // Bottom-right corner
            color: Color::from_rgba(255, 255, 255, 255),
            texcoord: math::Vector2::new(1.0, 0.0),
        },
        Vertex {
            position: math::Vector3::new(1.0, 1.0, 0.0), // Top-right corner
            color: Color::from_rgba(255, 255, 255, 255),
            texcoord: math::Vector2::new(1.0, 1.0),
        },
        Vertex {
            position: math::Vector3::new(-1.0, 1.0, 0.0), // Top-left corner
            color: Color::from_rgba(255, 255, 255, 255),
            texcoord: math::Vector2::new(0.0, 1.0),
        },
    ];

    // flip the y-axis
    for vertex in vertices.iter_mut() {
        vertex.position.y *= -1.0;
    }

    let indices = vec![0, 1, 2, 2, 3, 0];

    let vertices_size_bytes = size_of::<Vertex>() * vertices.len();
    let indices_size_bytes = size_of::<u32>() * indices.len();

    let vertex_buffer = gpu.create_buffer(vertices_size_bytes, buffer::GPUBufferType::Vertex);
    let index_buffer = gpu.create_buffer(indices_size_bytes, buffer::GPUBufferType::Index);

    if let Err(err) = vertex_buffer {
        panic!("Failed to create vertex buffer: {:?}", err);
    }

    if let Err(err) = index_buffer {
        panic!("Failed to create index buffer: {:?}", err);
    }

    let vertex_buffer = vertex_buffer.unwrap();
    let index_buffer = index_buffer.unwrap();

    let upload_buffer = gpu
        .create_transfer_buffer(vertices_size_bytes, buffer::GPUTransferUsage::Upload)
        .unwrap();

    let res = upload_buffer.write_buffer_t(&gpu, &vertex_buffer, &vertices);
    if let Err(err) = res {
        panic!("Failed to write vertex buffer: {:?}", err);
    }

    let upload_buffer = gpu
        .create_transfer_buffer(indices_size_bytes, buffer::GPUTransferUsage::Upload)
        .unwrap();

    let res = upload_buffer.write_buffer_t(&gpu, &index_buffer, &indices);
    if let Err(err) = res {
        panic!("Failed to write index buffer: {:?}", err);
    }

    println!("Hello, world!");

    let source = include_bytes!("../shaders/default.vert.spv");
    let vertex_shader = gpu.create_shader(GPUShaderType::Vertex, source, 0, 0, 0);

    let source = include_bytes!("../shaders/default.frag.spv");
    let fragment_shader = gpu.create_shader(GPUShaderType::Fragment, source, 1, 0, 0);

    if let Err(err) = vertex_shader {
        panic!("Failed to load vertex shader: {:?}", err);
    }

    if let Err(err) = fragment_shader {
        panic!("Failed to load fragment shader: {:?}", err);
    }

    let vertex_shader = vertex_shader.unwrap();
    let fragment_shader = fragment_shader.unwrap();

    let pipeline_create_info = GPUGraphicsPipelineCreateInfo {
        texture_format: gpu.get_swapchain_format(),
        vertex_shader: Arc::clone(&vertex_shader),
        fragment_shader: Arc::clone(&fragment_shader),
        vertex_attributes: vec![
            GPUVertexAttribute {
                location: 0,
                buffer_slot: 0,
                format: GPUVertexElement::Float3,
                offset: 0,
            },
            GPUVertexAttribute {
                location: 1,
                buffer_slot: 0,
                format: GPUVertexElement::Float4,
                offset: offset_of!(Vertex, color) as u32,
            },
            GPUVertexAttribute {
                location: 2,
                buffer_slot: 0,
                format: GPUVertexElement::Float2,
                offset: offset_of!(Vertex, texcoord) as u32,
            },
        ],
        vertex_buffer_desc: GPUVertexBufferDescription {
            slot: 0,
            pitch: size_of::<Vertex>() as u32,
            input_rate: GPUVertexInputRate::Vertex,
        },
        primitive_type: GPUPrimitiveType::TriangleList,
        blend_state: GPUColorTargetBlendState {
            enable_blend: true,
            enable_color_write_mask: true,
            color_write_mask: GPUColorComponent::A
                | GPUColorComponent::R
                | GPUColorComponent::G
                | GPUColorComponent::B,
            color_blend_op: GPUColorBlendOp::Add,
            color_blend_factor_src: GPUColorBlendFactor::SrcAlpha,
            color_blend_factor_dst: GPUColorBlendFactor::OneMinusSrcAlpha,
            alpha_blend_op: GPUColorBlendOp::Add,
            alpha_blend_factor_src: GPUColorBlendFactor::One,
            alpha_blend_factor_dst: GPUColorBlendFactor::OneMinusSrcAlpha,
        },
    };

    let pipeline = gpu.create_graphics_pipeline(&pipeline_create_info);
    if let Err(err) = pipeline {
        panic!("Failed to create graphics pipeline: {:?}", err);
    }

    let pipeline = pipeline.unwrap();

    println!("Hello, world!");

    let texture = gpu.create_texture_from_file("./assets/test.png");
    if let Err(err) = texture {
        panic!("Failed to load texture: {:?}", err);
    }

    println!("Hello world");

    let pipeline = pipeline.lock().unwrap();
    let texture = texture.unwrap();
    let texture = texture.lock().unwrap();

    while !window.should_close() {
        let command_buffer = gpu.acquire_command_buffer();
        if let Err(_err) = command_buffer {
            continue;
        }

        let mut command_buffer = command_buffer.unwrap();
        let swapchain = gpu.acquire_swapchain(&mut command_buffer);
        if let Err(_err) = swapchain {
            continue;
        }

        let swapchain = swapchain.unwrap();

        if let Some(swapchain) = swapchain {
            let color = Color::new(0.0, 0.0, 0.0, 1.0);

            let renderpass = gpu.begin_renderpass(&mut command_buffer, &color, &swapchain);
            if let Ok(renderpass) = renderpass {
                let mut renderpass = renderpass.lock().unwrap();

                renderpass.bind_vertex_buffer(&vertex_buffer, 0);
                renderpass.bind_index_buffer(&index_buffer, 0);
                renderpass.bind_graphics_pipeline(&pipeline);
                renderpass.bind_fragment_samplers(&texture);
                renderpass.draw_indexed(6, 0, 0);

                if let Err(err) = renderpass.end() {
                    println!("Failed to end renderpass: {:?}", err);
                }
            }
        }

        if let Err(msg) = command_buffer.submit() {
            println!("Failed to submit command buffer: {:?}", msg);
        }
    }
}
