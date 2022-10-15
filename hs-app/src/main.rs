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
    }
};

use hs_gba::Emulator;
use crate::{
    app::App,
    window::Window,
};

fn main() {
    let event_loop = EventLoop::new();

    // Spawn the Emulator thread

    thread::spawn(move || {
        let mut emulator = Emulator::new();
        emulator.run();
    });

    // Continue in the Gui thread

    let mut app = App::new();

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let mut window = Window::open(&event_loop, &mut imgui);

    let mut last_frame = Instant::now();
    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = Instant::now();

            imgui.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }
        Event::RedrawRequested(_) => {
            window.frame(&mut app, &mut imgui);
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        event => {
            let gl_window = window.display.gl_window();
            window.platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
        }
    });
}