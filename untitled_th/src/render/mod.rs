use std::any::type_name;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::process::abort;
use std::sync::Arc;
use std::thread::panicking;

use shaderc::ShaderKind;
use wgpu::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;

use pth_render_lib::*;
use root::audio::OpenalData;
use root::render::texture2d::Texture2DRender;
use root::resource::{ResourcesHandles, Texture};

use crate as root;
use crate::config::Config;
use crate::resource::TextureInfo;
use crate::ThreadPool;

pub mod texture2d;
pub mod water_wave;

/// Require sync.
#[derive(Debug)]
pub struct GlobalState {
    pub wgpu_data: WgpuData,
    pub handles: Arc<ResourcesHandles>,
    pub views: HashMap<String, crate::resource::Texture>,

    pub render: MainRendererData,

    pub dyn_data: DynamicData,
    pub config: Config,
    pub al: Option<OpenalData>,
    pub io_pool: ThreadPool,
}


pub trait EffectRenderer: Send + Sync + std::fmt::Debug + 'static {
    fn alive(&self) -> bool {
        true
    }

    fn render(&mut self, state: &GlobalState, renderer: &MainRendererData);
}

#[derive(Default, Debug)]
pub struct DynamicData {
    pub msgs: Vec<String>,
    pub effects: Vec<Box<dyn EffectRenderer>>,
}

#[derive(Debug)]
pub struct WgpuData {
    pub surface: wgpu::Surface,
    pub surface_cfg: wgpu::SurfaceConfiguration,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub screen_uni_buffer: Buffer,
    pub screen_uni_bind_layout: BindGroupLayout,
    pub screen_uni_bind: BindGroup,

    pub size_scale: [f32; 2],

}

impl WgpuData {
    #[inline]
    pub fn get_screen_size(&self) -> (u32, u32) {
        (self.surface_cfg.width, self.surface_cfg.height)
    }


    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_cfg.width = width;
        self.surface_cfg.height = height;
        self.surface.configure(&self.device, &self.surface_cfg);
        let size = [width as f32, height as f32];
        self.size_scale = [size[0] / 1600.0, size[1] / 900.0];
        self.queue.write_buffer(&self.screen_uni_buffer, 0, bytemuck::cast_slice(&size));
    }
    pub async fn new(window: &Window, config: &mut Config) -> Self {
        log::info!("New graphics state");
        let size = window.inner_size();
        log::info!("Got window inner size {:?}", size);

        let instance = wgpu::Instance::new(wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::PRIMARY));
        log::info!("Got wgpu  instance {:?}", instance);
        let surface = unsafe { instance.create_surface(window) };
        log::info!("Created surface {:?}", surface);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::util::power_preference_from_env().unwrap_or(PowerPreference::HighPerformance),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        log::info!("Got adapter {:?}", adapter);
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits {
                        max_bind_groups: 5,
                        ..wgpu::Limits::default()
                    },
                },
                None,
            )
            .await
            .unwrap();
        let (device, queue) = (Arc::new(device), Arc::new(queue));
        log::info!("Requested device {:?} and queue {:?}", device, queue);

        let mut format = surface.get_preferred_format(&adapter)
            .expect("get format from swap chain failed");
        log::info!("Adapter chose {:?} for swap chain format", format);
        format = TextureFormat::Bgra8Unorm;
        log::info!("Using {:?} for swap chain format", format);

        let surface_cfg = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::COPY_DST,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_cfg);

        let screen_uni_bind_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let size = [size.width as f32, size.height as f32];
        let screen_uni_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&size),
        });
        let screen_uni_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &screen_uni_bind_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &screen_uni_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });
        let size_scale = [surface_cfg.width as f32 / 1600.0, surface_cfg.height as f32 / 900.0];
        Self {
            surface,
            surface_cfg,
            device,
            queue,
            screen_uni_buffer,
            screen_uni_bind_layout,
            screen_uni_bind,
            size_scale,
        }
    }
}


#[derive(Debug)]
pub struct MainRenderViews {
    buffers: [Texture; 2],
    main: usize,
}


