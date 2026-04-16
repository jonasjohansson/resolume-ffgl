# TextSource Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust FFGL source plugin that renders system-font text via CoreGraphics, with styling, layout, and beat-reactive line cycling.

**Architecture:** Rust FFGL plugin using `SimpleFFGLInstance` from ffgl-rs. CoreText enumerates fonts, CoreGraphics renders styled text to an RGBA pixel buffer, OpenGL uploads the buffer as a texture. Re-renders only when parameters change or beat triggers a line switch.

**Tech Stack:** Rust, ffgl-rs (ffgl-core), core-graphics + core-text + core-foundation crates, gl + gl_loader crates, OpenGL 2.1+

---

### Task 1: Scaffold the Rust crate

**Files:**
- Create: `TextSource/Cargo.toml`
- Create: `TextSource/src/lib.rs`

**Step 1: Create Cargo.toml**

```toml
[package]
name = "text_source"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
ffgl-core = { path = "../../ffgl-rs/ffgl-core" }
gl = "0.14.0"
gl_loader = "0.1.0"
core-foundation = "0.10"
core-graphics = "0.24"
core-text = "20.1"
```

Note: The `ffgl-core` path assumes `ffgl-rs` is at `~/Documents/GitHub/ffgl-rs`. Adjust the relative path if needed — check with `ls ~/Documents/GitHub/ffgl-rs` and `ls ~/Documents/GitHub/org/jonasjohansson-archive/ffgl-rs` to find the correct location.

**Step 2: Create minimal lib.rs**

```rust
mod plugin;

use ffgl_core::{self, handler::simplified::SimpleFFGLHandler};

ffgl_core::plugin_main!(SimpleFFGLHandler<plugin::TextSource>);
```

**Step 3: Create stub plugin.rs**

Create `TextSource/src/plugin.rs` with a minimal `SimpleFFGLInstance` that compiles — just clear to black, no params yet:

```rust
use ffgl_core::{handler::simplified::SimpleFFGLInstance, info::PluginInfo, FFGLData, GLInput};

pub struct TextSource;

impl SimpleFFGLInstance for TextSource {
    fn new(_inst_data: &FFGLData) -> Self {
        unsafe {
            gl_loader::init_gl();
            gl::load_with(|s| gl_loader::get_proc_address(s).cast());
        }
        Self
    }

    fn plugin_info() -> PluginInfo {
        PluginInfo {
            unique_id: *b"TxSr",
            name: *b"TextSource      ",
            ty: ffgl_core::info::PluginType::Source,
            about: "Text source with system fonts".to_string(),
            description: "Renders text using macOS CoreText/CoreGraphics".to_string(),
        }
    }

    fn draw(&mut self, _data: &FFGLData, _frame_data: GLInput) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
```

**Step 4: Verify it compiles**

```bash
cd TextSource && cargo build --release 2>&1
```

Expected: successful compilation producing `target/release/libtext_source.dylib`

**Step 5: Commit**

```bash
git add TextSource/
git commit -m "feat: scaffold TextSource Rust FFGL plugin crate"
```

---

### Task 2: Define all parameters

**Files:**
- Create: `TextSource/src/params.rs`
- Modify: `TextSource/src/plugin.rs`

**Step 1: Create params.rs with parameter constants and info array**

