use std::ffi::{c_char, CString};
use std::ptr;

use ffgl_core::{
    handler::simplified::SimpleFFGLInstance,
    info::PluginInfo,
    parameters::ParamInfo,
    FFGLData, GLInput,
};

use crate::params::*;

pub struct TextSource {
    param_values: [f32; NUM_PARAMS],
    text: CString,
    dirty: bool,
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

        Self {
            param_values,
            text: CString::new("Hello World").unwrap(),
            dirty: true,
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

    fn draw(&mut self, _data: &FFGLData, _frame_data: GLInput) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
