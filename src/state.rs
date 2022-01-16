use crate::vertex;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    pub clear_color: wgpu::Color,
    pub render_pipeline: wgpu::RenderPipeline,
    pub objects_vec: Vec<vertex::RenderObject>,
    pub objects_count: u32,
}
async fn get_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: true,
        })
        .await
        .unwrap()
}

async fn get_device_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: Some("Device Descriptor"),
            },
            None,
        )
        .await
        .unwrap()
}
fn get_config(
    size: &winit::dpi::PhysicalSize<u32>,
    surface: &wgpu::Surface,
    adapter: &wgpu::Adapter,
) -> wgpu::SurfaceConfiguration {
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(adapter).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    }
}
enum ShaderLang {
    Wgsl,
    Glsl,
    Hlsl,
}

fn load_shader(source_str: &str, lang: ShaderLang) -> wgpu::ShaderModuleDescriptor {
    use wgpu::ShaderSource;
    let shader_source = match lang {
        ShaderLang::Wgsl => ShaderSource::Wgsl(source_str.into()),
        ShaderLang::Glsl => {
            todo!()
        }
        ShaderLang::Hlsl => todo!(),
    };
    wgpu::ShaderModuleDescriptor {
        label: Some(source_str.into()),
        source: shader_source,
    }
}
impl State {
    pub async fn new(window: &Window) -> Self {
        let color = wgpu::Color::default();
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = get_adapter(&instance, &surface).await;
        let (device, queue) = get_device_queue(&adapter).await;
        let config = get_config(&size, &surface, &adapter);
        surface.configure(&device, &config);

        let shader = device
            .create_shader_module(&load_shader(include_str!("shader.wgsl"), ShaderLang::Wgsl));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex::Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        let triangle_object = vertex::RenderObject::from_mesh(vertex::Mesh::triangle(), &device);
        let objects_vec = vec![triangle_object];
        let count = objects_vec.iter().map(|o| o.count).sum();
        Self {
            clear_color: color,
            surface,
            device,
            queue,
            config,
            size,

            render_pipeline,
            objects_vec,
            objects_count: count,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if (new_size.width > 0 && new_size.height > 0) {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        true
    }

    pub fn update(&mut self) {
        ()
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            for object in &self.objects_vec {
                render_pass.set_vertex_buffer(0, object.vertex_buffer.slice(..))
            }
            render_pass.draw(0..self.objects_count, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