```rust
use std::ffi::CString;
use std::sync::LazyLock;
use ffgl_core::parameters::info::{ParameterTypes, SimpleParamInfo};

// Parameter indices
pub const PARAM_TEXT: usize = 0;
pub const PARAM_BEAT_CYCLE: usize = 1;
pub const PARAM_FONT: usize = 2;
pub const PARAM_FONT_SIZE: usize = 3;
pub const PARAM_COLOR_R: usize = 4;
pub const PARAM_COLOR_G: usize = 5;
pub const PARAM_COLOR_B: usize = 6;
pub const PARAM_COLOR_A: usize = 7;
pub const PARAM_OUTLINE_ENABLED: usize = 8;
pub const PARAM_OUTLINE_R: usize = 9;
pub const PARAM_OUTLINE_G: usize = 10;
pub const PARAM_OUTLINE_B: usize = 11;
pub const PARAM_OUTLINE_A: usize = 12;
pub const PARAM_OUTLINE_WIDTH: usize = 13;
pub const PARAM_SHADOW_ENABLED: usize = 14;
pub const PARAM_SHADOW_R: usize = 15;
pub const PARAM_SHADOW_G: usize = 16;
pub const PARAM_SHADOW_B: usize = 17;
pub const PARAM_SHADOW_A: usize = 18;
pub const PARAM_SHADOW_OFFSET: usize = 19;
pub const PARAM_H_ALIGN: usize = 20;
pub const PARAM_V_ALIGN: usize = 21;
pub const PARAM_LINE_SPACING: usize = 22;
pub const PARAM_LETTER_SPACING: usize = 23;
pub const PARAM_POS_X: usize = 24;
pub const PARAM_POS_Y: usize = 25;
pub const NUM_PARAMS: usize = 26;

/// Build font dropdown elements from installed system fonts.
/// Called once at plugin load via LazyLock.
fn build_font_elements() -> Vec<(CString, f32)> {
    // Enumerate system fonts via CoreText
    use core_text::font_collection::create_for_all_families;
    let collection = create_for_all_families();
    let descriptors = collection.get_descriptors();

    let mut fonts: Vec<String> = Vec::new();
    if let Some(descs) = descriptors {
        for i in 0..descs.len() as isize {
            if let Some(desc) = descs.get(i as usize) {
                let name = desc.family_name();
                if !fonts.contains(&name) {
                    fonts.push(name);
                }
            }
        }
    }
    fonts.sort();

    let count = fonts.len().max(1) as f32;
    fonts
        .into_iter()
        .enumerate()
        .map(|(i, name)| (CString::new(name).unwrap(), i as f32 / (count - 1.0).max(1.0)))
        .collect()
}

pub static PARAM_INFOS: LazyLock<Vec<SimpleParamInfo>> = LazyLock::new(|| {
    let font_elements = build_font_elements();

    vec![
        // 0: Text
        SimpleParamInfo {
            name: CString::new("Text").unwrap(),
            param_type: ParameterTypes::Text,
            default_string: Some(CString::new("Hello World").unwrap()),
            ..Default::default()
        },
        // 1: Beat Cycle
        SimpleParamInfo {
            name: CString::new("Beat Cycle").unwrap(),
            param_type: ParameterTypes::Boolean,
            default: Some(0.0),
            ..Default::default()
        },
        // 2: Font
        SimpleParamInfo {
            name: CString::new("Font").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(0.0),
            min: Some(0.0),
            max: Some((font_elements.len().max(1) - 1) as f32),
            elements: Some(font_elements),
            ..Default::default()
        },
        // 3: Font Size
        SimpleParamInfo {
            name: CString::new("Font Size").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.25),
            ..Default::default()
        },
        // 4-7: Color RGBA
        SimpleParamInfo {
            name: CString::new("Fill R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(1.0),
            group: Some("Fill Color".to_string()),
            ..Default::default()
        },
        SimpleParamInfo {
            name: CString::new("Fill G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(1.0),
            group: Some("Fill Color".to_string()),
            ..Default::default()
        },
        SimpleParamInfo {
            name: CString::new("Fill B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(1.0),
            group: Some("Fill Color".to_string()),
            ..Default::default()
        },
        SimpleParamInfo {
            name: CString::new("Fill A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(1.0),
            group: Some("Fill Color".to_string()),
            ..Default::default()
        },
        // 8: Outline Enabled
        SimpleParamInfo {
            name: CString::new("Outline").unwrap(),
            param_type: ParameterTypes::Boolean,
            default: Some(0.0),
            ..Default::default()
        },
        // 9-12: Outline Color RGBA
        SimpleParamInfo {
            name: CString::new("Outline R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(0.0),
            group: Some("Outline Color".to_string()),
            ..Default::default()
        },
        SimpleParamInfo {
            name: CString::new("Outline G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(0.0),
            group: Some("Outline Color".to_string()),
            ..Default::default()
        },
        SimpleParamInfo {
            name: CString::new("Outline B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(0.0),
            group: Some("Outline Color".to_string()),
            ..Default::default()
        },
        SimpleParamInfo {
            name: CString::new("Outline A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(1.0),
            group: Some("Outline Color".to_string()),
            ..Default::default()
        },
        // 13: Outline Width
        SimpleParamInfo {
            name: CString::new("Outline Width").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.1),
            ..Default::default()
        },
        // 14: Shadow Enabled
        SimpleParamInfo {
            name: CString::new("Shadow").unwrap(),
            param_type: ParameterTypes::Boolean,
            default: Some(0.0),
            ..Default::default()
        },
        // 15-18: Shadow Color RGBA
        SimpleParamInfo {
            name: CString::new("Shadow R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(0.0),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        SimpleParamInfo {
            name: CString::new("Shadow G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(0.0),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        SimpleParamInfo {
            name: CString::new("Shadow B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(0.0),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        SimpleParamInfo {
            name: CString::new("Shadow A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(0.5),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        // 19: Shadow Offset
        SimpleParamInfo {
            name: CString::new("Shadow Offset").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.1),
            ..Default::default()
        },
        // 20: Horizontal Alignment
        SimpleParamInfo {
            name: CString::new("H Align").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(1.0),
            min: Some(0.0),
            max: Some(2.0),
            elements: Some(vec![
                (CString::new("Left").unwrap(), 0.0),
                (CString::new("Center").unwrap(), 1.0),
                (CString::new("Right").unwrap(), 2.0),
            ]),
            ..Default::default()
        },
        // 21: Vertical Alignment
        SimpleParamInfo {
            name: CString::new("V Align").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(1.0),
            min: Some(0.0),
            max: Some(2.0),
            elements: Some(vec![
                (CString::new("Top").unwrap(), 0.0),
                (CString::new("Center").unwrap(), 1.0),
                (CString::new("Bottom").unwrap(), 2.0),
            ]),
            ..Default::default()
        },
        // 22: Line Spacing
        SimpleParamInfo {
            name: CString::new("Line Spacing").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 23: Letter Spacing
        SimpleParamInfo {
            name: CString::new("Letter Spacing").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 24: Position X
        SimpleParamInfo {
            name: CString::new("Position X").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 25: Position Y
        SimpleParamInfo {
            name: CString::new("Position Y").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
    ]
});
```

