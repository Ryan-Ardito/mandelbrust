use std::{time::Duration, io::Write};

use minifb::{Key, Window, WindowOptions};

mod viewer;
mod constants;
mod utils;

use viewer::Viewer;
use constants::*;
use utils::get_color;


fn main() {
    let mut viewer = Viewer::new(WIDTH, HEIGHT);
    viewer.update();
    let mut last_update_time = std::time::Instant::now();
    let mut change = true;

    let mut window = Window::new(
        "Mandelbrot Viewer",
        viewer.width,
        viewer.height,
        WindowOptions::default(),
    ).unwrap();
    window.limit_update_rate(Some(Duration::from_millis(8)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // update window
        let buffer = viewer.buffer.iter().map(|&value|
            get_color(value, viewer.color_scale, viewer.color_shift)
        ).collect::<Vec<u32>>();
        window.update_with_buffer(&buffer, viewer.width, viewer.height).unwrap();

        // handle input
        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            print!("Saving... ");
            std::io::stdout().flush().unwrap();
            viewer.screenshot("screenshots/mandel.png", 1920, 1080, 4);
            println!("done!");
        }
        if window.is_key_pressed(Key::X, minifb::KeyRepeat::Yes) {
            viewer.color_shift += 1;
        }
        if window.is_key_pressed(Key::Z, minifb::KeyRepeat::Yes)
        && viewer.color_shift > 0 {
            viewer.color_shift -= 1;
        }
        if window.is_key_pressed(Key::F, minifb::KeyRepeat::Yes) {
            viewer.color_scale *= 1.01;
        }
        if window.is_key_pressed(Key::C, minifb::KeyRepeat::Yes)
        && viewer.color_scale > 1.0 {
            viewer.color_scale /= 1.01;
        }
        let keys = window.get_keys();
        if window.is_key_pressed(Key::T, minifb::KeyRepeat::No) {
            viewer.iter_up();
            println!("Iterations: {}", viewer.iterations);
            change = true;
        }
        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No) {
            viewer.iter_down();
            println!("Iterations: {}", viewer.iterations);
            change = true;
        }
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            viewer.reset();
            change = true;
        }
        if keys.contains(&Key::Q) { viewer.zoom(1.0 - ZOOM_FACTOR); change = true; }
        if keys.contains(&Key::E) { viewer.zoom(1.0 + ZOOM_FACTOR); change = true; }
        if keys.contains(&Key::W) { viewer.pan(0.0, -PAN_FACTOR); change = true; }
        if keys.contains(&Key::A) { viewer.pan(-PAN_FACTOR, 0.0); change = true; }
        if keys.contains(&Key::S) { viewer.pan(0.0, PAN_FACTOR); change = true; }
        if keys.contains(&Key::D) { viewer.pan(PAN_FACTOR, 0.0); change = true; }

        // update render
        let elapsed = last_update_time.elapsed().as_secs_f64();
        if change && elapsed >= UPDATE_TIME_STEP {
            viewer.update();
            last_update_time = std::time::Instant::now();
            change = false;
        }

    }
}