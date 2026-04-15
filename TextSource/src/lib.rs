mod params;
mod plugin;
mod renderer;

use ffgl_core::{self, handler::simplified::SimpleFFGLHandler};

ffgl_core::plugin_main!(SimpleFFGLHandler<plugin::TextSource>);
