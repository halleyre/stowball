use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

enum WindowStatus {
    Inactive(WindowAttributes),
    Active(Window),
}

struct Graphics {
    window: WindowStatus,
}

impl ApplicationHandler for Graphics {

    // also called on first startup
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        use WindowStatus::*;
        if let Inactive(win_attr) = &self.window {
            // Option::take and Box would be more performant but .clone() is cleaner code
            self.window = Active(event_loop.create_window(win_attr.clone()).unwrap());
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
            }
            _ => (),
        }
    }
}

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut win_attr = WindowAttributes::default()
       .with_title("stowball");

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesExtWebSys;
        let canvas = web_sys::window()
            .and_then(|w| w.document()?
                .get_element_by_id("canvas")?
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .ok()
            );
        win_attr = win_attr.with_canvas(canvas);
    }

    let mut app = Graphics { window: WindowStatus::Inactive(win_attr) };
    event_loop.run_app(&mut app).unwrap();
}
