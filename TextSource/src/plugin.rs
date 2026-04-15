use std::ffi::{c_char, CString};
use std::mem;
use std::ptr;

use gl::types::*;

use ffgl_core::{
    handler::simplified::SimpleFFGLInstance,
    info::PluginInfo,
    parameters::ParamInfo,
    FFGLData, GLInput,
};

use crate::params::*;
use crate::renderer::{RenderParams, TextRenderer};
use crate::shader;

// ---------------------------------------------------------------------------
// Shaders
// ---------------------------------------------------------------------------

const VS_SRC: &str = "#version 150
in vec2 position;
in vec2 texcoord;
out vec2 v_texcoord;
void main() {
    v_texcoord = texcoord;
    gl_Position = vec4(position, 0.0, 1.0);
}";

const FS_SRC: &str = "#version 150
in vec2 v_texcoord;
out vec4 out_color;
uniform sampler2D tex;
void main() {
    out_color = texture(tex, v_texcoord);
}";

// ---------------------------------------------------------------------------
// Fullscreen quad vertices: position(x,y) + texcoord(u,v)
// ---------------------------------------------------------------------------

#[rustfmt::skip]
static QUAD_VERTICES: [GLfloat; 16] = [
    -1.0, -1.0,  0.0, 0.0,  // bottom-left
     1.0, -1.0,  1.0, 0.0,  // bottom-right
    -1.0,  1.0,  0.0, 1.0,  // top-left
     1.0,  1.0,  1.0, 1.0,  // top-right
];

// ---------------------------------------------------------------------------
// Plugin struct
// ---------------------------------------------------------------------------

pub struct TextSource {
    param_values: [f32; NUM_PARAMS],
    text: CString,
    dirty: bool,
    // Beat-reactive line cycling
    current_line: usize,
    last_bar_phase: f32,
    // OpenGL state
    texture_id: GLuint,
    vao: GLuint,
    vbo: GLuint,
    program: GLuint,
}

impl SimpleFFGLInstance for TextSource {
    fn new(_inst_data: &FFGLData) -> Self {
        gl_loader::init_gl();
        gl::load_with(|s| gl_loader::get_proc_address(s).cast());

        // Initialize param_values from defaults
        let mut param_values = [0.0f32; NUM_PARAMS];
        for (i, info) in PARAM_INFOS.iter().enumerate() {
            param_values[i] = info.default.unwrap_or(0.0);
        }

        // Compile and link shaders
        let vs = shader::compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = shader::compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = shader::link_program(vs, fs);

        let (mut vao, mut vbo, mut texture_id) = (0, 0, 0);

        unsafe {
            // --- VAO / VBO ---------------------------------------------------
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (QUAD_VERTICES.len() * mem::size_of::<GLfloat>()) as isize,
                QUAD_VERTICES.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::UseProgram(program);

            // Bind fragment output
            let out_name = c"out_color";
            gl::BindFragDataLocation(program, 0, out_name.as_ptr());

            // Set up vertex attributes
            let stride = (4 * mem::size_of::<GLfloat>()) as GLsizei;

            let pos_name = c"position";
            let pos_attr = gl::GetAttribLocation(program, pos_name.as_ptr());
            if pos_attr >= 0 {
                gl::EnableVertexAttribArray(pos_attr as GLuint);
                gl::VertexAttribPointer(
                    pos_attr as GLuint,
                    2,
                    gl::FLOAT,
                    gl::FALSE as GLboolean,
                    stride,
                    ptr::null(),
                );
            }

            let tex_name = c"texcoord";
            let tex_attr = gl::GetAttribLocation(program, tex_name.as_ptr());
            if tex_attr >= 0 {
                gl::EnableVertexAttribArray(tex_attr as GLuint);
                gl::VertexAttribPointer(
                    tex_attr as GLuint,
                    2,
                    gl::FLOAT,
                    gl::FALSE as GLboolean,
                    stride,
                    (2 * mem::size_of::<GLfloat>()) as *const _,
                );
            }

            // --- Texture -----------------------------------------------------
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            // Unbind
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::UseProgram(0);
        }

        Self {
            param_values,
            text: CString::new("Hello World").unwrap(),
            dirty: true,
            current_line: 0,
            last_bar_phase: 0.0,
            texture_id,
            vao,
            vbo,
            program,
        }
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

    fn num_params() -> usize {
        NUM_PARAMS
    }

    fn param_info(index: usize) -> &'static dyn ParamInfo {
        &PARAM_INFOS[index]
    }

    fn get_param(&self, index: usize) -> f32 {
        self.param_values[index]
    }

