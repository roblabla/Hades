use hs_gba::protocol::FrontendChannels;

use crate::gui;

#[derive(Debug, Clone)]
pub struct App {
    pub channels: FrontendChannels,
    pub scale: u32,
}

impl App {
    pub fn new(channels: FrontendChannels) -> Self {
        Self {
            channels,
            scale: 0,
        }
    }

    /// Render the application
    #[allow(clippy::redundant_pattern_matching)]
    pub fn render(&mut self, ui: &mut imgui::Ui) {
        gui::menubar::render_menubar(self, ui);
    }
}