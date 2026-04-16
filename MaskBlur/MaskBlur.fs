/*{
    "DESCRIPTION": "Gaussian blur driven by a loaded mask image — white = blur, black = no blur",
    "CREDIT": "Jonas Johansson",
    "ISFVSN": "2",
    "CATEGORIES": ["Effect"],
    "INPUTS": [
        { "NAME": "inputImage", "TYPE": "image" },
        { "NAME": "maskImage", "TYPE": "image", "FILE": true },
        { "NAME": "blurSize", "TYPE": "float", "MIN": 0.0, "MAX": 200.0, "DEFAULT": 10.0 },
        { "NAME": "amount", "TYPE": "float", "MIN": 0.0, "MAX": 3.0, "DEFAULT": 1.0 },
        { "NAME": "maskSoftness", "TYPE": "float", "MIN": 0.0, "MAX": 50.0, "DEFAULT": 0.0 },
        { "NAME": "maskOffsetX", "TYPE": "float", "MIN": -1.0, "MAX": 1.0, "DEFAULT": 0.0 },
        { "NAME": "maskOffsetY", "TYPE": "float", "MIN": -1.0, "MAX": 1.0, "DEFAULT": 0.0 },
        { "NAME": "maskScaleX", "TYPE": "float", "MIN": 0.1, "MAX": 5.0, "DEFAULT": 1.0 },
        { "NAME": "maskScaleY", "TYPE": "float", "MIN": 0.1, "MAX": 5.0, "DEFAULT": 1.0 },
        { "NAME": "invert", "TYPE": "bool", "DEFAULT": false }
    ]
}*/

void main() {
    vec2 uv = isf_FragNormCoord;
    vec4 original = IMG_NORM_PIXEL(inputImage, uv);
    vec2 px = 1.0 / RENDERSIZE;

    // Transform UV for mask: offset then scale around center
    vec2 muv = uv;
    muv -= vec2(maskOffsetX, maskOffsetY);
    muv = (muv - 0.5) / vec2(maskScaleX, maskScaleY) + 0.5;

    // Sample mask with optional softness (blur the mask itself)
    float mask;
    if (muv.x < 0.0 || muv.x > 1.0 || muv.y < 0.0 || muv.y > 1.0) {
        mask = 0.0;
    } else if (maskSoftness > 0.0) {
        float total = 0.0;
        float wSum = 0.0;
        float sigma = maskSoftness * 0.5;
        float invSig2 = 1.0 / (2.0 * sigma * sigma + 0.001);
        int radius = int(ceil(maskSoftness));
        radius = min(radius, 15);
        for (int j = -15; j <= 15; j++) {
            for (int i = -15; i <= 15; i++) {
                if (abs(i) > radius || abs(j) > radius) continue;
                float fi = float(i);
                float fj = float(j);
                float d2 = fi * fi + fj * fj;
                float w = exp(-d2 * invSig2);
                total += IMG_NORM_PIXEL(maskImage, muv + vec2(fi, fj) * px).r * w;
                wSum += w;
            }
        }
        mask = total / wSum;
    } else {
        mask = IMG_NORM_PIXEL(maskImage, muv).r;
    }

    if (invert) mask = 1.0 - mask;

    float effectAmt = mask * amount;
    if (effectAmt < 0.01) {
        gl_FragColor = original;
        return;
    }

    // Adaptive Gaussian blur — radius scales with mask brightness
    float r = blurSize * effectAmt;
    vec3 blurred = vec3(0.0);
    float totalWeight = 0.0;
    float sigma = r * 0.5;
    float invSig2 = 1.0 / (2.0 * sigma * sigma + 0.001);

    for (int j = -15; j <= 15; j++) {
        for (int i = -15; i <= 15; i++) {
            float fi = float(i);
            float fj = float(j);
            float d2 = fi * fi + fj * fj;
            if (d2 > 225.0) continue;
            float w = exp(-d2 * invSig2);
            blurred += IMG_NORM_PIXEL(inputImage, uv + vec2(fi, fj) * px * r * 0.15).rgb * w;
            totalWeight += w;
        }
    }
    blurred /= totalWeight;

    gl_FragColor = vec4(mix(original.rgb, blurred, effectAmt), original.a);
}
