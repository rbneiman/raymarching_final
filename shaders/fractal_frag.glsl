#version 300 es
precision mediump float;

uniform mat4 invProjMat;
uniform mat4 viewProjMat;
uniform float time;

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
		if( m > 1200.0 )
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
        th =  t * THRESH * (rand(vec2(t, rayPos.x))*0.2+0.8);
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
    vec3 rayPos = rayPosFrag + rayDir*0.0001;
    vec3 col;
    float dist = rayMarch(rayPos, rayDir, col);
    vec3 finalRayPos = rayPos + rayDir * dist;
//    vec3 lightDir = normalize(LIGHT_POS - finalRayPos);
    vec3 normal = mengerNormal(finalRayPos);
    float shadowFactor = shadow(finalRayPos + normal * (0.01 + rand(vec2(uv * time)) * 0.02), LIGHT_DIR, 0.001, 500.0, 0.5);

//    fragColor = vec4(float(rayDir.x > 0.0), float(rayDir.y > 0.0), float(rayDir.z > 0.0), 1.0);

    if(dist < 0.0){
        gl_FragDepth = 0.999999;
//        fragColor = vec4(vec3(0.4, 0.4, 0.41), -1.0);
        fragColor = vec4(
            mix(vec3(0.4, 0.4, 0.41), vec3(1.0, 1.0, 0.90), smoothstep( 0.999, 1.0, dot(rayDir, LIGHT_DIR))), -1.0);
    }else{
        vec4 projCoords = viewProjMat * vec4(finalRayPos, 1.0);
        float depth = ((projCoords.z / projCoords.w) + 1.0) * 0.5;
        gl_FragDepth = depth;

        fragColor = vec4(vec3(1.0, 1.0, 1.0)
        * clamp(dot(normal, normalize(LIGHT_DIR)), 0.01, 1.0)
        * shadowFactor
        , dist);
    }
}
