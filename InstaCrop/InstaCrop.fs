/*{
    "DESCRIPTION": "Centered pixel crop with black letterbox. Common Instagram sizes: 1080x1080 (1:1), 1080x1350 (4:5), 1080x1440 (3:4), 1080x566 (1.91:1), 1080x1920 (9:16)",
    "CREDIT": "Jonas",
    "ISFVSN": "2",
    "CATEGORIES": ["Stylize"],
    "INPUTS": [
        { "NAME": "inputImage", "TYPE": "image" },
        { "NAME": "cropWidth", "TYPE": "float", "MIN": 1.0, "MAX": 4096.0, "DEFAULT": 1080.0 },
        { "NAME": "cropHeight", "TYPE": "float", "MIN": 1.0, "MAX": 4096.0, "DEFAULT": 1440.0 },
        { "NAME": "barColor", "TYPE": "color", "DEFAULT": [0.0, 0.0, 0.0, 1.0] }
    ]
}*/

void main() {
    vec2 uv = isf_FragNormCoord;
    vec4 color = IMG_NORM_PIXEL(inputImage, uv);

    vec2 d = abs(isf_FragNormCoord - 0.5) * RENDERSIZE;
    vec2 halfCrop = vec2(cropWidth, cropHeight) * 0.5;

    bool inside = d.x <= halfCrop.x && d.y <= halfCrop.y;

    gl_FragColor = inside ? color : barColor;
}
