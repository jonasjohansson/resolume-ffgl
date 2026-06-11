/*{
    "DESCRIPTION": "Centered circular crop with optional feathered edge. Outside is black, or transparent when 'transparent' is enabled.",
    "CREDIT": "Jonas",
    "ISFVSN": "2",
    "CATEGORIES": ["Stylize"],
    "INPUTS": [
        { "NAME": "inputImage", "TYPE": "image" },
        { "NAME": "radius", "TYPE": "float", "MIN": 1.0, "MAX": 2160.0, "DEFAULT": 540.0 },
        { "NAME": "feather", "TYPE": "float", "MIN": 0.0, "MAX": 512.0, "DEFAULT": 0.0 },
        { "NAME": "offsetX", "TYPE": "float", "MIN": -2048.0, "MAX": 2048.0, "DEFAULT": 0.0 },
        { "NAME": "offsetY", "TYPE": "float", "MIN": -2048.0, "MAX": 2048.0, "DEFAULT": 0.0 },
        { "NAME": "transparent", "TYPE": "bool", "DEFAULT": false },
        { "NAME": "invert", "TYPE": "bool", "DEFAULT": false }
    ]
}*/

void main() {
    vec4 color = IMG_NORM_PIXEL(inputImage, isf_FragNormCoord);

    vec2 uv = isf_FragNormCoord - 0.5;
    uv.x *= RENDERSIZE.x / RENDERSIZE.y;
    vec2 centerPx = uv * RENDERSIZE.y - vec2(offsetX, offsetY);
    float d = length(centerPx);

    float t = smoothstep(radius, radius + max(feather, 0.0001), d);
    if (invert) t = 1.0 - t;

    vec4 outside = transparent ? vec4(0.0) : vec4(0.0, 0.0, 0.0, 1.0);

    gl_FragColor = mix(color, outside, t);
}
