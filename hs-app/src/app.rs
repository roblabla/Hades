use imgui;

use crate::gui::Gui;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct App {
    gui: Gui,
}

impl App {
    pub fn new() -> Self {
        Self {
            gui: Gui::new(),
        }
    }

    /// Render the UI
    pub fn render(&mut self, ui: &mut imgui::Ui) {
        let test_win = ui.window("Game");
        if let Some(_) = test_win.begin() {
            ui.text_wrapped("Hello, World");
        }

    }
}