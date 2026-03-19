mod graphics;

use graphics::{Graphics, GraphicsEvent};

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{WindowAttributes, WindowId},
};

#[cfg(not(target_arch = "wasm32"))]
fn queue_future(f: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(f);
}

#[cfg(target_arch = "wasm32")]
fn queue_future(f: impl Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(f);
}

impl ApplicationHandler<GraphicsEvent> for Graphics {

    // also called on first startup
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        use graphics::WindowStatus::*;
        if let Pending(win_attr) = &self.window {
            self.window = Ready(Arc::new(
                event_loop.create_window(win_attr.clone()).unwrap()));
                // take() & Box would be performant but .clone() is cleaner
        };

        let window = if let Ready(window) = &self.window { window.clone() }
            else { panic!() };

        let proxy = self.event_loop_proxy.clone();

        self.wgpu = graphics::wgpu::WgpuStatus::Pending;

        queue_future(async move {
            graphics::wgpu::init_wgpu(proxy, window).await });
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: GraphicsEvent) {
        match event {
            GraphicsEvent::Wgpu(status) => {
                self.wgpu = status;
                if let graphics::WindowStatus::Ready(window) = &self.window {
                    window.request_redraw();
                }
            },
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }

        let graphics::WindowStatus::Ready(window) = &self.window else { panic!() };

        let graphics::wgpu::WgpuStatus::Ready{
            instance,
            surface,
            surface_config,
            device,
            queue,
            render_pipeline,
        } = &mut self.wgpu else { return };
    
        match event {

            WindowEvent::Resized(size) => {
                (*surface_config).width = size.width.max(1);
                (*surface_config).height = size.height.max(1);
                surface.configure(device, surface_config);
                window.request_redraw();
            }

            WindowEvent::RedrawRequested => {

                let frame = {
                    use wgpu::SurfaceError::*;
                    match surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(Timeout) | Err(Other) => {
                            window.request_redraw();
                            return;
                        },
                        Err(Outdated) => {
                            surface.configure(device, surface_config);
                            window.request_redraw();
                            return;
                        },
                        Err(Lost) => {
                            *surface = instance.create_surface(window.clone()).unwrap();
                            surface.configure(device, surface_config);
                            window.request_redraw();
                            return;
                        },
                        Err(OutOfMemory) => panic!(), }};

                if frame.suboptimal {
                    surface.configure(device, surface_config);
                    window.request_redraw();
                    return;
                }

                let view = frame.texture.create_view(
                    &wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { label: None });

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    });
                    rpass.set_pipeline(render_pipeline);
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                window.pre_present_notify();
                frame.present();
            }

            WindowEvent::Occluded(occ) => {
                if !occ { window.request_redraw(); }
            }
            _ => {},
        }
    }
}

fn main_for_real() {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let win_attr = WindowAttributes::default()
        .with_title("stowball");

    #[cfg(target_arch = "wasm32")]
    let mut win_attr = win_attr;
    #[cfg(target_arch = "wasm32")]
    { win_attr = graphics::wasm::with_canvas(win_attr); }

    let mut app = Graphics {
        event_loop_proxy: event_loop.create_proxy(),
        wgpu: graphics::wgpu::WgpuStatus::Absent,
        window: graphics::WindowStatus::Pending(win_attr),
    };
    event_loop.run_app(&mut app).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    main_for_real();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    main_for_real();
}