    fn set_param(&mut self, index: usize, value: f32) {
        self.param_values[index] = value;
        self.dirty = true;
    }

    fn get_text_param(&self, index: usize) -> *const c_char {
        match index {
            PARAM_TEXT => self.text.as_ptr(),
            _ => ptr::null(),
        }
    }

    fn set_text_param(&mut self, index: usize, value: &str) {
        match index {
            PARAM_TEXT => {
                if let Ok(cstr) = CString::new(value) {
                    self.text = cstr;
                    self.dirty = true;
                }
            }
            _ => {}
        }
    }

    fn draw(&mut self, data: &FFGLData, _frame_data: GLInput) {
        let (width, height) = (data.viewport.width as usize, data.viewport.height as usize);

        // Beat-reactive line cycling
        if self.param_values[PARAM_BEAT_CYCLE] > 0.5 {
            let bar_phase = data.host_beat.barPhase;
            // Detect beat boundary: barPhase wrapped from near 1.0 back to near 0.0
            if bar_phase < 0.1 && self.last_bar_phase > 0.9 {
                let full_text = self.text.to_str().unwrap_or("");
                let line_count = full_text.split('\n').count();
                if line_count > 1 {
                    self.current_line = (self.current_line + 1) % line_count;
                    self.dirty = true;
                }
            }
            self.last_bar_phase = bar_phase;
        }

        if self.dirty && width > 0 && height > 0 {
            self.dirty = false;

            let params = self.build_render_params();
            let rendered = TextRenderer::render(width, height, &params);

            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    rendered.width as i32,
                    rendered.height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    rendered.pixels.as_ptr() as *const _,
                );
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }

        // Draw fullscreen textured quad
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            let tex_uniform_name = c"tex";
            let tex_loc = gl::GetUniformLocation(self.program, tex_uniform_name.as_ptr());
            gl::Uniform1i(tex_loc, 0);

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
            gl::Disable(gl::BLEND);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

impl TextSource {
    fn build_render_params(&self) -> RenderParams {
        let pv = &self.param_values;

        // Look up font family name from the font param elements
        let font_family = {
            let font_info = &PARAM_INFOS[PARAM_FONT];
            if let Some(ref elements) = font_info.elements {
                let font_val = pv[PARAM_FONT];
                // The font value is a float; find the nearest element index
                let idx = (font_val * (elements.len().saturating_sub(1)) as f32)
                    .round()
                    .max(0.0) as usize;
                let idx = idx.min(elements.len().saturating_sub(1));
                elements[idx].0.to_str().unwrap_or("Helvetica").to_string()
            } else {
                "Helvetica".to_string()
            }
        };

        let font_size = (pv[PARAM_FONT_SIZE] * 400.0).max(1.0);

        // When beat cycle is enabled, show only the current line
        let full_text = self.text.to_str().unwrap_or("").to_string();
        let text = if pv[PARAM_BEAT_CYCLE] > 0.5 {
            let lines: Vec<&str> = full_text.split('\n').collect();
            if lines.len() > 1 {
                lines[self.current_line % lines.len()].to_string()
            } else {
                full_text
            }
        } else {
            full_text
        };

        RenderParams {
            text,
            font_family,
            font_size: font_size as f64,
            color: [pv[PARAM_FILL_R], pv[PARAM_FILL_G], pv[PARAM_FILL_B], pv[PARAM_FILL_A]],
            outline_enabled: pv[PARAM_OUTLINE_ENABLED] > 0.5,
            outline_color: [
                pv[PARAM_OUTLINE_R],
                pv[PARAM_OUTLINE_G],
                pv[PARAM_OUTLINE_B],
                pv[PARAM_OUTLINE_A],
            ],
            outline_width: pv[PARAM_OUTLINE_WIDTH] * 20.0,
            shadow_enabled: pv[PARAM_SHADOW_ENABLED] > 0.5,
            shadow_color: [
                pv[PARAM_SHADOW_R],
                pv[PARAM_SHADOW_G],
                pv[PARAM_SHADOW_B],
                pv[PARAM_SHADOW_A],
            ],
            shadow_offset: pv[PARAM_SHADOW_OFFSET] * 50.0,
            h_align: pv[PARAM_H_ALIGN].round() as u32,
            v_align: pv[PARAM_V_ALIGN].round() as u32,
            line_spacing: pv[PARAM_LINE_SPACING] * 4.0,
            letter_spacing: (pv[PARAM_LETTER_SPACING] - 0.5) * 40.0,
            position_x: pv[PARAM_POSITION_X],
            position_y: pv[PARAM_POSITION_Y],
        }
    }
}

// ---------------------------------------------------------------------------
// Cleanup
// ---------------------------------------------------------------------------

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
