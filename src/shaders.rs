

pub static VERT_SHADER: &str = r#"#version 300 es
// pycharm_glsl
precision mediump float;

in vec2 vertPos;

void main(){
    gl_Position = vec4(vertPos, 0.0, 1.0);
}
"#;

pub static FRAG_SHADER: &str = r#"#version 300 es
// pycharm_glsl
precision mediump float;

out vec4 fragColor;

void main(){
    fragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;