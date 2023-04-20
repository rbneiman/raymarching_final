use js_sys::{Float32Array, Uint32Array};
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, HtmlCanvasElement};
use wasm_bindgen::{JsCast, JsValue};
use crate::log_error;
use crate::shaders::{FRAG_SHADER, VERT_SHADER};
use crate::webgl_utils::render_pass;
use crate::webgl_utils::render_pass::{RenderPass, RenderPassConfig};


pub struct TestApp{
    ctx: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    render_pass: RenderPass
}

static INDEX_VALS: [u32; 3] = [2,1,0];
static VERTS: [f32; 6] = [-1.0f32,1.0f32,  1.0f32,1.0f32,  1.0f32,-1.0f32];

impl TestApp {
    pub fn new(ctx: WebGl2RenderingContext, canvas: HtmlCanvasElement) -> Self{
        let indices = Uint32Array::new(&JsValue::from(INDEX_VALS.len()));
        let verts = Float32Array::new(&JsValue::from(VERTS.len()));
        indices.copy_from(&INDEX_VALS);
        verts.copy_from(&VERTS);

        let render_pass_cfg: RenderPassConfig = RenderPassConfig::new(
            VERT_SHADER.to_string(),
            FRAG_SHADER.to_string(),
            WebGl2RenderingContext::TRIANGLES,
            3,
            WebGl2RenderingContext::UNSIGNED_INT,
            0
        )
        .set_index_buffer_data(indices)
        .add_attribute_data(String::from("vertPos"),
        2,
    WebGl2RenderingContext::FLOAT,
        false,
        8,
        0,
        0,
        verts.buffer());

        let render_pass = match render_pass_cfg.configure(ctx.clone()) {
            Ok(res) =>{
                Some(res)
            }
            Err(err)=>{
                log_error!("{}", err.as_str());
                None
            }
        };

        TestApp{
            ctx,
            canvas,
            render_pass: render_pass.expect("should not error")
        }
    }

    // pub fn start_loop(&self, window: web_sys::Window, ){
    //     window.request_animation_frame().expect("no animation frame").
    // }

    pub fn draw(&self){
        self.render_pass.draw();
    }
}