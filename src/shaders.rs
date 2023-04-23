

pub static VERT_SHADER: &str = r#"#version 300 es
// pycharm_glsl
precision mediump float;

uniform mat4 mvp;

in vec3 vertPos;

void main(){
    gl_Position = mvp * vec4(vertPos, 1.0);
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

pub static PIXEL_VERT_SHADER: &str = r#"#version 300 es
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

"#;

pub static FRACTAL_FRAG_SHADER: &str = r#"#version 300 es
precision mediump float;

in vec2 uv;
in vec3 rayPosFrag;
in vec3 rayDirFrag;

out vec4 fragColor;

const vec3 BULB_POS = vec3(100.0, 0.0, 100.0);
const float BULB_SCALE = 100.0;
const float THRESH = 0.0003;

// https://iquilezles.org/articles/intersectors/
vec2 intersectSphere(vec3 rayPos, vec3 rayDir, vec3 spherePos, float sphereSize){
    vec3 oc = rayPos - spherePos;
    float b = dot(oc, rayDir);
    vec3 qc = oc - b * rayDir;
    float h = sphereSize * sphereSize - dot(qc, qc);
    if(h<0.0) return vec2(-1.0); // no intersections
    h = sqrt(h);
    return vec2(-b-h, -b+h);
}

// https://www.shadertoy.com/view/ltfSWn
const int ITERATIONS = 4;
const float POWER = 3.5;
float calcBulbDist(vec3 pos){
    pos = pos - BULB_POS;
    pos /= BULB_SCALE;
    vec3 w = pos;
    float m = dot(w,w);
    float dz = 1.0;
	for (int i = 0; i < ITERATIONS ; i++) {
        // dz = 8*z^7*dz
		dz = 8.0*pow(m,POWER)*dz + 1.0;

        // z = z^8+c
        float r = length(w);
        float b = 8.0*acos( w.y/r);
        float a = 8.0*atan( w.x, w.z );
        w = pos + pow(r,8.0) * vec3( sin(b)*sin(a), cos(b), sin(b)*cos(a) );
        m = dot(w,w);
		if( m > 25600.0 )
            break;
	}
	return BULB_SCALE * 0.25*log(m)*sqrt(m)/dz;
}

// calculates the normal of the bulb sdf at the given point by finding the gradient
vec3 bulbNormal(vec3 pos){
    vec2 delta = vec2(0.0001, 0.0);
    return normalize(vec3(
        calcBulbDist(pos + delta.xyy) - calcBulbDist(pos - delta.xyy),
        calcBulbDist(pos + delta.yxy) - calcBulbDist(pos - delta.yxy),
        calcBulbDist(pos + delta.yyx) - calcBulbDist(pos - delta.yyx)
        ));
}

float rayMarch(vec3 rayPos, vec3 rayDir){

    vec2 boundingSphereDistance = intersectSphere(rayPos, rayDir, BULB_POS, BULB_SCALE*1.25);
    if(boundingSphereDistance.y < 0.0) return -1.0;
    boundingSphereDistance.x = max(boundingSphereDistance.x, 0.0);

    float t = boundingSphereDistance.x;
    float dist;
    for(int i=0; i<150; ++i){
        vec3 pos = rayPos + t * rayDir;
        dist = calcBulbDist(pos);
        float th = 0.25 * t * THRESH;
        if(t > boundingSphereDistance.y || dist < THRESH) break;
        t += dist;
    }

    if(t < boundingSphereDistance.y){
        return t;
    }else{
        return -1.0;
    }
}

const vec3 LIGHT = vec3(-15.0, -15.0, -15.0);
void main () {
    vec3 rayDir = normalize(rayDirFrag);
    vec3 rayPos = rayPosFrag;
    float dist = rayMarch(rayPos, rayDir);

//    fragColor = vec4(float(rayDir.x > 0.0), float(rayDir.y > 0.0), float(rayDir.z > 0.0), 1.0);
    vec3 finalRayPos = rayPos + rayDir * dist;
    if(dist < 0.0){
        fragColor = vec4(0.0, 1.0, 0.0, 1.0);
    }else{
        vec3 normal = bulbNormal(finalRayPos);
        fragColor = vec4(vec3(0.0, 0.0, 1.0)
        * clamp(dot(normal, normalize(LIGHT - finalRayPos)), 0.01, 1.0), 1.0);
    }

//    float clamped = clamp(dist, 0.0, THRESH);
//    float factor = 1.0 - (clamped / THRESH);
//    factor = factor * factor;
//    factor = factor * factor;
//    factor = factor * factor;
//    factor = factor * factor;
//    vec3 col = vec3(0.0, 0.0, 1.0) * factor * float(dist<THRESH);
//    fragColor = vec4(col, 1.0);
}

"#;