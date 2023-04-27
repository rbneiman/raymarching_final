#version 300 es
precision mediump float;

uniform mat4 mvp;

in vec3 vertPos;

void main(){
    gl_Position = mvp * vec4(vertPos, 1.0);
}