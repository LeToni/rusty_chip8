use glutin_window::GlutinWindow;
use opengl_graphics::GlGraphics;
use piston::Size;

pub struct GraphicsConfig {
    pub gl: GlGraphics,
    pub settings: GlutinWindow,
    pub size: Size,
}
