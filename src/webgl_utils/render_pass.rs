use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation, WebGlVertexArrayObject};
use js_sys::{Uint32Array, ArrayBuffer};
use wasm_bindgen::{JsValue};
use crate::{log, log_warn};
use crate::webgl_utils::utils::util_create_program;

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
    location: WebGlUniformLocation,
    unit: u32
}

struct AttributeConfig{
    name: String,
    size: u32,
    size_type: u32,
    normalized: bool,
    stride: u32,
    offset: u32,
    divisor: u32,
    data: ArrayBuffer,
}

struct Attribute{
    buffer: WebGlBuffer,
}

struct UniformConfig{
    name: String,
    bind_function: fn(gl: &WebGl2RenderingContext, loc: &WebGlUniformLocation),
}

struct Uniform{
    name: String,
    location: WebGlUniformLocation,
    bind_function: fn(gl: &WebGl2RenderingContext, loc: &WebGlUniformLocation),
}

pub struct RenderPassConfig{
    v_shader: String,
    f_shader: String,
    draw_mode: u32,
    draw_count: i32,
    draw_type: u32,
    draw_offset: i32,
    attributes: Vec<AttributeConfig>,
    uniforms: Vec<UniformConfig>,
    textures_unloaded: Vec<UnloadedTextureConfig>,
    textures_loaded: Vec<LoadedTextureConfig>,
    index_buffer_data: Uint32Array,
}

pub struct RenderPass {
    ctx: WebGl2RenderingContext,
    shader_program: WebGlProgram,
    draw_mode: u32,
    draw_count: i32,
    draw_type: u32,
    draw_offset: i32,
    vao: WebGlVertexArrayObject,
    index_buffer: WebGlBuffer,
    attributes: HashMap<String, Attribute>,
    uniforms: Vec<Uniform>,
    textures: Vec<TextureInstance>,
    index_buffer_data: Uint32Array
}

impl RenderPassConfig{
    pub fn new(v_shader: String,
                f_shader: String,
                draw_mode: u32,
                draw_count: i32,
                draw_type: u32,
                draw_offset: i32) -> Self{
        RenderPassConfig{
            v_shader,
            f_shader,
            draw_mode,
            draw_count,
            draw_type,
            draw_offset,
            attributes: Vec::new(),
            uniforms: Vec::new(),
            textures_unloaded: Vec::new(),
            textures_loaded: Vec::new(),
            index_buffer_data: Uint32Array::new(&JsValue::from(0u8)),
        }
    }

    // pub fn add_attribute_named(&mut self, name: String, size: u32, size_type: u32, normalized:bool,
    // stride: u32, offset:u32, buffer_name: String){
    //
    // }
    //
    pub fn add_attribute_data(mut self, name: String, size: u32, size_type: u32, normalized:bool,
    stride: u32, offset:u32, divisor: u32, data: ArrayBuffer) -> Self{
        self.attributes.push(AttributeConfig{
            name,
            size,
            size_type,
            normalized,
            stride,
            offset,
            divisor,
            data
        });
        self
    }

    pub fn add_uniform(mut self, name: String,
                       bind_function: fn(gl: &WebGl2RenderingContext, loc: &WebGlUniformLocation)) -> Self{
        self.uniforms.push(UniformConfig{name, bind_function});
        self
    }

    pub fn add_texture(mut self, texture: WebGlTexture, name: String) -> Self{
        self.textures_loaded.push(LoadedTextureConfig{name, texture});
        self
    }

    pub fn set_index_buffer_data(mut self, data: Uint32Array) -> Self{
        self.index_buffer_data = data;
        self
    }

