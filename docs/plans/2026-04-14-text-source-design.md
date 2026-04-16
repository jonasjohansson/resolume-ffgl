# Text Source Plugin Design

## Motivation

Resolume's built-in Text Animator and Text Block have poor alignment, sizing, and layout. This plugin replaces them with a properly functioning text source that also supports beat-reactive line cycling.

## Approach

Rust FFGL plugin using macOS CoreGraphics/CoreText for text rendering (Approach C).

- CoreText enumerates and selects installed system fonts
- CoreGraphics renders styled, laid-out text to an in-memory RGBA pixel buffer
- Rust uploads the buffer as an OpenGL texture to Resolume
- Bitmap is cached and only re-rendered when parameters change or beat triggers a line change

## Architecture

```
Resolume ─► FFGL Plugin (Rust)
               │
               ├── Parameters (font, text, size, color, alignment...)
               ├── Beat info from host
               │
               ├── CoreText: font enumeration + selection
               ├── CoreGraphics: render styled text → RGBA pixel buffer
               │
               └── OpenGL: upload pixel buffer as texture → output to Resolume
```

## Parameters

### Text
- `text` — string input (multi-line, newline-separated)
- `beatCycle` — bool: cycle through lines on the beat (hard cut)

### Font
- `font` — dropdown of installed system fonts
- `fontSize` — float slider

### Styling
- `color` — RGBA color
- `outlineEnabled` — bool
- `outlineColor` — RGBA
- `outlineWidth` — float
- `shadowEnabled` — bool
- `shadowColor` — RGBA
- `shadowOffset` — float

### Layout
- `hAlign` — left / center / right
- `vAlign` — top / center / bottom
- `lineSpacing` — float
- `letterSpacing` — float
- `positionX` / `positionY` — float sliders to offset text

## Technical Notes

- macOS-only (CoreText/CoreGraphics dependency)
- Built via ffgl-rs (Rust FFGL framework)
- Output: `.bundle` deployed to Resolume effects directory
- Category: Generator (source)
- Beat sync uses FFGL host beat/phase info
