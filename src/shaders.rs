

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

uniform mat4 invProjMat;
uniform mat4 viewProjMat;


in vec2 uv;
in vec3 rayPosFrag;
in vec3 rayDirFrag;

out vec4 fragColor;

const vec3 BULB_POS = vec3(11.0, 0.0, 11.0);
const float BULB_SCALE = 10.0;
const float THRESH = 0.003;
const float FAR_PLANE = 1000.0;
const vec3 LIGHT_DIR = normalize(vec3(-1, 1, -1));

float rand(vec2 p){
        p  = 50.0*fract( p*0.3183099  + vec2(0.71,0.113));
        return fract( p.x*p.y*(p.x+p.y) );
}

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

vec2 intersectCube(vec3 rayPos, vec3 rayDir, float size, out vec3 normal){
    vec3 m = 1.0/rayDir;
    vec3 n = m * rayPos;
    vec3 k = abs(m)*vec3(size);
    vec3 t1 = -n -k;
    vec3 t2 = -n + k;
    float tN = max( max( t1.x, t1.y ), t1.z );
    float tF = min( min( t2.x, t2.y ), t2.z );
    if( tN>tF || tF<0.0) return vec2(-1.0); // no intersection

    normal = (tN>0.0) ? step(vec3(tN),t1) : // ro ouside the box
                       step(t2,vec3(tF));  // ro inside the box
    normal *= -sign(rayDir);
    return vec2( tN, tF );
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
		if( m > 2560.0 )
            break;
	}
	return BULB_SCALE * 0.25*log(m)*sqrt(m)/dz;
}

float cubeSDF(vec3 pos){
    vec3 b = vec3(1.0);
    vec3 q = abs(pos) - b;
    return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}

float sdCross(vec3 pos){
  float da = cubeSDF(vec3(pos.xy, 0.0));
  float db = cubeSDF(vec3(pos.yz, 0.0));
  float dc = cubeSDF(vec3(pos.zx, 0.0));
  return min(da,min(db,dc));
}

const int MENGER_ITER = 8;
float mengerSpongeSdf(vec3 pos, out vec3 col){
    pos = pos - BULB_POS;
    pos /= BULB_SCALE;
    float dist = calcBulbDist(pos);

    col = vec3(dist, 1.0, dist);

    float scale = 1.0;
    for(int i=0; i<MENGER_ITER; ++i){
        vec3 posScaled = mod(pos*scale, 2.0) - 1.0;
        scale *= 3.0;
        vec3 posScaledTranslated = 1.0 - 3.0*abs(posScaled);


        float crossDist = sdCross(posScaledTranslated)/scale;
        if(crossDist > dist){
            col *= 0.75;
        }
        dist = max(dist, crossDist);

    }


    return BULB_SCALE * dist;
}

