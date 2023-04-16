use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation, WebGlVertexArrayObject};

struct UnloadedTextureConfig{
    name: String,
    path: String,
}

struct LoadedTextureConfig{
    name: String,
}

struct TextureInstance{
    name: String,
    texture: WebGlTexture,
    location: u32,
    unit: u32
}

struct AttributeConfig{

}

struct Attribute{

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
    textures_loaded: Vec<LoadedTextureConfig>
}

pub struct RenderPass{
    ctx: WebGl2RenderingContext,
    shader_program: WebGlProgram,
    vao: WebGlVertexArrayObject,
    index_buffer: WebGlBuffer,
    attributes: Vec<Attribute>,
    uniforms: Vec<Uniform>,
    textures: Vec<TextureInstance>,
}

impl RenderPassConfig{
    pub fn new() -> Self{

    }

    pub fn configure() -> RenderPass{

    }
}

impl RenderPass{


    fn draw(){

    }
}