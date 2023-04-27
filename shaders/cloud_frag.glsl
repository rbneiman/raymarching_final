#version 300 es
precision mediump float;

uniform sampler2D colorTex;

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


const float PERLIN_PERIODS[] = float[](2.0, 0.5, 0.2, 0.05);
const float PERLIN_WEIGHTS[] = float[](0.5,0.25,0.135,0.725);
float perlin3D(vec3 pos){

    vec3 rotPos = ROT_MAT * pos;
    float val = 0.0;
    for(int i=0; i<1; ++i){
        float weight = PERLIN_WEIGHTS[i];
        float freq = 1.0/PERLIN_PERIODS[i];

        vec3 noisePos = rotPos * freq;
        rotPos = ROT_MAT * rotPos;
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
    return val + 0.1;
}

float gridSphereSDF(vec3 floorPos, vec3 fracPos, vec3 off){
    float radius = 0.5 * (rand3D(floorPos + off) + 1.0) * 0.5;
    return length(fracPos - off) - radius;
}

float sphereNoiseSDF(vec3 pos){
    vec3 small_pos = pos / 50.0;
    vec3 floorPos = floor(small_pos);
    vec3 fracPos = fract(small_pos);
    return 50.0 * min(
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
    for(int i=0; i<1; ++i){
        float weight = WEIGHTS[i];
//        float freq = 1.0/PERIODS[i];

//        noisePos = ROT_MAT * noisePos;

        //val += weight * p2;
        val += weight * sphereNoiseSDF(noisePos / PERIODS[i]) * PERIODS[i];
    }
    return val;
}


const float THRESH = 0.00001;

const int ITERATIONS = 40;
const float TO_ADD = 1.0/float(ITERATIONS);
const float STEP = 1.0;
// 0 clear, 1 opaque
float volumeFactor(vec3 rayPos, vec3 rayDir, float t){
    float count = 0.0;
//    t += 1.0;
//    float t = 0.0;
    for(int i=0; i<ITERATIONS; ++i){
        vec3 pos = rayPos + t * rayDir;
        float dist = sphereNoiseSDF(pos);
        float density = clamp(perlin3D(pos) * 2.0, 0.0, 1.0);
//        float th = t * THRESH; // * (hash(vec2(dist, dist))*0.20+0.80);
        if(dist < 0.0001) count += density * TO_ADD;

        t += STEP;
    }
    return exp(-count * 4.0);
}


float cloudMarch(vec3 rayPos, vec3 rayDir){


    float t = 0.0;
    float dist;
    float th;
    for(int i=0; i<150; ++i){
        vec3 pos = rayPos + t * rayDir;
        dist = sphereNoiseSDF(pos);
        th =  THRESH;//* (hash(vec2(dist, rayDir.x))*0.20+0.80);
        if(dist < th || dist > 500.0) break;
//        if(dist < th){
//            for(int j=0; j<3; ++j){
//
//            }
//            break;
//        }
        t += dist;
    }

//    t += dist;
    if(dist< th){
        return t; //volumeFactor(rayPos + t*rayDir, rayDir);
    }else{
        return -1.0;
    }
}

vec3 cloudNormal(vec3 pos){
    const float epsilon = 0.001;
    const vec2 delta = vec2(1.0, -1.0);
    return normalize(vec3(
        delta.xyy * sphereNoiseSDF(pos + delta.xyy * epsilon) +
        delta.yyx * sphereNoiseSDF(pos + delta.yyx * epsilon) +
        delta.yxy * sphereNoiseSDF(pos + delta.yxy * epsilon) +
        delta.xxx * sphereNoiseSDF(pos + delta.xxx * epsilon)
        ));
}

void main () {
    vec3 rayDir = normalize(rayDirFrag);
    vec3 rayPos = rayPosFrag + rayDir*0.0001;

//    vec3 zeroed = vec3(rayPos.xy, 0.0);
    float t = cloudMarch(rayPos, rayDir);
    vec3 finalRayPos = rayPos + rayDir * t;

    vec3 prevPass = texture(colorTex, uv).rgb;
    float prevDepth = texture(colorTex, uv).a;

    vec4 projCoords = viewProjMat * vec4(finalRayPos, 1.0);
    float depth = ((projCoords.z / projCoords.w) + 1.0) * 0.5;

    if(depth > prevDepth || t == -1.0){
        gl_FragDepth = prevDepth;
        fragColor = vec4(prevPass, 1.0);
//            clamp(), 0.0, 1.0-BG), 1.0);
    }else{

        vec3 normal = cloudNormal(finalRayPos);

        float volumeFactor = clamp(volumeFactor(rayPos, rayDir, t), 0.0, 1.0);

        float finalDist = sphereNoiseSDF(finalRayPos);

        vec3 cloudCol = vec3(1.0, 1.0, 1.0);// * volumeFactor;
        //* clamp(dot(normal, normalize(LIGHT_DIR)), 0.01, 1.0);
            //vec3(0.0, 0.37254903, 0.37254903)
        vec3 col = mix(cloudCol, prevPass, volumeFactor);
        fragColor = vec4(col, 1.0);
    }
}