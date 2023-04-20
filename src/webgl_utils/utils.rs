use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use crate::log_error;

pub fn util_create_shader(ctx: &WebGl2RenderingContext, shader_type:u32, source: &str)
    -> Result<WebGlShader, String>{
    let shader : WebGlShader;
    match ctx.create_shader(shader_type) {
        Some(res) => {
            shader = res;
        },
        None => {
            return Err(String::from("Failed to create shader."));
        }
    };
    ctx.shader_source(&shader, source);
    ctx.compile_shader(&shader);

    let compile_status = ctx.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS);
    if compile_status.is_falsy() {
        return match ctx.get_shader_info_log(&shader) {
            Some(res) => {
                Err(format!("Error compiling shader:\n{}", res))
            },
            _ => {
                Err(String::from("Error compiling shader."))
            }
        };
    }

    Ok(shader)
}

pub fn util_create_program(ctx: &WebGl2RenderingContext, v_shader: &String, f_shader: &String)
    -> Result<WebGlProgram, String>{
    let shader_program: WebGlProgram = ctx.create_program()
        .ok_or("Context failed to create shader program.")?;

    let vertex_shader: WebGlShader =
        util_create_shader(ctx, WebGl2RenderingContext::VERTEX_SHADER, v_shader)?;
    ctx.attach_shader(&shader_program, &vertex_shader);

    let fragment_shader: WebGlShader =
        util_create_shader(ctx, WebGl2RenderingContext::FRAGMENT_SHADER, f_shader)?;
    ctx.attach_shader(&shader_program, &fragment_shader);

    ctx.link_program(&shader_program);

    let link_status = ctx.get_program_parameter(&shader_program, WebGl2RenderingContext::LINK_STATUS);
    if link_status.is_falsy() {
        return match ctx.get_program_info_log(&shader_program)  {
            Some(res) =>{
                Err(format!("Error linking program.\n{}", res))
            },
            _ => {
                Err(String::from("Error linking program."))
            }
        } ;
    }

    ctx.validate_program(&shader_program);
    let validate_status = ctx.get_program_parameter(&shader_program, WebGl2RenderingContext::VALIDATE_STATUS);
    if validate_status.is_falsy() {
        return match ctx.get_program_info_log(&shader_program)  {
            Some(res) =>{
                Err(format!("Error validating program.\n{}", res))
            },
            _ => {
                Err(String::from("Error validating program."))
            }
        };
    }

    Ok(shader_program)
}