**Step 2: Add param state to TextSource struct in plugin.rs**

Add fields to hold parameter values and wire up `num_params`, `param_info`, `get_param`, `set_param`, `get_text_param`, `set_text_param`:

```rust
use std::ffi::{c_char, CString};
use crate::params::*;

pub struct TextSource {
    param_values: [f32; NUM_PARAMS],
    text: CString,
    dirty: bool,
}
```

Implement all the parameter trait methods, storing float values in the array and the text string separately. Set `dirty = true` in every `set_param` and `set_text_param`.

**Step 3: Verify it compiles**

```bash
cd TextSource && cargo build --release 2>&1
```

**Step 4: Commit**

```bash
git add TextSource/src/params.rs TextSource/src/plugin.rs
git commit -m "feat: define all TextSource parameters"
```

---

### Task 3: CoreGraphics text rendering module

**Files:**
- Create: `TextSource/src/renderer.rs`

**Step 1: Create the text renderer**

This module takes rendering parameters and produces an RGBA pixel buffer:

```rust
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core_graphics::base::kCGImageAlphaPremultipliedLast;
use core_graphics::color::CGColor;
use core_graphics::color_space::CGColorSpace;
use core_graphics::context::CGContext;
use core_text::font::CTFont;

pub struct TextRenderer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

pub struct RenderParams {
    pub text: String,
    pub font_family: String,
    pub font_size: f64,
    pub color: [f32; 4],          // RGBA
    pub outline_enabled: bool,
    pub outline_color: [f32; 4],
    pub outline_width: f32,
    pub shadow_enabled: bool,
    pub shadow_color: [f32; 4],
    pub shadow_offset: f32,
    pub h_align: u32,             // 0=left, 1=center, 2=right
    pub v_align: u32,             // 0=top, 1=center, 2=bottom
    pub line_spacing: f32,
    pub letter_spacing: f32,
    pub position_x: f32,          // 0..1
    pub position_y: f32,          // 0..1
}

impl TextRenderer {
    pub fn render(width: usize, height: usize, params: &RenderParams) -> Self {
        // 1. Create CGContext with RGBA color space
        let color_space = CGColorSpace::create_device_rgb();
        let mut ctx = CGContext::create_bitmap_context(
            None,
            width,
            height,
            8,
            width * 4,
            &color_space,
            kCGImageAlphaPremultipliedLast,
        );

        // 2. Clear to transparent
        ctx.clear_rect(core_graphics::geometry::CGRect::new(
            &core_graphics::geometry::CGPoint::new(0.0, 0.0),
            &core_graphics::geometry::CGSize::new(width as f64, height as f64),
        ));

        // 3. Create CTFont from family name
        let font_name = CFString::new(&params.font_family);
        let font = core_text::font::new_from_name(&font_name, params.font_size)
            .unwrap_or_else(|_| {
                // Fallback to Helvetica
                let fallback = CFString::new("Helvetica");
                core_text::font::new_from_name(&fallback, params.font_size).unwrap()
            });

        // 4. Render text lines with CoreGraphics text drawing
        //    - Split text by newlines
        //    - Measure each line for alignment
        //    - Apply letter spacing, line spacing
        //    - Position based on h_align, v_align, position_x, position_y
        //    - Draw shadow first if enabled (offset + color)
        //    - Draw outline if enabled (stroke mode)
        //    - Draw fill (fill mode)

        // ... (full implementation in this task)

        // 5. Extract pixel data
        let data = ctx.data();
        let len = width * height * 4;
        let pixels = unsafe { std::slice::from_raw_parts(data as *const u8, len) }.to_vec();

        Self { width, height, pixels }
    }
}
```

