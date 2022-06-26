use ggez::conf::*;
pub const WINDOW_SIZE: (f32, f32) = (640.0, 320.0);
// Make a Context.
pub const WINDOW_MODE_CONF: ggez::conf::WindowMode = WindowMode {
    width: WINDOW_SIZE.0,
    /// Window height in physical pixels
    height: WINDOW_SIZE.1,
    /// Whether or not to maximize the window
    maximized: false,
    /// Fullscreen type
    fullscreen_type: FullscreenType::Windowed,
    /// Whether or not to show window decorations
    borderless: false,
    /// Whether or not the window should be transparent
    transparent: false,
    /// Minimum width for resizable windows; 1 is the technical minimum,
    /// as wgpu will panic on a width of 0.
    min_width: 1.0,
    /// Minimum height for resizable windows; 1 is the technical minimum,
    /// as wgpu will panic on a height of 0.
    min_height: 1.0,
    /// Maximum width for resizable windows; 0 means no limit
    max_width: 0.0,
    /// Maximum height for resizable windows; 0 means no limit
    max_height: 0.0,
    /// Whether or not the window is resizable
    resizable: false,
    /// Whether this window should displayed (true) or hidden (false)
    visible: true,
    resize_on_scale_factor_change: false,
};
