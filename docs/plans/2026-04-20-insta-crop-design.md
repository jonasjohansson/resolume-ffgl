# InstaCrop Design

## Motivation

Mimic Instagram's crop framing in Resolume: set a visible area in pixels (e.g. 1080×1440 on a 1080×1920 composition) and letterbox everything outside in black. Useful for previewing how content will look when posted to Instagram at various aspect ratios.

## Approach

ISF shader compiled as FFGL effect. A centered rectangle of `cropWidth × cropHeight` pixels passes through the source; everything outside becomes solid opaque black.

## Parameters

- `inputImage` — bound by Resolume to the layer/composition the effect sits on
- `cropWidth` — pixels, 1–4096, default 1080
- `cropHeight` — pixels, 1–4096, default 1440

No preset dropdown — ISF shaders can't write back into their own params, so presets would be cosmetic only. Common Instagram sizes are documented in the effect description for reference.

## Math

```
pixelCoord = gl_FragCoord.xy
center     = RENDERSIZE * 0.5
d          = abs(pixelCoord - center)
halfCrop   = vec2(cropWidth, cropHeight) * 0.5
inside     = d.x <= halfCrop.x && d.y <= halfCrop.y
output     = inside ? input : vec4(0, 0, 0, 1)
```

Crop is centered, not movable. If the crop is larger than the composition the whole frame passes through (`d` can never exceed half the composition size).

## File

`InstaCrop/InstaCrop.fs`

Category: Stylize. Build via `ffgl-isf/deploy_isf.sh` per project README.
