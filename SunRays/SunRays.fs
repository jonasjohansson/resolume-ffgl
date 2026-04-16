/*{
    "DESCRIPTION": "Warm sun rays with bokeh light circles",
    "CREDIT": "Jonas",
    "ISFVSN": "2",
    "CATEGORIES": ["Stylize"],
    "INPUTS": [
        { "NAME": "inputImage", "TYPE": "image" },
        { "NAME": "blend", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.0 },
        { "NAME": "rayIntensity", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.6 },
        { "NAME": "rayCount", "TYPE": "float", "MIN": 2.0, "MAX": 30.0, "DEFAULT": 12.0 },
        { "NAME": "bokehAmount", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.7 },
        { "NAME": "bokehSize", "TYPE": "float", "MIN": 0.01, "MAX": 0.15, "DEFAULT": 0.06 },
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

float bokehCircle(vec2 uv, vec2 center, float radius, float soft) {
    float d = length(uv - center);
    float edge = mix(0.005, radius * 0.8, soft);
    float ring = smoothstep(radius + edge, radius, d) * smoothstep(radius - edge * 0.5, radius, d);
    float fill = smoothstep(radius + edge, radius * 0.2, d);
    return fill * 0.6 + ring * 0.4;
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
    float rayPattern = sin(angle * n + t * 0.5) * 0.5 + 0.5;
    rayPattern = pow(rayPattern, 1.5);
    float rayFade = exp(-distToSource * 1.5);
    float rays = rayPattern * rayFade * rayIntensity;

    float rays2 = sin(angle * n * 2.3 - t * 0.3) * 0.5 + 0.5;
    rays2 = pow(rays2, 2.25);
    rays = rays + rays2 * rayFade * rayIntensity * 0.3;

    float bokeh = 0.0;
    int i = 0;
    float fi = 0.0;
    vec2 bPos = vec2(0.0);
    float bRadius = 0.0;
    float bDist = 0.0;
    float bBright = 0.0;

    for (i = 0; i < 30; i++) {
        fi = float(i);
        bPos = vec2(hash2d(vec2(fi, 1.0)) * aspect, hash2d(vec2(fi, 2.0)));
        bPos = mix(bPos, src, hash2d(vec2(fi, 3.0)) * 0.4);
        bPos.x = bPos.x + sin(t * 0.2 + fi * 0.7) * 0.02;
        bPos.y = bPos.y + cos(t * 0.15 + fi * 0.5) * 0.02;
        bRadius = bokehSize * (0.3 + hash2d(vec2(fi, 4.0)));
        bDist = length(bPos - src);
        bBright = exp(-bDist * bDist * 2.0);
        bBright = bBright * (0.3 + hash2d(vec2(fi, 5.0)) * 0.7);
        bokeh = bokeh + bokehCircle(uvA, bPos, bRadius, 0.7) * bBright;
    }
    bokeh = bokeh * bokehAmount;

    vec3 warmColor = vec3(1.0, 0.85, 0.5);
    vec3 greenColor = vec3(0.6, 0.8, 0.3);
    vec3 shadowColor = vec3(0.2, 0.3, 0.15);

    float lightVal = glow + rays + bokeh * 0.5;
    vec3 col = mix(shadowColor, warmColor, lightVal * warmth + glow);
    col = mix(col, greenColor, greenTint * (1.0 - glow) * 0.5);
    col = col + bokeh * warmColor * 1.5;
    col = col + glow * glow * vec3(1.0, 0.95, 0.85) * 2.0;
    col = col + rays * mix(warmColor, vec3(1.0, 1.0, 0.9), 0.5);
    col = col / (col + vec3(1.0));
    col = pow(col, vec3(0.9, 0.95, 1.1));

    vec4 original = IMG_NORM_PIXEL(inputImage, uv);
    col = mix(col, original.rgb + col, blend);

    gl_FragColor = vec4(col, 1.0);
}