pub struct MainRendererData {
    pub render2d: Texture2DRender,
    pub staging_belt: wgpu::util::StagingBelt,
    pub glyph_brush: wgpu_glyph::GlyphBrush<()>,
    pub views: MainRenderViews,
}

impl Debug for MainRendererData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("render2d", &self.render2d)
            .field("glyph_brush", &self.glyph_brush)
            .field("views", &self.views)
            .finish()
    }
}

impl GlobalState {
    #[inline]
    pub fn get_screen_size(&self) -> (u32, u32) {
        self.wgpu_data.get_screen_size()
    }

    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.wgpu_data.resize(width, height);
        self.render.views = MainRenderViews::new(&self.wgpu_data);
    }

    pub async fn new(window: &Window, mut config: Config) -> GlobalState {
        let wgpu_data = WgpuData::new(window, &mut config).await;
        let mut res = ResourcesHandles::default();
        res.load_font("default", "cjkFonts_allseto_v1.11.ttf");
        res.load_with_compile_shader("n2dt.v", "normal2dtexture.vert", "main", ShaderKind::Vertex).unwrap();
        res.load_with_compile_shader("n2dt.f", "normal2dtexture.frag", "main", ShaderKind::Fragment).unwrap();


        let al = match OpenalData::new(&mut config) {
            Ok(data) => Some(data),
            Err(e) => {
                log::warn!("Cannot create openal context for {:?}" , e);
                None
            }
        };

        let render = MainRendererData::new(&wgpu_data, &config, &res);
        GlobalState {
            wgpu_data,
            handles: Arc::new(res),
            views: Default::default(),
            render,
            dyn_data: Default::default(),
            config,
            al,
            io_pool: ThreadPool::builder()
                .name_prefix("IO Thread")
                .before_stop(|idx| {
                    log::info!("IO Thread #{} stop", idx);
                    if panicking() {
                        log::error!("Someone panicked io thread, aborting...");
                        abort();
                    }
                })
                .create()
                .expect("Create pth io thread pool failed"),
        }
    }
}


impl MainRendererData {
    pub fn new(state: &WgpuData, config: &Config, handles: &ResourcesHandles) -> Self {
        let staging_belt = wgpu::util::StagingBelt::new(2048);
        let glyph_brush =
            wgpu_glyph::GlyphBrushBuilder::using_font(handles.fonts.read().unwrap()
                .get("default").unwrap().clone())
                .build(&state.device, state.surface_cfg.format);

        let render2d = Texture2DRender::new(state, config, state.surface_cfg.format.into(), handles);
        let views = MainRenderViews::new(state);
        Self {
            render2d,
            staging_belt,
            glyph_brush,
            views,
        }
    }
}


impl MainRenderViews {
    pub fn new(state: &WgpuData) -> Self {
        let size = state.get_screen_size();
        let texture_desc = wgpu::TextureDescriptor {
            label: None,
            size: Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: state.surface_cfg.format,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
        };
        let sampler_desc = wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: None,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            ..wgpu::SamplerDescriptor::default()
        };
        let buffer_a = {
            let texture = state.device.create_texture(&texture_desc);
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            let sampler = state.device.create_sampler(&sampler_desc);
            Texture {
                texture,
                view,
                sampler,
                info: TextureInfo::new(size.0, size.1),
            }
        };

        let buffer_b = {
            let texture = state.device.create_texture(&texture_desc);
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            let sampler = state.device.create_sampler(&sampler_desc);
            Texture {
                texture,
                view,
                sampler,
                info: TextureInfo::new(size.0, size.1),
            }
        };

        Self {
            buffers: [buffer_a, buffer_b],
            main: 0,
        }
    }

    pub fn get_screen(&self) -> &Texture {
        &self.buffers[self.main]
    }

    pub fn swap_screen(&mut self) -> (&Texture, &Texture) {
        let src = self.main;
        self.main = (self.main + 1) & 1;
        let dst = self.main;
        (&self.buffers[src], &self.buffers[dst])
    }
}
