use std::rc::Rc;
use js_sys::{Float32Array, Uint32Array};
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation, WebGlFramebuffer, WebGlTexture,
    WebGlRenderbuffer};
use crate::input::InputManager;
use crate::shaders::{CLOUD_FRAG_SHADER, FRACTAL_FRAG_SHADER, FRAG_SHADER, PIXEL_VERT_SHADER, VERT_SHADER};
use crate::vec_lib::mat4;
use crate::vec_lib::vec3::Vec3f;
use crate::webgl_utils::render_pass::{RenderPass, RenderPassConfig, UniformProvider};
use web_sys::WebGl2RenderingContext as gl;


pub struct RasterRenderPass{
    ctx: WebGl2RenderingContext,
    render_pass: RenderPass,
    uniform_provider: Rc<RasterUniformProvider>,
    framebuffer: WebGlFramebuffer,
    depth_buffer: WebGlRenderbuffer,
    color_texture: WebGlTexture,
}

pub struct FractalRenderPass{
    ctx: WebGl2RenderingContext,
    render_pass: RenderPass,
    uniform_provider: Rc<FractalUniformProvider>,
    framebuffer: WebGlFramebuffer,
}

pub struct CloudRenderPass{
    ctx: WebGl2RenderingContext,
    render_pass: RenderPass,
    uniform_provider: Rc<FractalUniformProvider>
}

struct RasterUniformProvider{
    input_manager: Rc<InputManager>,
}

struct FractalUniformProvider{
    input_manager: Rc<InputManager>,
}


static PIXEL_INDEX_VALS: [u32; 6] = [3,1,0, 0,2,3];
static PIXEL_VERTS: [f32;8] = [-1.0,1.0,  1.0,1.0,  -1.0,-1.0,  1.0,-1.0];

fn setup_pixel_shader(frag_shader: String) -> RenderPassConfig{
    let indices = Uint32Array::new(&JsValue::from(PIXEL_INDEX_VALS.len()));
    let verts = Float32Array::new(&JsValue::from(PIXEL_VERTS.len()));
    indices.copy_from(&PIXEL_INDEX_VALS);
    verts.copy_from(&PIXEL_VERTS);


    RenderPassConfig::new(
        PIXEL_VERT_SHADER.to_string(),
        frag_shader,
        gl::TRIANGLES,
        6,
        gl::UNSIGNED_INT,
        0
    )
    .set_index_buffer_data(indices)
    .add_attribute_data(String::from("vertPos"),
        2,
    gl::FLOAT,
        false,
        8,
        0,
        0,
        verts.buffer()
    )
}

static INDEX_VALS: [u32; 3] = [2,1,0];
static VERTS: [f32; 6] = [-1.0f32,1.0f32,  1.0f32,1.0f32,  1.0f32,-1.0f32];

impl RasterRenderPass{
    pub fn new(ctx: WebGl2RenderingContext, input_manager: Rc<InputManager>)
        -> Result<Self, String>{
        let uniform_provider = Rc::new(RasterUniformProvider{input_manager: input_manager.clone()});

        let indices = Uint32Array::new(&JsValue::from(INDEX_VALS.len()));
        let verts = Float32Array::new(&JsValue::from(VERTS.len()));
        indices.copy_from(&INDEX_VALS);
        verts.copy_from(&VERTS);

        let render_pass_cfg: RenderPassConfig = RenderPassConfig::new(
            VERT_SHADER.to_string(),
            FRAG_SHADER.to_string(),
            gl::TRIANGLES,
            3,
            gl::UNSIGNED_INT,
            0
        )
        .set_index_buffer_data(indices)
        .add_attribute_data(String::from("vertPos"),
        2,
    gl::FLOAT,
        false,
        8,
        0,
        0,
        verts.buffer())
        .add_uniform(String::from("mvp"), uniform_provider.clone(),0);
        let render_pass = render_pass_cfg.configure(ctx.clone())?;

        let color_texture = ctx.create_texture()
            .ok_or(String::from("Failed to create color texture."))?;
        ctx.bind_texture(gl::TEXTURE_2D, Some(&color_texture));
        ctx.tex_storage_2d(gl::TEXTURE_2D,
                           1,
            gl::RGBA32F,
            1280, 960
        );
        ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        let framebuffer = ctx.create_framebuffer()
            .ok_or(String::from("Failed to create frame buffer."))?;
        ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&framebuffer));
        ctx.framebuffer_texture_2d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                                   gl::TEXTURE_2D, Some(&color_texture), 0);

        let depth_buffer = ctx.create_renderbuffer()
            .ok_or(String::from("Failed to create depth buffer."))?;
        ctx.bind_renderbuffer(gl::RENDERBUFFER, Some(&depth_buffer));
        ctx.renderbuffer_storage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT16, 1280, 960);
        ctx.framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, Some(&depth_buffer));


        ctx.bind_framebuffer(gl::FRAMEBUFFER, None);

        Ok(Self{
            ctx,
            render_pass,
            uniform_provider,
            framebuffer,
            depth_buffer,
            color_texture
        })
    }

    pub fn depth_buffer(&self) -> &WebGlRenderbuffer{
        &self.depth_buffer
    }

    pub fn color_texture(&self) -> &WebGlTexture{
        &self.color_texture
    }

    pub fn draw(&self){
        self.ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&self.framebuffer));

        let buffers = js_sys::Uint32Array::new(&JsValue::from(1));
        buffers.copy_from(&[gl::COLOR_ATTACHMENT0]);
        self.ctx.draw_buffers(&buffers);

        self.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT
            | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        self.render_pass.draw();

        self.ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
    }
}

