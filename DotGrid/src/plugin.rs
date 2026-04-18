use std::ffi::c_char;
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
use crate::renderer::{DotGridRenderer, RenderParams};
use crate::shader;

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

#[rustfmt::skip]
static QUAD_VERTICES: [GLfloat; 16] = [
    -1.0, -1.0,  0.0, 0.0,
     1.0, -1.0,  1.0, 0.0,
    -1.0,  1.0,  0.0, 1.0,
     1.0,  1.0,  1.0, 1.0,
];

pub struct DotGrid {
    param_values: [f32; NUM_PARAMS],
    dirty: bool,
    last_width: usize,
    last_height: usize,
    texture_id: GLuint,
    vao: GLuint,
    vbo: GLuint,
    program: GLuint,
}

impl SimpleFFGLInstance for DotGrid {
    fn new(_inst_data: &FFGLData) -> Self {
        gl_loader::init_gl();
        gl::load_with(|s| gl_loader::get_proc_address(s).cast());

        let mut param_values = [0.0f32; NUM_PARAMS];
        for (i, info) in PARAM_INFOS.iter().enumerate() {
            param_values[i] = info.default.unwrap_or(0.0);
        }

        let vs = shader::compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = shader::compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = shader::link_program(vs, fs);

        let (mut vao, mut vbo, mut texture_id) = (0, 0, 0);

        unsafe {
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

            let out_name = c"out_color";
            gl::BindFragDataLocation(program, 0, out_name.as_ptr());

            let pos_name = c"position";
            let pos_attr = gl::GetAttribLocation(program, pos_name.as_ptr()) as GLuint;
            gl::EnableVertexAttribArray(pos_attr);
            gl::VertexAttribPointer(
                pos_attr,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * mem::size_of::<GLfloat>()) as i32,
                ptr::null(),
            );

            let tex_name = c"texcoord";
            let tex_attr = gl::GetAttribLocation(program, tex_name.as_ptr()) as GLuint;
            gl::EnableVertexAttribArray(tex_attr);
            gl::VertexAttribPointer(
                tex_attr,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * mem::size_of::<GLfloat>()) as i32,
                (2 * mem::size_of::<GLfloat>()) as *const _,
            );

            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::UseProgram(0);
        }

        Self {
            param_values,
            dirty: true,
            last_width: 0,
            last_height: 0,
            texture_id,
            vao,
            vbo,
            program,
        }
    }

    fn plugin_info() -> PluginInfo {
        PluginInfo {
            unique_id: *b"DtGr",
            name: *b"DotGrid         ",
            ty: ffgl_core::info::PluginType::Source,
            about: "Grid of dots".to_string(),
            description: "Renders a grid of dots with configurable size and alignment".to_string(),
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
        if (self.param_values[index] - value).abs() > f32::EPSILON {
            self.param_values[index] = value;
            self.dirty = true;
        }
    }

    fn get_text_param(&self, _index: usize) -> *const c_char {
        ptr::null()
    }

    fn set_text_param(&mut self, _index: usize, _value: &str) {}

    fn draw(&mut self, data: &FFGLData, _frame_data: GLInput) {
        let (width, height) = (data.viewport.width as usize, data.viewport.height as usize);

        if width != self.last_width || height != self.last_height {
            self.last_width = width;
            self.last_height = height;
            self.dirty = true;
        }

        if self.dirty && width > 0 && height > 0 {
            self.dirty = false;
            let params = self.build_render_params();
            let rendered = DotGridRenderer::render(width, height, &params);

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

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}

impl DotGrid {
    fn build_render_params(&self) -> RenderParams {
        let pv = &self.param_values;
        RenderParams {
            columns: pv[PARAM_COLUMNS].round().max(1.0) as u32,
            rows: pv[PARAM_ROWS].round().max(1.0) as u32,
            dot_size: pv[PARAM_DOT_SIZE],
            align_x: pv[PARAM_ALIGN_X].round() as u32,
            align_y: pv[PARAM_ALIGN_Y].round() as u32,
            fill: [pv[PARAM_FILL_R], pv[PARAM_FILL_G], pv[PARAM_FILL_B], pv[PARAM_FILL_A]],
            background: [pv[PARAM_BG_R], pv[PARAM_BG_G], pv[PARAM_BG_B], pv[PARAM_BG_A]],
        }
    }
}
