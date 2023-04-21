use std::rc::Rc;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext};
use std::cell::RefCell;
use crate::app::TestApp;

mod shaders;
mod utils;
mod app;
mod input;

mod webgl_utils{
    pub mod render_pass;
    pub mod utils;
    pub mod camera;
}

pub mod vec_lib{
    pub mod vec2;
    pub mod vec3;
    pub mod vec4;

    pub mod mat2;
    pub mod mat3;
    pub mod mat4;

    pub mod quat;
}



// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window : web_sys::Window = web_sys::window().ok_or(JsValue::null())?;
    let document = window.document().ok_or(JsValue::null())?;
    let canvas_element = document.get_element_by_id("glCanvas")
        .ok_or(JsValue::null())?;
    let canvas = canvas_element.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas.get_context("webgl2")
        .and_then(|res| res.ok_or(JsValue::null()))
        .and_then(|obj: Object|
            obj.dyn_into::<WebGl2RenderingContext>().map_err(|_| JsValue::null())
    )?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let app = TestApp::new(context, canvas);
    *g.borrow_mut() = Some(Closure::new(move || {
        // log!("draw");
        app.draw();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    // console::log_1(&JsValue::from_str("Hello world!"));
    Ok(())
}
