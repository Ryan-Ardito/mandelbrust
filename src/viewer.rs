use std::thread;

use crate::constants::ITERATIONS;
use crate::rendering::render;
use crate::imaging::{PostProc, screenshot, upscale_buffer};

#[derive(Debug)]
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

    pub fn update(&mut self, low_res: bool, scale: usize) {
        let mut width = self.width;
        let mut height = self.height;
        let mut iterations = self.iterations;
        if low_res {
            width /= scale;
            height /= scale;
            iterations /= 2;
        }
        let buffer = render(
            self.x_pos,
            self.y_pos,
            width,
            height,
            self.base_view_height,
            self.zoom,
            iterations,
        );
        if low_res {
            self.buffer = upscale_buffer(buffer, width, height, scale)
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
        let x_pos = self.x_pos;
        let y_pos = self.y_pos;
        let zoom = self.zoom;
        let iterations = self.iterations;
        thread::spawn(move || {
            screenshot(
                x_pos,
                y_pos,
                width,
                height,
                oversample,
                zoom,
                iterations,
                post_proc
            )
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