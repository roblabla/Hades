use crate::gui;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct App {
    pub scale: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            scale: 0,
        }
    }

    /// Render the application
    #[allow(clippy::redundant_pattern_matching)]
    pub fn render(&mut self, ui: &mut imgui::Ui) {
        gui::menubar::render_menubar(self, ui);
    }
}