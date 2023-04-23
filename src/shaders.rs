

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
out vec3 rayPosIn;
out vec3 rayDir;

out vec2 uv;

void main () {
    vec2 tempUV = (vertPos + 1.0) * 0.5;
    uv = tempUV;

    vec4 rayDirCam = invProjMat * vec4(vertPos.x, vertPos.y, -1.0, 1.0);
    rayDirCam *= 1.0/rayDirCam.w;
    rayDirCam.w = 0.0;
    vec3 rayDirWorld = normalize((invViewMat * rayDirCam).xyz);
    rayDir = rayDirWorld;

    vec4 rayPosWorld = invViewMat * vec4(0.0, 0.0, 0.0, 1.0);
    rayPosIn = rayPosWorld.xyz;


    gl_Position = vec4(vertPos.x, vertPos.y, -1.0, 1.0);
}

"#;

pub static FRACTAL_FRAG_SHADER: &str = r#"#version 300 es
precision mediump float;

in vec2 uv;
in vec3 rayPosIn;
in vec3 rayDir;

out vec4 fragColor;

const vec3 SPHERE_POS = vec3(10.0, 0.0, 10.0);

float calcSphereDist(vec3 pos, float s){
    return length(pos) - s;
}

const int ITERATIONS = 20;
const float POWER = 2.0;
float calcBulbDist(vec3 pos){
    pos /= 10.0;
    vec3 z = pos;
	float dr = 1.0;
	float r = 0.0;
	for (int i = 0; i < ITERATIONS ; i++) {
		r = length(z);
		if (r>5.0) break;

		// convert to polar coordinates
		float theta = acos(z.z/r);
		float phi = atan(z.y,z.x);
		dr =  pow( r, POWER-1.0)*POWER*dr + 1.0;

		// scale and rotate the point
		float zr = pow( r,POWER);
		theta = theta*POWER;
		phi = phi*POWER;

		// convert back to cartesian coordinates
		z = zr*vec3(sin(theta)*cos(phi), sin(phi)*sin(theta), cos(theta));
		z+=pos;
	}
	return 0.5*log(r)*r/dr;
}

const float THRESH = 0.2;
void main () {
//    vec3 rayPos = (invViewMat * vec4(0.0, 0.0, 0.0, 1.0)).xyawz;
//    vec3 rayDir = normalize((invViewMat * vec4(uv, -1.0, 0.0)).xyz);
    vec3 rayPos = rayPosIn;
    for(int i=0; i<200; ++i){
        float dist = calcBulbDist(rayPos - SPHERE_POS);
        if(dist < 0.0) break;
        rayPos += dist * rayDir;
    }

    float dist = calcBulbDist(rayPos - SPHERE_POS);
    float clamped = clamp(dist, 0.0, THRESH);
    float factor = 1.0 - (clamped / THRESH);
    factor = factor * factor;
    factor = factor * factor;
    factor = factor * factor;
    factor = factor * factor;
    vec3 col = vec3(0.0, 0.0, 1.0) * factor * float(dist<THRESH);
    fragColor = vec4(col, 1.0);
}

"#;