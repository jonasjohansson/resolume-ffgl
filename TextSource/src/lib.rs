mod params;
mod plugin;
mod renderer;
mod shader;

use ffgl_core::{self, handler::simplified::SimpleFFGLHandler};

ffgl_core::plugin_main!(SimpleFFGLHandler<plugin::TextSource>);
