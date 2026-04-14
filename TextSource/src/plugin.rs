use ffgl_core::{handler::simplified::SimpleFFGLInstance, info::PluginInfo, FFGLData, GLInput};

pub struct TextSource;

impl SimpleFFGLInstance for TextSource {
    fn new(_inst_data: &FFGLData) -> Self {
        gl_loader::init_gl();
        gl::load_with(|s| gl_loader::get_proc_address(s).cast());
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
