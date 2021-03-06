use std::borrow::Cow;
use std::sync::Arc;

use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferDescriptor, BufferUsages, RenderPipeline, SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension, VertexAttribute, VertexBufferLayout, VertexFormat};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use pth_render_lib::*;

use crate::GlobalState;
use crate::resource::ResourcesHandles;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(C, align(4))]
pub struct WaterWaveVertex {
    pub pos: [f32; 2],
}

pub struct WaterWave {
    pub vertex: [WaterWaveVertex; 4],
    pub radius: f32,
}

pub struct WaterWaveRender {
    frag_bind_group_layout: BindGroupLayout,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl WaterWaveRender {
    pub fn new(state: &GlobalState, target_color_state: wgpu::ColorTargetState, handles: &Arc<ResourcesHandles>) -> Self {
        let state = &state.wgpu_data;
        let device = &state.device;
        let frag_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }, BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            }],
        });
        //done bind group

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: (std::mem::size_of::<WaterWaveVertex>() * 8) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&(0..8).map(|obj_idx| {
                let offset = obj_idx as u16 * 6;
                [offset, offset + 1, offset + 2, offset + 1, offset + 2, offset + 3]
            }).collect::<Vec<_>>()),
            usage: BufferUsages::INDEX,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&state.screen_uni_bind_layout, &frag_bind_group_layout],
            push_constant_ranges: &[],
        });

        let vert = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(Cow::from(handles.shaders.read().unwrap().get("n2dt.v").unwrap())),
        });

        let frag = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(Cow::from(handles.shaders.read().unwrap().get("n2dt.f").unwrap())),
        });

        let vertex_len = std::mem::size_of::<WaterWaveVertex>();
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert,
                entry_point: "main",
                buffers: &[VertexBufferLayout {
                    array_stride: vertex_len as u64,
                    step_mode: Default::default(),
                    attributes: &[VertexAttribute {
                        format: VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }, VertexAttribute {
                        format: VertexFormat::Float32x2,
                        offset: 2 * 4,
                        shader_location: 1,
                    }],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag,
                entry_point: "main",
                targets: &[target_color_state],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        });

        Self {
            frag_bind_group_layout,
            render_pipeline,
            vertex_buffer,
            index_buffer,
        }
    }
}