impl FractalRenderPass{
    pub fn new(ctx: WebGl2RenderingContext, input_manager: Rc<InputManager>,
               color_texture: &WebGlTexture, depth_buffer: &WebGlRenderbuffer)
        -> Result<Self, String>{
        let uniform_provider = Rc::new(FractalUniformProvider{input_manager: input_manager.clone()});
        let render_pass_cfg: RenderPassConfig = setup_pixel_shader(FRACTAL_FRAG_SHADER.to_string())
            .add_uniform(String::from("invProjMat"), uniform_provider.clone(), 0)
            .add_uniform(String::from("invViewMat"), uniform_provider.clone(), 1)
            .add_uniform(String::from("viewProjMat"), uniform_provider.clone(), 2)
            .add_uniform(String::from("time"), uniform_provider.clone(), 3);
        let render_pass = render_pass_cfg.configure(ctx.clone())?;

        let framebuffer = ctx.create_framebuffer()
            .ok_or(String::from("Failed to create frame buffer."))?;
        ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&framebuffer));
        ctx.framebuffer_texture_2d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                                   gl::TEXTURE_2D, Some(&color_texture), 0);

        ctx.bind_renderbuffer(gl::RENDERBUFFER, Some(&depth_buffer));
        ctx.framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, Some(&depth_buffer));

        ctx.bind_framebuffer(gl::FRAMEBUFFER, None);

        Ok(Self{
            ctx,
            render_pass,
            uniform_provider,
            framebuffer
        })
    }

    pub fn draw(&self){
        self.ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&self.framebuffer));

        let buffers = js_sys::Uint32Array::new(&JsValue::from(1));
        buffers.copy_from(&[gl::COLOR_ATTACHMENT0]);
        self.ctx.draw_buffers(&buffers);

        self.render_pass.draw();

        self.ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
    }
}

impl CloudRenderPass{
    pub fn new(ctx: WebGl2RenderingContext, input_manager: Rc<InputManager>,
               color_texture: &WebGlTexture)
        -> Result<Self, String>{
        let fractal_uniform_provider = Rc::new(FractalUniformProvider{input_manager: input_manager.clone()});
        let render_pass_cfg: RenderPassConfig = setup_pixel_shader(CLOUD_FRAG_SHADER.to_string())
            .add_uniform(String::from("invProjMat"), fractal_uniform_provider.clone(), 0)
            .add_uniform(String::from("invViewMat"), fractal_uniform_provider.clone(), 1)
            .add_uniform(String::from("viewProjMat"), fractal_uniform_provider.clone(), 2)
            .add_uniform(String::from("time"), fractal_uniform_provider.clone(), 3)
            .add_texture(color_texture.clone(), String::from("colorTex"));
        let render_pass = render_pass_cfg.configure(ctx.clone())?;
        Ok(Self{
            ctx,
            render_pass,
            uniform_provider: fractal_uniform_provider
        })
    }

    pub fn draw(&self){
        self.render_pass.draw();
    }
}

impl UniformProvider for FractalUniformProvider{
    fn update(&self, gl: &WebGl2RenderingContext, loc: &WebGlUniformLocation, index: u32) {
        if index == 3 {
            let time = js_sys::Date::now() / 1000.0 % 100.0;
            gl.uniform1fv_with_f32_array(Some(loc), &[time as f32]);
            return;
        }
        let mat = match index {
            0 =>{
                self.input_manager.proj_matrix().inverse().transpose()
            },
            1 =>{
                self.input_manager.view_matrix().inverse().transpose()
            }
            _ =>{
                let view = self.input_manager.view_matrix();
                let proj = self.input_manager.proj_matrix();
                proj.multiply_mat4(&view).transpose()
            }
        };
        let vals = mat.vals();
        gl.uniform_matrix4fv_with_f32_array(Some(loc), false,vals);
    }
}

impl UniformProvider for RasterUniformProvider{
    fn update(&self, gl: &WebGl2RenderingContext, loc: &WebGlUniformLocation, _index: u32) {
        let time = js_sys::Date::now() / 1000.0 % 100.0;
        let model = mat4::IDENTITY
            .translate(&Vec3f::new(0.0f32, 0.0f32, 5.0f32))
            .rotate3d(&Vec3f::new(0.0, 1.0, 0.0), 0.0 as f32);
        let view = self.input_manager.view_matrix();
        let proj = self.input_manager.proj_matrix();
        let mvp = proj.multiply_mat4(&view).multiply_mat4(&model)
            .transpose();
        let vals = mvp.vals();
        gl.uniform_matrix4fv_with_f32_array(Some(loc), false, vals);
    }
}