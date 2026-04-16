/*{
    "DESCRIPTION": "Photorealistic sun rays with hexagonal bokeh and dust motes",
    "CREDIT": "Jonas",
    "ISFVSN": "2",
    "CATEGORIES": ["Stylize"],
    "INPUTS": [
        { "NAME": "inputImage", "TYPE": "image" },
        { "NAME": "blend", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.0 },
        { "NAME": "rayIntensity", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.6 },
        { "NAME": "rayCount", "TYPE": "float", "MIN": 2.0, "MAX": 30.0, "DEFAULT": 12.0 },
        { "NAME": "bokehAmount", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.7 },
        { "NAME": "bokehSize", "TYPE": "float", "MIN": 0.01, "MAX": 0.2, "DEFAULT": 0.07 },
        { "NAME": "chromatic", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.3 },
        { "NAME": "streakAmount", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.3 },
        { "NAME": "dustAmount", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.4 },
        { "NAME": "warmth", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.7 },
        { "NAME": "greenTint", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.3 },
        { "NAME": "speed", "TYPE": "float", "MIN": 0.0, "MAX": 2.0, "DEFAULT": 0.3 },
        { "NAME": "sourceX", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.85 },
        { "NAME": "sourceY", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.75 },
        { "NAME": "glowRadius", "TYPE": "float", "MIN": 0.1, "MAX": 2.0, "DEFAULT": 0.8 }
    ]
}*/

float hash2d(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

vec2 hash2v(vec2 p) {
    return vec2(
        fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453),
        fract(sin(dot(p, vec2(269.5, 183.3))) * 43758.5453)
    );
}

