use glium::{
    Display,
    Surface,
    glutin::{
        ContextBuilder,
        event_loop::EventLoop,
        window::WindowBuilder,
        dpi::LogicalSize,
    },
};
use imgui::{
    self,
    FontSource,
    FontConfig,
};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use crate::app::App;

pub struct Window {
    pub display: glium::Display,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub scale: f64,
}

impl Window {
    pub fn open(event_loop: &EventLoop<()>, imgui: &mut imgui::Context) -> Self {

        let ctx_builder = ContextBuilder::new()
            .with_vsync(true);

        let win_builder = WindowBuilder::new()
            .with_title("Hades")
            .with_inner_size(LogicalSize::new(1024.0f64, 768.0f64));

        let display =
            Display::new(win_builder, ctx_builder, &event_loop)
            .expect("Failed to initialize display");

        let mut platform = WinitPlatform::init(imgui);
        {
            let gl_window = display.gl_window();
            let window= gl_window.window();
            platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
        }

        let scale = platform.hidpi_factor();

        let font_size = (13.0 * scale) as f32;
        imgui.fonts().add_font(&[
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: font_size,
                    ..FontConfig::default()
                }),
            },
        ]);

        imgui.io_mut().font_global_scale = (1.0 / scale) as f32;

        let renderer = Renderer::init(imgui, &display).expect("Failed to initialize renderer");

        Self {
            display,
            platform,
            renderer,
            scale,
        }
    }

    /// Ask `app` to render the frame, draw it and swap the OpenGL buffers
    pub fn frame(&mut self, app: &mut App, imgui: &mut imgui::Context) {
        let gl_window = self.display.gl_window();
        self.platform
            .prepare_frame(imgui.io_mut(), gl_window.window())
            .expect("Failed to prepare frame");
        gl_window.window().request_redraw();

        let ui = imgui.new_frame();
        app.render(ui);

        let gl_window = self.display.gl_window();
        let mut target = self.display.draw();

        target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);

        self.platform.prepare_render(ui, gl_window.window());

        let draw_data = imgui.render();

        self.renderer
            .render(&mut target, draw_data)
            .expect("Rendering failed");

        target.finish().expect("Failed to swap buffers");
    }
}
