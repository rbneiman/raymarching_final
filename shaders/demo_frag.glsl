#version 300 es
precision mediump float;

uniform mat4 invProjMat;
uniform mat4 viewProjMat;
uniform float time;

in vec2 uv;
in vec3 rayPosFrag;
in vec3 rayDirFrag;

out vec4 fragColor;

const vec3 LIGHT_DIR = normalize(vec3(-1, 1, -1));
const float THRESH = 0.001;






float sceneSDF(vec3 pos, out vec3 col){

}

float rayMarch(vec3 rayPos, vec3 rayDir, out vec3 col){
    vec3 col = vec3(1.0);
    float t = 0.0;
    float dist;
    float th;
    for(int i=0; i<150; ++i){
        vec3 pos = rayPos + t * rayDir;
        dist = sceneSDF(pos, col);
        th =  t * THRESH; // * (rand(vec2(t, rayPos.x))*0.2+0.8);
        if(dist < th || dist > 500.0) break;
        t += dist;
    }

    if(dist< th){
        return t;
    }else{
        return -1.0;
    }
}

vec3 sceneNormal(vec3 pos){
    vec3 col;
    const float epsilon = 0.001;
    const vec2 delta = vec2(1.0, -1.0);
    return normalize(vec3(
        delta.xyy * sceneSDF(pos + delta.xyy * epsilon, col) +
        delta.yyx * sceneSDF(pos + delta.yyx * epsilon, col) +
        delta.yxy * sceneSDF(pos + delta.yxy * epsilon, col) +
        delta.xxx * sceneSDF(pos + delta.xxx * epsilon, col)
        ));
}


void main () {
    vec3 rayDir = normalize(rayDirFrag);
    vec3 rayPos = rayPosFrag + rayDir * 0.0001;



}