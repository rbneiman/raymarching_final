use std::ops::Add;
use std::time::{Duration, Instant, SystemTime};
use js_sys::{Float32Array, Uint32Array};
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, HtmlCanvasElement, WebGlUniformLocation};
use wasm_bindgen::{JsCast, JsValue};
use crate::{log, log_error};
use crate::shaders::{FRAG_SHADER, VERT_SHADER};
use crate::vec_lib::mat4;
use crate::vec_lib::mat4::Mat4f;
use crate::vec_lib::vec3::Vec3f;
use crate::webgl_utils::render_pass;
use crate::webgl_utils::render_pass::{RenderPass, RenderPassConfig};


pub struct TestApp{
    ctx: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    render_pass: RenderPass
}

static INDEX_VALS: [u32; 3] = [2,1,0];
static VERTS: [f32; 6] = [-1.0f32,1.0f32,  1.0f32,1.0f32,  1.0f32,-1.0f32];
static CAM_POS : Vec3f = Vec3f::new(0.0, 0.0, -1.0);

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
        verts.buffer())
        .add_uniform(String::from("mvp"),
        |x, x1| {
            Self::get_mvp(x, x1);
        });

        let render_pass = match render_pass_cfg.configure(ctx.clone()) {
            Ok(res) =>{
                Some(res)
            }
            Err(err)=>{
                log_error!("{}", err.as_str());
                None
            }
        }.expect("should not error");



        TestApp{
            ctx,
            canvas,
            render_pass
        }
    }

    fn mat_to_str(mat : &Mat4f) -> String{
        let out = String::new();
        let vals = mat.vals();
        format!("{:#?}", vals)
    }

    pub fn get_mvp(gl: &WebGl2RenderingContext, loc: &WebGlUniformLocation){
        let time = js_sys::Date::now();

        let model = mat4::IDENTITY
            .translate(&Vec3f::new(0.0f32, 0.0f32, 5.0f32))
            .rotate3d(&Vec3f::new(0.0, 1.0, 0.0), time as f32);
        let view = Mat4f::look_at(
            &CAM_POS,
            &Vec3f::new(0.0,0.0,0.0),
            &Vec3f::new(0.0, 1.0, 0.0)
        );

        let perspective = Mat4f::perspective(
            45.0,
            1280.0 / 960.0,
            0.1,
            1000.0
        );
        let mvp = perspective.multiply_mat4(&view).multiply_mat4(&model)
            .transpose();
        let vals = mvp.vals();
        // let array = Float32Array::new(&JsValue::from(vals.len()));
        // array.copy_from(vals);
        // log!("{}", Self::mat_to_str(&model));
        gl.uniform_matrix4fv_with_f32_array(Some(loc), false, vals);
    }

    // pub fn start_loop(&self, window: web_sys::Window, ){
    //     window.request_animation_frame().expect("no animation frame").
    // }

    pub fn draw(&self){
        self.ctx.clear_color(0.0, 0.37254903, 0.37254903, 1.0);
        self.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT
            | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        self.render_pass.draw();
    }
}