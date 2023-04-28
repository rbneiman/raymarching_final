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
const float THRESH = 0.0001;

float plane(vec3 pos){
    return pos.y;
}

float sphereSDF(vec3 pos, float size){
    return length(pos) - size;
}

float boxSDF(vec3 pos, vec3 dim){
    vec3 q = abs(pos) - dim;
    return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}

float torus(vec3 pos, vec2 dim){
    vec2 q = vec2(length(pos.xz)-dim.x,pos.y);
    return length(q)-dim.y;
}

float link(vec3 pos, float le, float r1, float r2){
    vec3 q = vec3( pos.x, max(abs(pos.y)-le,0.0), pos.z );
    return length(vec2(length(q.xy)-r1,q.z)) - r2;
}

vec2 colorCheck(vec2 shape1, vec2 shape2){
    return (shape1.x < shape2.x) ? shape1 : shape2;
}

vec2 sceneSDF(vec3 pos){
    vec2 res = vec2(pos.y, 0.0);

    res = colorCheck(res, vec2(sphereSDF(pos - vec3(1.0, 1.0, 1.0), 1.0), 23.0));

    return res;
}

float shadow(vec3 rayPos, vec3 rayDir, float minT, float maxT, float k){
    float t = minT;
    float res = 1.0;
    float prevDist = 0.00000000000000000001;
    vec3 col;
    for(int i=0; i<25 && t<maxT; ++i){
        vec3 pos = rayPos + t * rayDir;

        float dist = sceneSDF(pos).x;
        if(dist < minT * 10.0 * t) return 0.0;

        float y = (dist*dist) /(2.0 * prevDist);
        float d = sqrt(dist*dist - y*y);
        res = min(res, d / (k*max(0.0, t-y)));

        prevDist = dist;
        t += dist;
    }
    return res;
}

float rayMarch(vec3 rayPos, vec3 rayDir, out vec3 col){
    float t = 0.0;
    float dist;
    float th;
    float resy;
    for(int i=0; i<150; ++i){
        vec3 pos = rayPos + t * rayDir;

        vec2 res = sceneSDF(pos);
        float dist = res.x;
        resy = res.y;

        th =  t * THRESH; // * (rand(vec2(t, rayPos.x))*0.2+0.8);
        if(dist < th || dist > 200.0) break;
        t += dist;
    }
    col = 0.2 + 0.2*sin( resy*2.0 + vec3(0.0,1.0,2.0) );
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
        delta.xyy * sceneSDF(pos + delta.xyy * epsilon).x +
        delta.yyx * sceneSDF(pos + delta.yyx * epsilon).x +
        delta.yxy * sceneSDF(pos + delta.yxy * epsilon).x +
        delta.xxx * sceneSDF(pos + delta.xxx * epsilon).x
        ));
}

const vec3 BG = vec3(0.0, 0.37254903, 0.37254903);
const vec3 FOG = vec3(0.0, 0.37254903, 0.37254903);
void main () {
    vec3 rayDir = normalize(rayDirFrag);
    vec3 rayPos = rayPosFrag + rayDir * 0.0001;

    vec3 col;
    float dist = rayMarch(rayPos, rayDir, col);
    vec3 finalRayPos = rayPos + rayDir * dist;
    vec3 normal = sceneNormal(finalRayPos);
    float shadowFactor = shadow(finalRayPos + normal * 0.01, LIGHT_DIR, 0.001, 500.0, 0.3);



    if(dist > 200.0 || dist < 0.0){
        gl_FragDepth = 0.999999;
        fragColor = vec4(
            mix(BG, vec3(1.0, 1.0, 0.90), smoothstep( 0.999, 1.0, dot(rayDir, LIGHT_DIR))), 1.0);
    }else{
        vec4 projCoords = viewProjMat * vec4(finalRayPos, 1.0);
        float depth = ((projCoords.z / projCoords.w) + 1.0) * 0.5;
        gl_FragDepth = depth;

        col = col
        * clamp(dot(normal, normalize(LIGHT_DIR)), 0.01, 1.0)
        * shadowFactor;
        col = mix(col, FOG, smoothstep(0.6, 1.0, clamp(dist/500.0, 0.0, 1.0)));

        fragColor = vec4(col, 1.0);
    }
}