vec3 mengerNormal(vec3 pos){
    const float epsilon = 0.0001;
    const vec2 delta = vec2(1, -1);
    vec3 col;
    return normalize(vec3(
        delta.xyy * mengerSpongeSdf(pos + delta.xyy * epsilon, col) +
        delta.yyx * mengerSpongeSdf(pos + delta.yyx * epsilon, col) +
        delta.yxy * mengerSpongeSdf(pos + delta.yxy * epsilon, col) +
        delta.xxx * mengerSpongeSdf(pos + delta.xxx * epsilon, col)
        ));
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



float shadow(vec3 rayPos, vec3 rayDir, float minT, float maxT, float k){
    float t = minT;
    float res = 1.0;
    float prevDist = 0.00000000000000000001;
    vec3 col;
    for(int i=0; i<25 && t<maxT; ++i){
        vec3 pos = rayPos + t * rayDir;
        float dist = mengerSpongeSdf(pos, col);
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

    vec2 boundingSphereDistance = intersectSphere(rayPos, rayDir, BULB_POS, BULB_SCALE*1.25);
//    if(boundingSphereDistance.y < 0.0) return -1.0;
    boundingSphereDistance.x = max(boundingSphereDistance.x, 0.0);

    float t = 0.0;
    float dist;
    float th;
    for(int i=0; i<150; ++i){
        vec3 pos = rayPos + t * rayDir;
        dist = mengerSpongeSdf(pos, col);
        th =  t * THRESH * (rand(rayDir.xy)*0.3+0.7);
        if(dist < th || dist > 500.0) break;
        t += dist;
    }


    if(dist< th){
        return t;
    }else{
        return -1.0;
    }
}

const float BG = 0.37254903;
void main () {
    vec3 rayDir = normalize(rayDirFrag);
    vec3 rayPos = rayPosFrag;
    vec3 col;
    float dist = rayMarch(rayPos, rayDir, col);
    vec3 finalRayPos = rayPos + rayDir * dist;
//    vec3 lightDir = normalize(LIGHT_POS - finalRayPos);
    vec3 normal = mengerNormal(finalRayPos);
    float shadowFactor = shadow(finalRayPos + normal * (0.01 + rand(vec2(dist)) * 0.02), LIGHT_DIR, 0.001, 500.0, 0.5);

//    fragColor = vec4(float(rayDir.x > 0.0), float(rayDir.y > 0.0), float(rayDir.z > 0.0), 1.0);

    if(dist < 0.0){
        gl_FragDepth = 1.0;
        fragColor = vec4(
            mix(vec3(0.0, BG, BG), vec3(1.0, 1.0, 0.90), smoothstep( 0.999, 1.0, dot(rayDir, LIGHT_DIR))), 1.0);
//            clamp(), 0.0, 1.0-BG), 1.0);
    }else{
        vec4 projCoords = viewProjMat * vec4(finalRayPos, 1.0);
        gl_FragDepth = ((projCoords.z / projCoords.w) + 1.0) * 0.5;

        fragColor = vec4(vec3(1.0, 1.0, 1.0)
        * clamp(dot(normal, normalize(LIGHT_DIR)), 0.01, 1.0)
        * shadowFactor
        , 1.0);
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


pub static CLOUD_FRAG_SHADER: &str = r#"#version 300 es
precision mediump float;

uniform mat4 invProjMat;
uniform mat4 viewProjMat;

in vec2 uv;
in vec3 rayPosFrag;
in vec3 rayDirFrag;

out vec4 fragColor;

const mat3 ROT_MAT = mat3(
    vec3(0.0896239459514618, 0.7085132598876953, -0.6435269713401794),
    vec3(-0.28627997636795044, 0.7056889533996582, 0.6481102705001831),
    vec3(0.9133245348930359, -0.0034793566446751356, 0.4072175621986389)
);
const vec3 LIGHT_DIR = normalize(vec3(-1, 1, -1));


float hash(vec2 p){
        p  = 50.0*fract( p*0.3183099  + vec2(0.71,0.113));
        return -1.0+2.0*fract( p.x*p.y*(p.x+p.y) );
}

float rand3D(vec3 co){
    return hash(vec2(hash(co.xy), hash(co.yz)));
}

vec3 unit_vec(in vec3 xyz) {
//        float theta = 6.28318530718*rand3D(xyz);
        float rand1 = hash(xyz.xy);
        float rand2 = hash(vec2(rand1, xyz.z));
        float rand3 = hash(vec2(rand2, rand1));
        return normalize(vec3(rand1, rand2, rand3));
}

float smoothmix(float a0, float a1, float w) {
        return (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0;
}

const float PERIODS[] = float[](100.0, 50.0, 20.0, 5.0);
const float WEIGHTS[] = float[](1.0,0.5,0.05,0.00135);

float perlin3D(vec3 pos){

    float val = 0.0;
    for(int i=0; i<1; ++i){
        float weight = WEIGHTS[i];
        float freq = 1.0/PERIODS[i];

        vec3 noisePos = pos * freq;
        vec3 floorPos = floor(noisePos);
        vec3 fracPos = fract(noisePos);

        vec2 sampleVec = vec2(0.0, 1.0);

        float p1 = dot(unit_vec(floorPos), noisePos - floorPos);
        float p2 = dot(unit_vec(floorPos + sampleVec.xxy), noisePos - sampleVec.xxy - floorPos);
        float p3 = dot(unit_vec(floorPos + sampleVec.xyx), noisePos - sampleVec.xyx - floorPos);
        float p4 = dot(unit_vec(floorPos + sampleVec.xyy), noisePos - sampleVec.xyy - floorPos);

        float p5 = dot(unit_vec(floorPos + sampleVec.yxx), noisePos - sampleVec.yxx - floorPos);
        float p6 = dot(unit_vec(floorPos + sampleVec.yxy), noisePos - sampleVec.yxy - floorPos);
        float p7 = dot(unit_vec(floorPos + sampleVec.yyx), noisePos - sampleVec.yyx - floorPos);
        float p8 = dot(unit_vec(floorPos + sampleVec.yyy), noisePos - sampleVec.yyy - floorPos);

        float s1 = smoothmix(p1, p5, fracPos.x);
        float s2 = smoothmix(p2, p6, fracPos.x);
        float s3 = smoothmix(p3, p7, fracPos.x);
        float s4 = smoothmix(p4, p8, fracPos.x);

        float s5 = smoothmix(s1, s3, fracPos.y);
        float s6 = smoothmix(s2, s4, fracPos.y);


        //val += weight * p2;
        val += weight * smoothmix(s5, s6, fracPos.z);
    }
    return val + 0.8;
}

float gridSphereSDF(vec3 floorPos, vec3 fracPos, vec3 off){
    float radius = 0.5 * rand3D(floorPos + off);
    return length(fracPos - off) - radius;
}

float sphereNoiseSDF(vec3 pos){
    vec3 small_pos = pos;// / 50.0;
    vec3 floorPos = floor(small_pos);
    vec3 fracPos = fract(small_pos);
    return min(
        min(
            min(
                gridSphereSDF(floorPos, fracPos, vec3(0.0,0.0,0.0)),
                gridSphereSDF(floorPos, fracPos, vec3(0.0,0.0,1.0))
            ),
            min(
                gridSphereSDF(floorPos, fracPos, vec3(0.0,1.0,0.0)),
                gridSphereSDF(floorPos, fracPos, vec3(0.0,1.0,1.0))
            )
        ),
        min(
            min(
                gridSphereSDF(floorPos, fracPos, vec3(1.0,0.0,0.0)),
                gridSphereSDF(floorPos, fracPos, vec3(1.0,0.0,1.0))
            ),
            min(
                gridSphereSDF(floorPos, fracPos, vec3(1.0,1.0,0.0)),
                gridSphereSDF(floorPos, fracPos, vec3(1.0,1.0,1.0))
            )
        )
    );
}

float sphereFractalNoiseSDF(vec3 pos){
//    float val = sphereNoiseSDF(pos / PERIODS[0]) * PERIODS[0];
    float val = 0.0;
    vec3 noisePos = pos;
    for(int i=0; i<3; ++i){
        float weight = WEIGHTS[i];
//        float freq = 1.0/PERIODS[i];

        noisePos = ROT_MAT * noisePos;

        //val += weight * p2;
        val += weight * sphereNoiseSDF(noisePos / PERIODS[i]) * PERIODS[i];
    }
    return val;
}


const float THRESH = 0.01;

const float ITERATIONS = 20.0;
const float TO_ADD = 1.0/20.0;
const float STEP = 0.1;
float volumeFactor(vec3 rayPos, vec3 rayDir){
    float count = 0.0;
    float t = 0.0;
    for(int i=0; i<20; ++i){
        vec3 pos = rayPos + t * rayDir;
        float dist = sphereFractalNoiseSDF(pos);
        if(dist < THRESH) count += TO_ADD;
        t += STEP;
    }
    return count;
}


float cloudMarch(vec3 rayPos, vec3 rayDir){


    float t = 0.0;
    float dist;
    float th;
    for(int i=0; i<150; ++i){
        vec3 pos = rayPos + t * rayDir;
        dist = sphereFractalNoiseSDF(pos);
        th = t * THRESH * (hash(vec2(dist, rayDir.x))*0.20+0.80);
        if(dist < th || dist > 500.0) break;
        t += dist;
    }


    if(dist< th){
        return t; //volumeFactor(rayPos + t*rayDir, rayDir);
    }else{
        return -1.0;
    }
}

vec3 cloudNormal(vec3 pos){
    const float epsilon = 0.0001;
    const vec2 delta = vec2(1.0, -1.0);
    return normalize(vec3(
        delta.xyy * sphereFractalNoiseSDF(pos + delta.xyy * epsilon) +
        delta.yyx * sphereFractalNoiseSDF(pos + delta.yyx * epsilon) +
        delta.yxy * sphereFractalNoiseSDF(pos + delta.yxy * epsilon) +
        delta.xxx * sphereFractalNoiseSDF(pos + delta.xxx * epsilon)
        ));
}

void main () {
    vec3 rayDir = normalize(rayDirFrag);
    vec3 rayPos = rayPosFrag;

//    vec3 zeroed = vec3(rayPos.xy, 0.0);
    float dist = cloudMarch(rayPos, rayDir);
    vec3 finalRayPos = rayPos + rayDir * dist;

    if(dist < 0.0){
        gl_FragDepth = 1.0;
        fragColor = vec4(0.0, 0.0, 0.0, 1.0);
//            clamp(), 0.0, 1.0-BG), 1.0);
    }else{
        vec4 projCoords = viewProjMat * vec4(finalRayPos, 1.0);
        gl_FragDepth = ((projCoords.z / projCoords.w) + 1.0) * 0.5;
        vec3 normal = cloudNormal(finalRayPos);
        fragColor = vec4(vec3(1.0, 1.0, 1.0)
        * clamp(dot(normal, normalize(LIGHT_DIR)), 0.01, 1.0)
        , volumeFactor(finalRayPos, rayDir));
    }
//    float noise = perlin3D(ROT_MAT*rayPos + rayDir*10.0);
    //perlin3D(zeroed + 100.0 * vec3(uv, 0.0));
//    float noise3d = rand3D(rayPos.xyz + 100.0 * rayDir);
//    vec3 vec = unit_vec(rayPos + 100.0 * rayDir);
//    fragColor = vec4(vec3(cloud), 1.0);
}
"#;