    pub fn configure(self, gl: WebGl2RenderingContext)
                         -> Result<RenderPass, String>{
        let shader_program = util_create_program(&gl, &self.v_shader, &self.f_shader)?;
        gl.use_program(Some(&shader_program));

        let vao = gl.create_vertex_array()
            .ok_or(String::from("Failed to create vertex array object."))?;
        gl.bind_vertex_array(Some(&vao));

        let index_buffer = gl.create_buffer()
            .ok_or(String::from("Failed to create index buffer."))?;
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.buffer_data_with_opt_array_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer_data.buffer()),
            WebGl2RenderingContext::STATIC_DRAW
        );

        let mut attributes: HashMap<String, Attribute> = HashMap::new();
        for attr_config in self.attributes{
            let loc = gl.get_attrib_location(&shader_program, attr_config.name.as_str());
            if loc == -1{
                log_warn!("Attribute '{}' doesn't exist or was optimized out, Skipping.", attr_config.name);
                continue;
            }

            let buffer = gl.create_buffer()
                .ok_or(format!("Failed to create buffer for attribute '{}'", attr_config.name))?;
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
            gl.buffer_data_with_opt_array_buffer(
              WebGl2RenderingContext::ARRAY_BUFFER,
                Some(&attr_config.data),
                WebGl2RenderingContext::STATIC_DRAW
            );
            gl.vertex_attrib_pointer_with_i32(
                loc as u32,
                attr_config.size as i32,
                attr_config.size_type,
                attr_config.normalized,
                attr_config.stride as i32,
                attr_config.offset as i32
            );
            gl.vertex_attrib_divisor(loc as u32, attr_config.divisor);
            gl.enable_vertex_attrib_array(loc as u32);

            attributes.insert(attr_config.name, Attribute{
                buffer,
            });

        }

        let mut uniforms: Vec<Uniform> = Vec::new();
        for uniform_config in self.uniforms{
            let loc_res =
                gl.get_uniform_location(&shader_program,uniform_config.name.as_str())
                .ok_or(format!("Failed to find uniform '{}'", uniform_config.name));

            if loc_res.is_err(){
                log_warn!("Uniform '{}' doesn't exist or was optimized out, Skipping.", uniform_config.name);
                continue;
            }
            let loc = loc_res?;

            uniforms.push(Uniform{
                name: uniform_config.name,
                location: loc,
                bind_function: uniform_config.bind_function
            });
        }

        let mut texture_unit = 0u32;
        let mut textures: Vec<TextureInstance> = Vec::new();
        for texture_config in self.textures_loaded{
            let loc =
                gl.get_uniform_location(&shader_program,texture_config.name.as_str())
                .ok_or(format!("Failed to find texture '{}'", texture_config.name))?;
            let unit = texture_unit;
            texture_unit += 1;
            let texture_enum = WebGl2RenderingContext::TEXTURE0 + unit;
            textures.push(TextureInstance{
                name: texture_config.name,
                texture: texture_config.texture,
                location: loc,
                unit: texture_enum,
            })
        }

        gl.use_program(None);
        gl.bind_vertex_array(None);

        Ok(RenderPass{
            ctx: gl,
            shader_program,
            draw_mode: self.draw_mode,
            draw_count: self.draw_count,
            draw_type: self.draw_type,
            draw_offset: self.draw_offset,
            vao,
            index_buffer,
            attributes,
            uniforms,
            textures,
            index_buffer_data: self.index_buffer_data
        })
    }
}

impl RenderPass{

    pub fn draw(&self){
        self.draw_instanced(0);
    }

    pub fn draw_instanced(&self, instances: i32){
        let gl = &self.ctx;

        gl.use_program(Some(&self.shader_program));
        gl.bind_vertex_array(Some(&self.vao));

        for uniform in &self.uniforms{
            (uniform.bind_function)(gl, &uniform.location);
        }

        for texture in &self.textures{
            gl.uniform1i(Some(&texture.location), texture.unit as i32);
            gl.active_texture(texture.unit);
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture.texture));
        }

        if instances == 0{
            gl.draw_elements_with_i32(self.draw_mode,
                                      self.draw_count,
                                      self.draw_type,
                                      self.draw_offset);
        }else{
            gl.draw_elements_instanced_with_i32(self.draw_mode,
                                                self.draw_count,
                                                self.draw_type,
                                                self.draw_offset,
                                                instances);
        }

        gl.use_program(None);
        gl.bind_vertex_array(None);
    }
}