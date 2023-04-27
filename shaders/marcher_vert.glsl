#version 300 es
precision mediump float;

uniform mat4 invProjMat;
uniform mat4 invViewMat;

in vec2 vertPos;
out vec3 rayPosFrag;
out vec3 rayDirFrag;

out vec2 uv;

void main () {
    vec2 tempUV = (vertPos + 1.0) * 0.5;
    uv = tempUV;

    vec4 rayPosWorld = invViewMat * vec4(0.0, 0.0, 0.0, 1.0);
    rayPosFrag = rayPosWorld.xyz;

    vec4 rayDirCam = invProjMat * vec4(vertPos.x, vertPos.y, -1.0, 1.0);
    rayDirCam *= 1.0/rayDirCam.w;
    rayDirCam.w = 0.0;
    vec3 rayDirWorld = normalize((invViewMat * rayDirCam).xyz);
    rayDirFrag = rayDirWorld;




    gl_Position = vec4(vertPos.x, vertPos.y, -1.0, 1.0);
}
