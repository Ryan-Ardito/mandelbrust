use crate::constants::*;
use crate::utils::{render, get_color, save_image};

pub struct Viewer {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub color_scale: f64,
    pub color_shift: u32,
    pub iterations: u32,
    base_view_height: f64,
    x_center: f64,
    y_center: f64,
    zoom: f64,
}

impl Viewer {
    pub fn new(width: usize, height: usize) -> Viewer {
        Viewer {
            buffer: vec![0; width * height],
            width,
            height,
            color_scale: 1.0,
            color_shift: 0,
            iterations: ITERATIONS,
            base_view_height: 2.0 / (width as f64 / height as f64),
            x_center: 0.0,
            y_center: 0.0,
            zoom: 1.0,
        }
    }

    pub fn screenshot(&self, path: &str, width: usize, height: usize, oversample: u32) {
        let hi_w = width * oversample as usize;
        let hi_h = height * oversample as usize;
        let base_vh = 2.0 / (hi_w as f64 / hi_h as f64);
        let hires_buffer = render(self.x_center, self.y_center, hi_w, hi_h, base_vh, self.zoom, self.iterations);
        let color_buffer = hires_buffer.iter().map(|&value|
            get_color(value, self.color_scale, self.color_shift)
        ).collect::<Vec<u32>>();
        save_image(color_buffer, hi_w as u32, hi_h as u32, path, oversample).unwrap();
    }
    
    pub fn reset(&mut self) {
        *self = Self::new(self.width, self.height);
    }

    pub fn iter_up(&mut self) {
        self.iterations <<= 1;
    }

    pub fn iter_down(&mut self) {
        self.iterations = self.iterations >> 1 | 1;
    }

    pub fn update(&mut self) {
        self.buffer = render(
            self.x_center,
            self.y_center,
            self.width,
            self.height,
            self.base_view_height,
            self.zoom,
            self.iterations,
        );
    }

    pub fn zoom(&mut self, factor: f64) {
        // limit zooming out
        if self.zoom * factor < 0.2 { return; }
        self.zoom *= factor;
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        // bound movement to bounds of the set
        if self.x_center + dx / self.zoom < -2.0
        || self.x_center + dx / self.zoom > 2.0
        || self.y_center + dy / self.zoom < -2.0
        || self.y_center + dy / self.zoom > 2.0 {
            return;
        }
        self.x_center += dx / self.zoom;
        self.y_center += dy / self.zoom;
    }
}