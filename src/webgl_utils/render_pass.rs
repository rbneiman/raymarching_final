use web_sys::{
    WebGl2RenderingContext,
    WebGlBuffer,
    WebGlProgram,
    WebGlTexture,
    WebGlUniformLocation,
    WebGlVertexArrayObject
};
use js_sys::{Float32Array, WebAssembly, Uint32Array};

struct UnloadedTextureConfig{
    name: String,
    path: String,
}

struct LoadedTextureConfig{
    name: String,
    texture: WebGlTexture
}

struct TextureInstance{
    name: String,
    texture: WebGlTexture,
    location: u32,
    unit: u32
}

struct AttributeConfig{
    name: String,
    size: u32,
    size_type: u32,

}

struct Attribute{
    name: String,
    location: WebGlUniformLocation,
    buffer: WebGlBuffer
}

struct UniformConfig{
    name: String,
    bind_function: fn(gl: WebGl2RenderingContext, loc: WebGlUniformLocation),
}

struct Uniform{
    location: WebGlUniformLocation,
    bind_function: fn(gl: WebGl2RenderingContext, loc: WebGlUniformLocation),
}

pub struct RenderPassConfig{
    f_shader: String,
    v_shader: String,
    attributes: Vec<AttributeConfig>,
    uniforms: Vec<UniformConfig>,
    textures_unloaded: Vec<UnloadedTextureConfig>,
    textures_loaded: Vec<LoadedTextureConfig>,
    index_buffer_data: Uint32Array,
}

pub struct RenderPass{
    ctx: WebGl2RenderingContext,
    shader_program: WebGlProgram,
    vao: WebGlVertexArrayObject,
    index_buffer: WebGlBuffer,
    attributes: Vec<Attribute>,
    uniforms: Vec<Uniform>,
    textures: Vec<TextureInstance>,
    index_buffer_data: Uint32Array
}

impl RenderPassConfig{
    // pub fn new() -> Self{
    //
    // }
    //
    //
    // pub fn configure(self) -> RenderPass{
    //
    // }
}

impl RenderPass{


    fn draw(){

    }
}