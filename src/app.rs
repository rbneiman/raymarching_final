use std::cell::RefCell;
use std::f32;
use std::rc::Rc;
use js_sys::{Float32Array, Uint32Array};
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement, WebGlUniformLocation};
use wasm_bindgen::{JsCast, JsValue};
use crate::{log, log_error};
use crate::input::InputManager;
use crate::shaders::{FRAG_SHADER, VERT_SHADER};
use crate::vec_lib::mat4;
use crate::vec_lib::mat4::Mat4f;
use crate::vec_lib::vec3::Vec3f;
use crate::webgl_utils::render_pass::{RenderPass, RenderPassConfig, UniformProvider};


pub struct TestApp{
    ctx: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    window: web_sys::Window,
    render_pass: RenderPass,
    input_manager: Rc<InputManager>,
    cam_uniform_provider: Rc<CamProvider>,
}

struct CamProvider{
    input_manager: Rc<InputManager>,
}

static INDEX_VALS: [u32; 3] = [2,1,0];
static VERTS: [f32; 6] = [-1.0f32,1.0f32,  1.0f32,1.0f32,  1.0f32,-1.0f32];
static CAM_POS : Vec3f = Vec3f::new(0.0, 0.0, -1.0);

impl TestApp {
    pub fn new(ctx: WebGl2RenderingContext, canvas: HtmlCanvasElement, window: web_sys::Window) -> Self{
        let indices = Uint32Array::new(&JsValue::from(INDEX_VALS.len()));
        let verts = Float32Array::new(&JsValue::from(VERTS.len()));
        indices.copy_from(&INDEX_VALS);
        verts.copy_from(&VERTS);

        let input_manager = Rc::new(InputManager::new(&canvas, &window));
        let cam_uniform_provider = Rc::new(CamProvider{input_manager: input_manager.clone()});

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
        .add_uniform(String::from("mvp"), cam_uniform_provider.clone());

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
            window,
            render_pass,
            input_manager,
            cam_uniform_provider
        }
    }

    // fn mat_to_str(mat : &Mat4f) -> String{
    //     let out = String::new();
    //     let vals = mat.vals();
    //     format!("{:#?}", vals)
    // }

    pub fn draw(&self){
        self.ctx.clear_color(0.0, 0.37254903, 0.37254903, 1.0);
        self.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT
            | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        self.render_pass.draw();
    }
}

impl UniformProvider for CamProvider{
    fn update(&self, gl: &WebGl2RenderingContext, loc: &WebGlUniformLocation) {
        let time = js_sys::Date::now() / 1000.0 % 100.0;
        let model = mat4::IDENTITY
            .translate(&Vec3f::new(0.0f32, 0.0f32, 5.0f32))
            .rotate3d(&Vec3f::new(0.0, 1.0, 0.0), time as f32);
        let view = self.input_manager.view_matrix();
        let proj = self.input_manager.proj_matrix();
        let mvp = proj.multiply_mat4(&view).multiply_mat4(&model)
            .transpose();
        let vals = mvp.vals();
        gl.uniform_matrix4fv_with_f32_array(Some(loc), false, vals);
    }
}
