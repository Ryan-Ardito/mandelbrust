use std::time::Duration;

use mandelbrust::{ Viewer, PostProc, constants::* };

use minifb::{ Key, Window, WindowOptions, KeyRepeat };

// view state
enum VS {
    Motion,
    LowRes,
    FullRes
}

fn main() {
    let mut viewer = Viewer::new(WIDTH, HEIGHT);
    let mut post_proc = PostProc::new();
    let mut view_buffer = viewer.buffer();

    let mut state = VS::FullRes;

    // true triggers post_proc and update_with_buffer
    let mut change = true;

    let mut window = Window::new(
        "Mandelbrot Viewer",
        viewer.width,
        viewer.height,
        WindowOptions::default(),
    ).unwrap();
    window.limit_update_rate(Some(Duration::from_nanos(FRAME_DURATION_NS)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // update window
        if change {
            let buffer = post_proc.process(&view_buffer);
            window.update_with_buffer(&buffer, viewer.width, viewer.height).unwrap();
            change = false;
        } else {
            window.update();
        }

        // single key press events
        window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key| match key {
            Key::L => println!("{:?}", viewer),
            Key::P => viewer.screenshot(1920, 1080, 4, post_proc),
            Key::O => { post_proc.blackwhite = !post_proc.blackwhite; change = true; },
            Key::U => { post_proc.grayscale = !post_proc.grayscale; change = true; },
            Key::I => { post_proc.invert = !post_proc.invert; change = true; },
            Key::K => { post_proc.clamp = !post_proc.clamp; change = true; },
            Key::T => { viewer.iter_up(); state = VS::LowRes; },
            Key::G => { viewer.iter_down(); state = VS::LowRes; },
            Key::RightBracket => viewer.downsample_up(),
            Key::LeftBracket => viewer.downsample_down(),
            Key::Key1 => { viewer.reset(); post_proc.reset(); state = VS::LowRes; },
            _ => (),
        });
        
        // keyboard repeat events
        window.get_keys_pressed(KeyRepeat::Yes).iter().for_each(|key| match key {
            Key::X => { post_proc.color_shift_up(); change = true; },
            Key::Z => { post_proc.color_shift_down(); change = true; },
            Key::F => { post_proc.color_scale_up(); change = true; },
            Key::C => { post_proc.color_scale_down(); change = true; },
            _ => (),
        });

        // continuous input events
        window.get_keys().iter().for_each(|key| match key {
            Key::Q => { viewer.zoom(1.0 - ZOOM_FACTOR); state = VS::Motion; },
            Key::E => { viewer.zoom(1.0 + ZOOM_FACTOR); state = VS::Motion; },
            Key::W => { viewer.pan(0.0, -PAN_FACTOR); state = VS::Motion; },
            Key::A => { viewer.pan(-PAN_FACTOR, 0.0); state = VS::Motion; },
            Key::S => { viewer.pan(0.0, PAN_FACTOR); state = VS::Motion; },
            Key::D => { viewer.pan(PAN_FACTOR, 0.0); state = VS::Motion; },
            _ => (),
        });

        // update render
        (view_buffer, state, change) = match state {
            VS::Motion => (viewer.buffer_low(), VS::LowRes, true),
            VS::LowRes => (viewer.buffer(), VS::FullRes, true),
            VS::FullRes => (view_buffer, state, change),
        };
    }
}