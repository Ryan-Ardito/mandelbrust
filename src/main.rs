#[allow(unused_imports)]

mod viewer;
mod constants;
mod utils;

use viewer::{Viewer, PostProc};
use constants::*;

use minifb::{Key, Window, WindowOptions, KeyRepeat};


fn main() {
    let mut viewer = Viewer::new(WIDTH, HEIGHT);
    viewer.update();
    let mut last_update_time = std::time::Instant::now();
    let mut change = true;

    let mut post_proc = PostProc::new();

    let mut window = Window::new(
        "Mandelbrot Viewer",
        viewer.width,
        viewer.height,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // update window
        let buffer = post_proc.process(&viewer.buffer);
        window.update_with_buffer(&buffer, viewer.width, viewer.height).unwrap();

        // handle input
        window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key| match key {
            Key::U => post_proc.grayscale = !post_proc.grayscale,
            Key::I => post_proc.invert = !post_proc.invert,
            Key::K => post_proc.clamp = !post_proc.clamp,
            Key::P => viewer.screenshot(1920, 1080, 4, post_proc),
            Key::T => { viewer.iter_up(); change = true; },
            Key::G => { viewer.iter_down(); change = true; },
            Key::Key1 => { viewer.reset(); post_proc.reset(); change = true; },
            _ => (),
        });
        
        window.get_keys_pressed(KeyRepeat::Yes).iter().for_each(|key| match key {
            Key::X => post_proc.color_shift_up(),
            Key::Z => post_proc.color_shift_down(),
            Key::F => post_proc.color_scale_up(),
            Key::C => post_proc.color_scale_down(),
            _ => (),
        });

        window.get_keys().iter().for_each(|key| match key {
            Key::Q => { viewer.zoom(1.0 - ZOOM_FACTOR); change = true; },
            Key::E => { viewer.zoom(1.0 + ZOOM_FACTOR); change = true; },
            Key::W => { viewer.pan(0.0, -PAN_FACTOR); change = true; },
            Key::A => { viewer.pan(-PAN_FACTOR, 0.0); change = true; },
            Key::S => { viewer.pan(0.0, PAN_FACTOR); change = true; },
            Key::D => { viewer.pan(PAN_FACTOR, 0.0); change = true; },
            _ => (),
        });

        // update render
        let elapsed = last_update_time.elapsed().as_secs_f64();
        if change && elapsed >= UPDATE_TIME_STEP {
            viewer.update();
            last_update_time = std::time::Instant::now();
            change = false;
        }
    }
}