float gnoise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f);
    float a = hash2d(i);
    float b = hash2d(i + vec2(1.0, 0.0));
    float c = hash2d(i + vec2(0.0, 1.0));
    float d = hash2d(i + vec2(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

float fbm(vec2 p) {
    float v = 0.0;
    float a = 0.5;
    vec2 shift = vec2(100.0);
    mat2 rot = mat2(0.877, 0.479, -0.479, 0.877);
    int i = 0;
    for (i = 0; i < 5; i++) {
        v = v + a * gnoise(p);
        p = rot * p * 2.0 + shift;
        a = a * 0.5;
    }
    return v;
}

float hexDist(vec2 p) {
    p = abs(p);
    return max(p.x * 0.866025 + p.y * 0.5, p.y);
}

float hexBokeh(vec2 uv, vec2 center, float radius, float rotation) {
    vec2 d = uv - center;
    float cs = cos(rotation);
    float sn = sin(rotation);
    d = vec2(d.x * cs - d.y * sn, d.x * sn + d.y * cs);
    float hex = hexDist(d / radius);
    float edge = smoothstep(1.0, 0.85, hex);
    float ring = smoothstep(0.85, 0.95, hex) * smoothstep(1.05, 0.95, hex);
    float inner = smoothstep(1.0, 0.3, hex) * 0.15;
    return (edge * 0.3 + ring * 0.5 + inner) * smoothstep(1.1, 0.9, hex);
}

void main() {
    vec2 uv = isf_FragNormCoord;
    float aspect = RENDERSIZE.x / RENDERSIZE.y;
    vec2 uvA = vec2(uv.x * aspect, uv.y);
    float t = TIME * speed;

    vec2 src = vec2(sourceX * aspect, sourceY);
    vec2 toSource = src - uvA;
    float distToSource = length(toSource);

    float glow = exp(-distToSource * distToSource / (glowRadius * glowRadius * 0.5));

    float angle = atan(toSource.y, toSource.x);
    float n = rayCount;
    float noiseWarp = fbm(vec2(angle * 3.0, t * 0.1)) * 0.4;
    float rayPattern = sin(angle * n + t * 0.5 + noiseWarp * n) * 0.5 + 0.5;
    float rayNoise = fbm(vec2(angle * n * 0.5, distToSource * 3.0 + t * 0.05));
    rayPattern = rayPattern * (0.7 + rayNoise * 0.3);
    rayPattern = pow(rayPattern, 1.5);
    float rayFade = exp(-distToSource * 1.5);
    float rays = rayPattern * rayFade * rayIntensity;

    float rays2 = sin(angle * n * 2.3 - t * 0.3 + noiseWarp * n * 1.5) * 0.5 + 0.5;
    rays2 = pow(rays2, 2.25);
    rays = rays + rays2 * rayFade * rayIntensity * 0.3;

    float rays3 = sin(angle * n * 0.5 + t * 0.2) * 0.5 + 0.5;
    rays3 = pow(rays3, 1.0);
    rays = rays + rays3 * rayFade * rayIntensity * 0.15;

    float bokehR = 0.0;
    float bokehG = 0.0;
    float bokehB = 0.0;
    int i = 0;
    float fi = 0.0;
    vec2 bPos = vec2(0.0);
    float bRadius = 0.0;
    float bDist = 0.0;
    float bBright = 0.0;
    float bLayer = 0.0;
    float bRot = 0.0;
    float bVal = 0.0;
    float chromOff = 0.0;

    for (i = 0; i < 40; i++) {
        fi = float(i);
        bLayer = floor(fi / 13.0);
        bPos = vec2(hash2d(vec2(fi, 1.0)) * aspect, hash2d(vec2(fi, 2.0)));
        bPos = mix(bPos, src, hash2d(vec2(fi, 3.0)) * 0.5);
        float layerSpeed = 0.7 + bLayer * 0.3;
        bPos.x = bPos.x + sin(t * 0.2 * layerSpeed + fi * 0.7) * (0.015 + bLayer * 0.01);
        bPos.y = bPos.y + cos(t * 0.15 * layerSpeed + fi * 0.5) * (0.015 + bLayer * 0.01);
        float sizeMultiplier = 0.5 + bLayer * 0.4;
        bRadius = bokehSize * (0.3 + hash2d(vec2(fi, 4.0)) * 1.2) * sizeMultiplier;
        bRot = hash2d(vec2(fi, 6.0)) * 3.14159 + t * 0.05;
        bDist = length(bPos - src);
        bBright = exp(-bDist * bDist * 2.0);
        bBright = bBright * (0.2 + hash2d(vec2(fi, 5.0)) * 0.8);
        bBright = bBright * (0.6 + bLayer * 0.3);
        bVal = hexBokeh(uvA, bPos, bRadius, bRot) * bBright;
        chromOff = chromatic * bRadius * 0.3;
        bokehR = bokehR + hexBokeh(uvA + vec2(chromOff, 0.0), bPos, bRadius, bRot) * bBright;
        bokehG = bokehG + bVal;
        bokehB = bokehB + hexBokeh(uvA - vec2(chromOff, 0.0), bPos, bRadius, bRot) * bBright;
    }

    float bokehMono = bokehG * bokehAmount;
    vec3 bokehColor = vec3(bokehR, bokehG, bokehB) * bokehAmount;

    float streaks = 0.0;
    float sx = 0.0;
    float sy = 0.0;
    float sDist = 0.0;
    float sBright = 0.0;
    for (i = 0; i < 15; i++) {
        fi = float(i);
        sx = hash2d(vec2(fi, 10.0)) * aspect;
        sy = hash2d(vec2(fi, 11.0));
        vec2 sPos = mix(vec2(sx, sy), src, hash2d(vec2(fi, 12.0)) * 0.5);
        sPos.x = sPos.x + sin(t * 0.1 + fi) * 0.02;
        sPos.y = sPos.y + cos(t * 0.08 + fi) * 0.02;
        sDist = length(sPos - src);
        sBright = exp(-sDist * sDist * 3.0) * 0.4;
        float dy = abs(uvA.y - sPos.y);
        float streakFade = exp(-dy * dy * 800.0);
        float dx = abs(uvA.x - sPos.x);
        float streakLen = exp(-dx * dx * 8.0);
        streaks = streaks + streakFade * streakLen * sBright;
    }
    streaks = streaks * streakAmount;

    float dust = 0.0;
    float dfi = 0.0;
    vec2 dPos = vec2(0.0);
    float dDist = 0.0;
    float dBright = 0.0;
    float dSize = 0.0;
    float dd = 0.0;
    for (i = 0; i < 60; i++) {
        dfi = float(i);
        dPos = vec2(hash2d(vec2(dfi, 20.0)) * aspect, hash2d(vec2(dfi, 21.0)));
        float dSpeed = 0.5 + hash2d(vec2(dfi, 24.0)) * 0.5;
        dPos.x = dPos.x + sin(t * 0.3 * dSpeed + dfi * 1.7) * 0.04;
        dPos.y = dPos.y + cos(t * 0.2 * dSpeed + dfi * 1.3) * 0.04 + t * 0.01 * dSpeed;
        dPos.y = fract(dPos.y);
        dPos.x = mod(dPos.x, aspect);
        dDist = length(dPos - src);
        dBright = exp(-dDist * dDist * 1.5) * 0.5;
        dBright = dBright + 0.1;
        dSize = 0.002 + hash2d(vec2(dfi, 22.0)) * 0.004;
        dd = length(uvA - dPos);
        float sparkle = 0.5 + 0.5 * sin(t * 3.0 + dfi * 5.0);
        dust = dust + smoothstep(dSize, dSize * 0.2, dd) * dBright * sparkle;
    }
    dust = dust * dustAmount;

    vec3 warmColor = vec3(1.0, 0.85, 0.5);
    vec3 greenColor = vec3(0.6, 0.8, 0.3);
    vec3 shadowColor = vec3(0.2, 0.3, 0.15);

    float lightVal = glow + rays + bokehMono * 0.3;
    vec3 col = mix(shadowColor, warmColor, lightVal * warmth + glow);
    col = mix(col, greenColor, greenTint * (1.0 - glow) * 0.5);

    col = col + bokehColor * warmColor * 1.5;
    col = col + glow * glow * vec3(1.0, 0.95, 0.85) * 2.0;
    col = col + rays * mix(warmColor, vec3(1.0, 1.0, 0.9), 0.5);
    col = col + streaks * vec3(1.0, 0.9, 0.7);
    col = col + dust * vec3(1.0, 0.95, 0.8);

    col = col / (col + vec3(1.0));
    col = pow(col, vec3(0.9, 0.95, 1.1));

    vec4 original = IMG_NORM_PIXEL(inputImage, uv);
    col = mix(col, original.rgb + col, blend);

    gl_FragColor = vec4(col, 1.0);
}
