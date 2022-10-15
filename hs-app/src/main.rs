#![allow(dead_code)]

mod app;
mod window;
mod gui;

use std::{
    thread,
    time::Instant,
};

use glium::glutin::{
    event_loop::{
        EventLoop,
        ControlFlow,
    },
    event::{
        Event,
        WindowEvent,
    },
};

use hs_gba::Emulator;

use crate::{
    app::App,
    window::Window,
};

struct Hades {
    app: App,
    event_loop: EventLoop<()>,
    window: Window,
    imgui: imgui::Context,
}

impl Hades {
    pub fn new() -> Self {
        let event_loop  = EventLoop::new();

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let window = Window::open(&event_loop, &mut imgui);

        let mut app = App::new();
        app.scale = window.scale as u32;

        Self {
            app,
            event_loop,
            window,
            imgui,
        }
    }

    pub fn run(mut self) {
        let mut last_frame = Instant::now();
        self.event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();

                self.imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::RedrawRequested(_) => {
                self.window.frame(&mut self.imgui, |ui| {
                    self.app.render(ui);
                });
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = self.window.display.gl_window();
                self.window.platform.handle_event(self.imgui.io_mut(), gl_window.window(), &event);
            }
        });

    }
}

fn main() {

    // Spawn the Emulator thread

    thread::spawn(move || {
        let mut emulator = Emulator::new();
        emulator.run();
    });

    // Continue in the Gui thread

    let hades = Hades::new();
    hades.run();
}