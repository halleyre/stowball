use std::{borrow::Cow, sync::Arc};
use winit::{
    event_loop::EventLoopProxy,
    window::Window,
};
use wgpu::{
    Instance,
    Surface,
    Device,
    Queue,
    RenderPipeline,
};

pub enum WgpuStatus {
    Absent,
    Pending,
    Ready {
        instance: Instance,
        surface: Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
        device: Device,
        queue: Queue,
        render_pipeline: RenderPipeline,
    },
}

pub async fn init_wgpu(
    event_loop: EventLoopProxy<super::GraphicsEvent>,
    window: Arc<Window>) {

    let window_size = window.inner_size();

    let instance = Instance::new(
        &wgpu::InstanceDescriptor::from_env_or_default());

    let surface = instance.create_surface(window).unwrap();

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: Default::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).await.unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            // Make sure we use the texture resolution limits from the adapter,
            // so we can support images the size of the swapchain.
            required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            trace: wgpu::Trace::Off,
        }).await.unwrap();

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/simple.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        immediate_size: 0,
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    });

    let surface_config = surface
        .get_default_config(&adapter, window_size.width.max(1), window_size.height.max(1))
        .unwrap();
    surface.configure(&device, &surface_config);

    let _ = event_loop.send_event(super::GraphicsEvent::Wgpu(
        WgpuStatus::Ready{
            instance,
            surface,
            surface_config,
            device,
            queue,
            render_pipeline,
    }));
}
