use winit::window::WindowAttributes;

pub fn with_canvas(win_attr: WindowAttributes) -> WindowAttributes { 
    use wasm_bindgen::JsCast;
    use winit::platform::web::WindowAttributesExtWebSys;

    let canvas = web_sys::window()
        .and_then(|w| w.document()?
            .get_element_by_id("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .ok());

    win_attr.with_canvas(canvas)
}
