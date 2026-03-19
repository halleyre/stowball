#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub mod wgpu;

use std::sync::Arc;
use winit::{
    event_loop::EventLoopProxy,
    window::{Window, WindowAttributes},
};

pub enum WindowStatus {
    Pending ( WindowAttributes ),
    Ready ( Arc<Window> ),
}

pub enum GraphicsEvent {
    Wgpu ( wgpu::WgpuStatus ),
}

pub struct Graphics {
    pub event_loop_proxy: EventLoopProxy<GraphicsEvent>,
    pub window: WindowStatus,
    pub wgpu: wgpu::WgpuStatus,
}