The full implementation should:
- Use `CTLine` / `CTFramesetter` from core-text for proper text layout and measurement
- Handle multi-line text by splitting on `\n`
- Measure each line width with `CTLine::get_typographic_bounds`
- Compute vertical position based on v_align and total text block height
- Apply h_align per-line
- Apply position_x/position_y as offset from aligned position
- Draw shadow pass first (translated by shadow_offset)
- Draw outline pass with `CGContext::set_text_drawing_mode(CGTextDrawingMode::Stroke)`
- Draw fill pass with `CGContext::set_text_drawing_mode(CGTextDrawingMode::Fill)`
- Map letter_spacing to `kCTKernAttributeName` via attributed strings

**Step 2: Verify it compiles**

```bash
cd TextSource && cargo build --release 2>&1
```

**Step 3: Commit**

```bash
git add TextSource/src/renderer.rs TextSource/src/lib.rs
git commit -m "feat: add CoreGraphics text rendering module"
```

---

### Task 4: OpenGL texture upload and fullscreen quad

**Files:**
- Modify: `TextSource/src/plugin.rs`

**Step 1: Add OpenGL state to TextSource**

Add fields for the texture ID, VAO, VBO, and shader program. Initialize them in `new()`:

- Create a simple vertex+fragment shader that draws a textured fullscreen quad
- Create VAO/VBO with fullscreen quad vertices (2 triangles)
- Create an OpenGL texture (RGBA, nearest filtering)

Vertex shader:
```glsl
#version 150
in vec2 position;
in vec2 texcoord;
out vec2 v_texcoord;
void main() {
    v_texcoord = texcoord;
    gl_Position = vec4(position, 0.0, 1.0);
}
```

Fragment shader:
```glsl
#version 150
in vec2 v_texcoord;
out vec4 out_color;
uniform sampler2D tex;
void main() {
    out_color = texture(tex, v_texcoord);
}
```

**Step 2: Implement draw()**

In `draw()`:
1. If `dirty` flag is set or beat triggered a line change:
   - Build `RenderParams` from current parameter values
   - Call `TextRenderer::render(viewport_width, viewport_height, &params)`
   - Upload pixel buffer to OpenGL texture via `glTexImage2D`
   - Clear `dirty` flag
