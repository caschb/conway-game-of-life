use glow::HasContext;
use imgui::{Context};
use sdl2::{
    event::Event,
    keyboard::Keycode, video::{GLProfile, Window}, mouse::MouseButton
};
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn handle_mouse_button(mouse_button: MouseButton, x: i32, y: i32) {
    match mouse_button {
        sdl2::mouse::MouseButton::Right => { print!("Right, ") },
        sdl2::mouse::MouseButton::Left => { print!("Left, ") },
        _ => { print!("Any, ") }
    }
    println!("{x}, {y}");
}

fn main() {
    let sdl_context = sdl2::init().expect("Error creating SDL context");
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    let window = video_subsystem
        .window("Conway's Game of Life", 400, 400).
        allow_highdpi().
        resizable().
        position_centered().
        opengl().
        build().
        expect("Error creating window");
    
    let gl_context = window.gl_create_context().expect("Error with GL Context");
    window.gl_make_current(&gl_context).expect("Error creating window context");

    window.subsystem().gl_set_swap_interval(1).expect("Error changing window property");
    
    let gl = glow_context(&window);

    let mut imgui_context = Context::create();
    imgui_context.set_ini_filename(None);
    imgui_context.set_log_filename(None);
    imgui_context.fonts()
        .add_font(&[imgui::FontSource::DefaultFontData {config: None }]);


    let mut platform = SdlPlatform::init(&mut imgui_context);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui_context)
        .expect("Error creating renderer");


    let mut event_pump = sdl_context.event_pump().expect("Error creating event pump");

    unsafe { renderer.gl_context().clear_color(1.0f32, 0.2f32, 0.5f32, 1.0f32); }
    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    handle_mouse_button(mouse_btn, x, y)
                },
                _ => {}
            }
            platform.handle_event(&mut imgui_context, &event);
        }
        platform.prepare_frame(&mut imgui_context, &window, &event_pump);
        let ui = imgui_context.new_frame();
        ui.show_demo_window(&mut true);
        let draw_data = imgui_context.render();
        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).expect("Renderer error");
        window.gl_swap_window();
    }
}
