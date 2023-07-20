use std::io::{ Write, stdout };
use std::thread;

use rayon::prelude::*;

use crate::constants::{ITERATIONS, IMAGE_PATH};
use crate::rendering::{render, save_image, scale_image};

pub struct Viewer {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub color_scale: f64,
    pub color_shift: u32,
    pub iterations: u32,
    base_view_height: f64,
    x_pos: f64,
    y_pos: f64,
    zoom: f64,
}

impl Viewer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
            color_scale: 1.0,
            color_shift: 0,
            iterations: ITERATIONS,
            base_view_height: 2.0 / (width as f64 / height as f64),
            x_pos: 0.0,
            y_pos: 0.0,
            zoom: 1.0,
        }
    }

    pub fn update(&mut self, low_res: bool) {
        let mut width = self.width;
        let mut height = self.height;
        if low_res {
            width /= 4;
            height /= 4;
        }
        let buffer = render(
            self.x_pos,
            self.y_pos,
            width,
            height,
            self.base_view_height,
            self.zoom,
            self.iterations,
        );
        if low_res {
            self.buffer = scale_image(buffer, self.width / 4, self.height / 4)
        } else {
            self.buffer = buffer;
        }
    }

    pub fn screenshot(
        &self,
        width: usize,
        height: usize,
        oversample: u32,
        post_proc: PostProc,
    ) {
        print!("Saving... ");
        stdout().flush().expect("terminal error");

        let x_pos = self.x_pos;
        let y_pos = self.y_pos;
        let hi_w = width * oversample as usize;
        let hi_h = height * oversample as usize;
        let base_vh = 2.0 / (hi_w as f64 / hi_h as f64);
        let zoom = self.zoom;
        let iterations = self.iterations;
        thread::spawn(move || {
            let hires_buffer = render(
                x_pos,
                y_pos,
                hi_w,
                hi_h,
                base_vh,
                zoom,
                iterations,
            );
            let color_buffer = post_proc.process(&hires_buffer);
            match save_image(
                color_buffer,
                hi_w as u32,
                hi_h as u32,
                IMAGE_PATH,
                oversample,
            ) {
                Ok(()) => println!("done!"),
                Err(_) => println!("failed!"),
            }
        });
    }
    
    pub fn reset(&mut self) {
        *self = Self::new(self.width, self.height);
    }

    pub fn iter_up(&mut self) {
        self.iterations *= 2;
        println!("Iterations: {}", self.iterations);
    }

    pub fn iter_down(&mut self) {
        if self.iterations > 1 { self.iterations /= 2; }
        println!("Iterations: {}", self.iterations);
    }

    pub fn zoom(&mut self, factor: f64) {
        // limit zooming out
        if self.zoom * factor < 0.4 { return; }
        self.zoom *= factor;
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        // bound movement to bounds of the set
        if self.out_of_bounds(dx, dy) { return; }
        self.x_pos += dx / self.zoom;
        self.y_pos += dy / self.zoom;
    }

    fn out_of_bounds(&self, dx: f64, dy: f64) -> bool {
        self.x_pos + dx / self.zoom < -2.0
        || self.x_pos + dx / self.zoom > 2.0
        || self.y_pos + dy / self.zoom < -2.0
        || self.y_pos + dy / self.zoom > 2.0
    }
}

#[derive(Clone, Copy)]
pub struct PostProc {
    pub color_scale: f64,
    pub color_shift: u32,
    pub blackwhite: bool,
    pub grayscale: bool,
    pub invert: bool,
    pub clamp: bool,
}

impl Default for PostProc {
    fn default() -> Self {
        Self::new()
    }
}

impl PostProc {
    pub fn new() -> Self {
        Self {
            color_scale: 1.0,
            color_shift: 0,
            blackwhite: false,
            grayscale: false,
            invert: false,
            clamp: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn color_shift_up(&mut self) {
            self.color_shift += 1;
    }

    pub fn color_shift_down(&mut self) {
        if self.color_shift > 1 {
            self.color_shift -= 1;
        }
    }

    pub fn color_scale_up(&mut self) {
        self.color_scale *= 1.01
    }

    pub fn color_scale_down(&mut self) {
        if self.color_scale > 1.0 {
            self.color_scale /= 1.01;
        }
    }

    pub fn process(&self, buffer: &[u32]) -> Vec<u32> {
        buffer
            .par_iter()
            .map(|&value| self.get_color(value))
            .collect()
    }

    fn get_color(&self, value: u32) -> u32 {
        if value == 0 { return 0; }

        if self.blackwhite { return 0xFFFFFF; }

        let mut val = self.color_shift + (value as f64 / self.color_scale) as u32;

        if self.clamp {
            val = val.clamp(0, 255);
        }

        if self.invert {
            val = !(val as u8) as u32;
        }

        if self.grayscale {
            val = val % 256;
            (val << 16) | (val << 8) | val
        } else {
            let r = val % 8 * 32;
            let g = val % 16 * 16;
            let b = val % 32 * 8;
            (r << 16) | (g << 8) | b
        }
    }
}