#version 300 es
precision mediump float;

uniform sampler2D colorTex;

uniform mat4 invProjMat;
uniform mat4 viewProjMat;
uniform float time;

in vec2 uv;
in vec3 rayPosFrag;
in vec3 rayDirFrag;

out vec4 fragColor;

const mat3 ROT_MAT = mat3(
    vec3(0.0896239459514618, 0.6085132598876953, -0.6435269713401794),
    vec3(-0.28627997636795044, 0.7056889533996582, 0.6281102705001831),
    vec3(0.9133245348930359, -0.0034793566446751356, 0.3572175621986389)
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
        float rand1 = hash(xyz.xy * xyz.z);
        float rand2 = hash(vec2(rand1, xyz.z));
        float rand3 = hash(vec2(rand1, rand2));
//        float rand2 = hash(vec2(rand1, xyz.z));
//        float rand3 = hash(vec2(rand2, rand1));
        return normalize(vec3(rand1, rand2, rand3));
}

float smoothmix(float a0, float a1, float w) {
        return (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0;
}

const float PERIODS[] = float[](100.0, 50.0, 20.0, 5.0);
const float WEIGHTS[] = float[](1.0,0.5,0.05,0.00135);



const float PERLIN_PERIODS[] = float[](80.0, 30.0, 2.0, 0.5);
const float PERLIN_WEIGHTS[] = float[](80.0,8.0,7.0,2.0);
const float PERLIN_OFFSETS[] = float[](-18.0,-1.8,0.0,0.0);


float perlin3D(vec3 pos){

    vec3 rotPos = ROT_MAT * (pos + vec3(10000.0)  + vec3(time ) );
    float val = 0.0;
    for(int i=0; i<4; ++i){
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
        val += weight * smoothmix(s5, s6, fracPos.z) + PERLIN_OFFSETS[i];
    }
    return val * 0.5 + 1.0;
}

const float THRESH = 0.001;

const int ITERATIONS = 150;
const float TO_ADD = 1.0/float(ITERATIONS);
const float STEP = 2.0;

float cloudMarch(vec3 rayPos, vec3 rayDir, float prevT, out float volumeFactor){

    float count = 0.0;
    float t = 0.0;
    float dist;
    float th;
    for(int i=0; i<ITERATIONS; ++i){
        vec3 pos = rayPos + t * rayDir;
        th =  THRESH * (hash(vec2(dist, rayDir.x))*0.20+0.80);

        float density = max(perlin3D(pos) - 0.6 + 0.1* sqrt(t), 0.0);
        count += density * TO_ADD;

        float densityFactor = clamp( (1.0 - density * 2.0 - hash(uv * time) * 0.2)
        , 0.0, 1.0) * hash(vec2(dist * time));


        t += (STEP * t * sqrt(t) )/1000.0
        + STEP * t / 40.0
        + 2.0 * STEP
        - 2.5 * STEP * densityFactor;
        if(count > 0.3) break;
        if(t * (count + 1.2 - hash(uv * time) * 0.4) > prevT) break;
    }

    volumeFactor = clamp(exp(-count * 20.0) * 2.0, 0.0, 1.0); // * (1.0-exp(-count * 2.0));
    return t;
}

vec3 cloudNormal(vec3 pos){
    const float epsilon = 0.001;
    const vec2 delta = vec2(1.0, -1.0);
    return normalize(vec3(
        delta.xyy * perlin3D(pos + delta.xyy * epsilon) +
        delta.yyx * perlin3D(pos + delta.yyx * epsilon) +
        delta.yxy * perlin3D(pos + delta.yxy * epsilon) +
        delta.xxx * perlin3D(pos + delta.xxx * epsilon)
        ));
}

void main () {
    vec3 rayDir = normalize(rayDirFrag);
    vec3 rayPos = rayPosFrag + rayDir*0.0001;

    vec3 prevPass = texture(colorTex, uv).rgb;
    float prevT = texture(colorTex, uv).a;
    if(prevT < 0.0){
        prevT = 700.0;
    }

    float volumeFactor;
    float t = cloudMarch(rayPos, rayDir, prevT, volumeFactor);
    vec3 finalRayPos = rayPos + rayDir * t;

    vec4 projCoords = viewProjMat * vec4(finalRayPos, 1.0);
    vec4 projCoordsPrev = viewProjMat * vec4(rayPos + prevT * rayDir, 1.0);
    float depth = ((projCoords.z / projCoords.w) + 1.0) * 0.5;
    float prevDepth = ((projCoordsPrev.z / projCoordsPrev.w) + 1.0) * 0.5;


    gl_FragDepth = -0.999;
    vec3 normal = cloudNormal(finalRayPos);
    vec3 cloudCol = vec3(0.8, 0.8, 0.8) * volumeFactor;
//    vec3 col = mix(cloudCol, prevPass, volumeFactor);
    fragColor = vec4(cloudCol * prevPass, 1.0);

}