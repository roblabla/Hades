#![allow(clippy::redundant_pattern_matching)]

use imgui::WindowFlags;

use crate::app::App;

pub fn render_menubar_file(ui: &imgui::Ui) {
    if let Some(_) = ui.begin_menu("File") {

    }
}

pub fn render_menubar_emulation(ui: &imgui::Ui) {
    if let Some(_) = ui.begin_menu("Emulation") {

    }
}

pub fn render_menubar_video(ui: &imgui::Ui) {
    if let Some(_) = ui.begin_menu("Video") {

    }
}

pub fn render_menubar_audio(ui: &imgui::Ui) {
    if let Some(_) = ui.begin_menu("Audio") {

    }
}

pub fn render_menubar_help(ui: &imgui::Ui) {
    let mut open_about = false;

    if let Some(_) = ui.begin_menu("Help") {
        if ui.menu_item("Report Issue") {

        }

        ui.separator();

        if ui.menu_item("About") {
            open_about = true;
        }
    }

    if open_about {
        ui.open_popup("About");
    }

    ui.modal_popup_config("About")
        .flags(WindowFlags::NO_RESIZE | WindowFlags::NO_MOVE)
        .build(|| {
        ui.text("Hades");
        ui.spacing();
        ui.separator();
        ui.spacing();
        ui.text(format!("Version: {}", env!("CARGO_PKG_VERSION")));
        ui.spacing();
        ui.separator();
        ui.spacing();
        ui.text("Software written by Arignir");
        ui.text("Thank you for using it <3");
        ui.spacing();
        if ui.button("Close") {
            ui.close_current_popup();
        }
    });
}

pub fn render_menubar(_app: &App, ui: &mut imgui::Ui) {
    if let Some(_) = ui.begin_main_menu_bar() {
        render_menubar_file(ui);
        render_menubar_emulation(ui);
        render_menubar_video(ui);
        render_menubar_audio( ui);
        render_menubar_help(ui);
    }
}