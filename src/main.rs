extern crate sdl2;

use imgui::Context;
use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode
};
use imgui_sdl2_support::SdlPlatform;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("Error creating SDL context");
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Conway's Game of Life", 400, 400).
        position_centered().
        build().
        expect("Error creating window");

    let mut imgui = Context::create();

    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    imgui
        .fonts()
        // .add_font(&[imgui::FontSource::DefaultFontData {config: None}])
        .build_alpha8_texture();

    let mut platform = SdlPlatform::init(&mut imgui);


    let mut canvas = window.into_canvas().build().expect("Error creating canvas");
    canvas.set_draw_color(Color::RGB(233, 100, 200));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().expect("Error creating event pump");
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 100, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn, x, y, ..} => {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {print!("Left,")},
                        sdl2::mouse::MouseButton::Right => {print!("Right,")},
                        _ => {print!("Any,")}
                    }
                    println!("{x}, {y}");
                }
                _ => {}
            }
        }
        platform.prepare_frame(&mut imgui, canvas.window(), &event_pump);
        let ui = imgui.new_frame();
        ui.show_demo_window(&mut true);
        let _draw_data = imgui.render();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
