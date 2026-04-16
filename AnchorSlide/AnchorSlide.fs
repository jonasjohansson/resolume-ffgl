/*{
    "DESCRIPTION": "Animate the anchor point within a scaled-up image — smooth panning without repeat or tiling",
    "CREDIT": "Jonas Johansson",
    "ISFVSN": "2",
    "CATEGORIES": ["Effect"],
    "INPUTS": [
        { "NAME": "inputImage", "TYPE": "image" },
        { "NAME": "speedX", "TYPE": "float", "MIN": 0.0, "MAX": 2.0, "DEFAULT": 0.1 },
        { "NAME": "speedY", "TYPE": "float", "MIN": 0.0, "MAX": 2.0, "DEFAULT": 0.07 },
        { "NAME": "rangeX", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.5 },
        { "NAME": "rangeY", "TYPE": "float", "MIN": 0.0, "MAX": 1.0, "DEFAULT": 0.3 },
        { "NAME": "phase", "TYPE": "float", "MIN": 0.0, "MAX": 6.28, "DEFAULT": 1.57 },
        { "NAME": "centerX", "TYPE": "float", "MIN": -0.5, "MAX": 0.5, "DEFAULT": 0.0 },
        { "NAME": "centerY", "TYPE": "float", "MIN": -0.5, "MAX": 0.5, "DEFAULT": 0.0 },
        { "NAME": "zoom", "TYPE": "float", "MIN": 1.0, "MAX": 5.0, "DEFAULT": 1.0 },
        { "NAME": "ease", "TYPE": "float", "MIN": 0.5, "MAX": 3.0, "DEFAULT": 1.0 },
        { "NAME": "pingPong", "TYPE": "bool", "DEFAULT": true }
    ]
}*/

float easedSin(float x, float e) {
    float s = sin(x);
    float sign = s >= 0.0 ? 1.0 : -1.0;
    return sign * pow(abs(s), e);
}

void main() {
    vec2 uv = isf_FragNormCoord;

    // Animate X and Y independently
    float tx, ty;
    if (pingPong) {
        tx = easedSin(TIME * speedX * 3.14159, ease);
        ty = easedSin(TIME * speedY * 3.14159 + phase, ease);
    } else {
        tx = fract(TIME * speedX * 0.5) * 2.0 - 1.0;
        ty = fract(TIME * speedY * 0.5 + phase / 6.28) * 2.0 - 1.0;
    }

    // How much room we have to pan
    float slack = 1.0 - 1.0 / zoom;

    // Offset with center bias
    vec2 offset = vec2(tx * rangeX + centerX, ty * rangeY + centerY) * slack * 0.5;

    // Zoom into center then apply offset
    vec2 zoomed = (uv - 0.5) / zoom + 0.5 + offset;

    // Clamp to edge — no repeat, no tiling
    zoomed = clamp(zoomed, 0.0, 1.0);

    gl_FragColor = IMG_NORM_PIXEL(inputImage, zoomed);
}
