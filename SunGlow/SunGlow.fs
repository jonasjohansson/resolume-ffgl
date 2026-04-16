/*{
    "DESCRIPTION": "Warm sun rays with soft circular bokeh and bloom",
    "CREDIT": "Jonas",
    "ISFVSN": "2",
    "CATEGORIES": ["Generator"],
    "INPUTS": [
        { "NAME": "sourceX", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.85 },
        { "NAME": "sourceY", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.7 },
        { "NAME": "glowRadius", "TYPE": "float", "MIN": 0.1, "MAX": 3.0, "DEFAULT": 0.8 },
        { "NAME": "glowIntensity", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.5 },
        { "NAME": "bloom", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.3 },
        { "NAME": "haze", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.2 },
        { "NAME": "rayIntensity", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.4 },
        { "NAME": "rayCount", "TYPE": "float", "MIN": 2.0, "MAX": 30.0, "DEFAULT": 8.0 },
        { "NAME": "rayWidth", "TYPE": "float", "MIN": 0.5, "MAX": 4.0, "DEFAULT": 2.0 },
        { "NAME": "bokehAmount", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.6 },
        { "NAME": "bokehSize", "TYPE": "float", "MIN": 0.01, "MAX": 0.4, "DEFAULT": 0.1 },
        { "NAME": "bokehSoftness", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.4 },
        { "NAME": "warmth", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.8 },
        { "NAME": "greenTint", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.2 },
        { "NAME": "speed", "TYPE": "float", "MIN": 0.0, "MAX": 2.0, "DEFAULT": 0.2 },
        { "NAME": "rayWobble", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.4 }
    ]
}*/

float hash2d(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

void main() {
    vec2 uv = isf_FragNormCoord;
    float aspect = RENDERSIZE.x / RENDERSIZE.y;
    vec2 uvA = vec2(uv.x * aspect, uv.y);
    float t = TIME * speed;

    vec2 src = vec2(sourceX * aspect, sourceY);
    vec2 toSource = src - uvA;
    float distToSource = length(toSource);

    float glow = exp(-distToSource * distToSource / (glowRadius * glowRadius * 0.4));
    float wideGlow = exp(-distToSource * distToSource / (glowRadius * glowRadius * 1.5));
    float hugeGlow = exp(-distToSource / (glowRadius * 2.0));

    float angle = atan(toSource.y, toSource.x);
    float n = rayCount;
    float wobble1 = sin(angle * 3.0 + t * 0.7) * sin(angle * 5.0 - t * 0.5) * rayWobble * 0.5;
    float wobble2 = sin(angle * 7.0 + t * 0.3) * rayWobble * 0.25;
    float rayPattern = sin(angle * n + t * 0.4 + wobble1 * n) * 0.5 + 0.5;
    float rayBreath = 1.0 + sin(t * 0.6 + angle * 2.0) * rayWobble * 0.3;
    rayPattern = pow(rayPattern, rayWidth) * rayBreath;
    float rayFade = exp(-distToSource * 1.0);
    float rays = rayPattern * rayFade * rayIntensity;

    float rays2 = sin(angle * n * 2.1 - t * 0.25 + wobble2 * n) * 0.5 + 0.5;
    rays2 = pow(rays2, rayWidth * 1.3);
    rays = rays + rays2 * rayFade * rayIntensity * 0.2;

    float bokeh = 0.0;
    float bokehHalo = 0.0;
    int i = 0;
    float fi = 0.0;
    vec2 bPos = vec2(0.0);
    float bRadius = 0.0;
    float bDist = 0.0;
    float bBright = 0.0;
    float dd = 0.0;
    float circle = 0.0;
    float ring = 0.0;
    float haloVal = 0.0;
    float haloSize = 1.0 + bokehSoftness * 5.0;

    for (i = 0; i < 24; i++) {
        fi = float(i);
        bPos = vec2(hash2d(vec2(fi, 1.0)) * aspect, hash2d(vec2(fi, 2.0)));
        bPos = mix(bPos, src, hash2d(vec2(fi, 3.0)) * 0.55);
        bPos.x = bPos.x + sin(t * 0.12 + fi * 0.7) * 0.03;
        bPos.y = bPos.y + cos(t * 0.08 + fi * 0.5) * 0.03;
        bRadius = bokehSize * (0.3 + hash2d(vec2(fi, 4.0)) * 1.0);
        bDist = length(bPos - src);
        bBright = exp(-bDist * bDist * 1.2);
        bBright = bBright * (0.4 + hash2d(vec2(fi, 5.0)) * 0.6);

        dd = length(uvA - bPos);
        circle = smoothstep(bRadius, bRadius * 0.5, dd) * 0.3;
        ring = smoothstep(bRadius * 0.5, bRadius * 0.85, dd) * smoothstep(bRadius * 1.1, bRadius * 0.85, dd) * 0.6;
        bokeh = bokeh + (circle + ring) * bBright;

        haloVal = exp(-dd * dd / (bRadius * bRadius * haloSize));
        bokehHalo = bokehHalo + haloVal * bBright * bokehSoftness;
    }
    bokeh = bokeh * bokehAmount;
    bokehHalo = bokehHalo * bokehAmount;

    float totalBloom = (glow * 0.4 + wideGlow * 0.3 + hugeGlow * 0.3 + bokehHalo * 0.5) * bloom;
    float hazeVal = (wideGlow * 0.5 + hugeGlow * 0.5) * haze;

    vec3 warmWhite = vec3(1.0, 0.95, 0.85);
    vec3 goldenColor = vec3(1.0, 0.82, 0.42);
    vec3 deepGold = vec3(0.9, 0.68, 0.25);
    vec3 oliveColor = vec3(0.4, 0.38, 0.12);
    vec3 darkOlive = vec3(0.25, 0.22, 0.08);

    float lightVal = glow * glowIntensity + rays + bokeh * 0.3 + totalBloom;
    vec3 col = mix(darkOlive, oliveColor, greenTint);
    col = mix(col, deepGold, lightVal * 0.5 * warmth);
    col = mix(col, goldenColor, lightVal * warmth);
    col = mix(col, warmWhite, glow * glow * glowIntensity + totalBloom * 0.4);

    col = col + bokeh * goldenColor * 2.0;
    col = col + bokehHalo * goldenColor * 1.0;
    col = col + glow * glow * warmWhite * 3.5 * glowIntensity;
    col = col + totalBloom * warmWhite * 1.5;
    col = col + rays * goldenColor * 0.8;
    col = col + hazeVal * goldenColor * 0.6;

    col = col / (col + vec3(0.7));
    col = pow(col, vec3(0.78, 0.83, 0.98));

    gl_FragColor = vec4(col, 1.0);
}