2. Draw fullscreen quad with the texture

**Step 3: Implement Drop to clean up GL resources**

```rust
impl Drop for TextSource {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.program);
        }
    }
}
```

**Step 4: Verify it compiles**

```bash
cd TextSource && cargo build --release 2>&1
```

**Step 5: Commit**

```bash
git add TextSource/src/plugin.rs
git commit -m "feat: add OpenGL texture upload and fullscreen quad rendering"
```

---

### Task 5: Beat-reactive line cycling

**Files:**
- Modify: `TextSource/src/plugin.rs`

**Step 1: Add beat tracking state**

```rust
pub struct TextSource {
    // ... existing fields ...
    current_line: usize,
    last_beat_phase: f32,
}
```

**Step 2: Implement beat detection in draw()**

In `draw()`, before rendering:
1. Check if `beatCycle` param is enabled
2. Read `data.host_beat.barPhase`
3. Detect beat boundary: if `barPhase < last_beat_phase` (phase wrapped around from ~1.0 to ~0.0), a new bar started
4. On beat boundary: increment `current_line`, wrap around to 0 when past last line
5. Set `dirty = true` so the text re-renders with the new line

**Step 3: Modify renderer to support single-line mode**

When beat cycling is active, instead of rendering all lines, render only `lines[current_line]`.

**Step 4: Verify it compiles**

```bash
cd TextSource && cargo build --release 2>&1
```

**Step 5: Commit**

```bash
git add TextSource/src/plugin.rs
git commit -m "feat: add beat-reactive line cycling"
```

---

### Task 6: Build script and deploy

**Files:**
- Create: `TextSource/build_and_deploy.sh`
- Modify: `build.sh` (add note about TextSource)

**Step 1: Create build and deploy script**

```bash
#!/bin/bash
set -e

FFGL_RS="$HOME/Documents/GitHub/ffgl-rs"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

cd "$FFGL_RS"

# Build the plugin (cargo needs to find the workspace or use manifest path)
cargo build --release --manifest-path "$SCRIPT_DIR/Cargo.toml"

# Deploy bundle
bash deploy_bundle.sh text_source TextSource
```

Note: This may need adjustment depending on whether `TextSource/Cargo.toml` can find `ffgl-core` via the relative path, or whether it needs to be a workspace member of ffgl-rs. If the latter, we'll add it as a workspace member temporarily or use a path dependency that works standalone.

**Step 2: Build and deploy**

```bash
cd TextSource && bash build_and_deploy.sh
```

Expected: `TextSource.bundle` deployed to `~/Library/Graphics/FreeFrame Plug-Ins/` and/or `~/Documents/Resolume Arena/Extra Effects/`

**Step 3: Test in Resolume**

- Open Resolume Arena
- Add a new source → find TextSource in the source list
- Verify: text renders, font dropdown works, color/outline/shadow params work, alignment works, beat cycling works

**Step 4: Commit**

```bash
git add TextSource/build_and_deploy.sh
git commit -m "feat: add TextSource build and deploy script"
```

---

### Task 7: Polish and edge cases

**Files:**
- Modify: `TextSource/src/renderer.rs`
- Modify: `TextSource/src/plugin.rs`

**Step 1: Handle edge cases**

- Empty text string: render nothing (transparent)
- Font not found: fallback to Helvetica
- Single-line text with beat cycle enabled: just show that line (no cycling)
- Very long text: clamp to viewport bounds
- Zero font size: skip rendering

**Step 2: Optimize re-rendering**

- Only re-render when parameters actually change (compare new vs old values)
- Cache the font object between renders if font selection hasn't changed
- Consider rendering at a fixed internal resolution and scaling, rather than re-rendering at full viewport size every frame

**Step 3: Final build and test**

```bash
cd TextSource && bash build_and_deploy.sh
```

Test all parameters in Resolume.

**Step 4: Commit**

```bash
git add TextSource/
git commit -m "feat: polish TextSource edge cases and rendering optimization"
```
