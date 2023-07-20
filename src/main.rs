#[allow(unused_imports)]
use std::{ thread, time::Duration };

use mandelbrust::{ Viewer, PostProc, constants::* };

use minifb::{ Key, Window, WindowOptions, KeyRepeat };


fn main() {
    let mut viewer = Viewer::new(WIDTH, HEIGHT);
    viewer.update(false, 2);
    // true when the final render is displayed
    let mut full_res = true;
    // true triggers low-res first pass
    let mut motion = false;

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

        // single key press events
        window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key| match key {
            Key::L => println!("{:?}", viewer),
            Key::O => post_proc.blackwhite = !post_proc.blackwhite,
            Key::U => post_proc.grayscale = !post_proc.grayscale,
            Key::I => post_proc.invert = !post_proc.invert,
            Key::K => post_proc.clamp = !post_proc.clamp,
            Key::P => viewer.screenshot(1920, 1080, 4, post_proc),
            Key::T => { viewer.iter_up(); full_res = false; },
            Key::G => { viewer.iter_down(); full_res = false; },
            Key::Key1 => { viewer.reset(); post_proc.reset(); full_res = false; },
            _ => (),
        });
        
        // keyboard repeat events
        window.get_keys_pressed(KeyRepeat::Yes).iter().for_each(|key| match key {
            Key::X => post_proc.color_shift_up(),
            Key::Z => post_proc.color_shift_down(),
            Key::F => post_proc.color_scale_up(),
            Key::C => post_proc.color_scale_down(),
            _ => (),
        });

        // continuous input events
        window.get_keys().iter().for_each(|key| match key {
            Key::Q => { viewer.zoom(1.0 - ZOOM_FACTOR); motion = true; },
            Key::E => { viewer.zoom(1.0 + ZOOM_FACTOR); motion = true; },
            Key::W => { viewer.pan(0.0, -PAN_FACTOR); motion = true; },
            Key::A => { viewer.pan(-PAN_FACTOR, 0.0); motion = true; },
            Key::S => { viewer.pan(0.0, PAN_FACTOR); motion = true; },
            Key::D => { viewer.pan(PAN_FACTOR, 0.0); motion = true; },
            _ => (),
        });

        // update render
        if motion {
            viewer.update(true, 2);
            motion = false;
            full_res = false;
        } else if !full_res {
            viewer.update(false, 2);
            full_res = true;
        }

        thread::sleep(Duration::new(0, FRAME_DURATION_NS))
    }
}