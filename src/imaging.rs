use std::io::{ Write, stdout };

use crate::rendering::{render, MetaData};
use crate::constants::IMAGE_PATH;

use image::{DynamicImage, ImageBuffer, Rgba, imageops::FilterType};
use rayon::prelude::*;

/// VERY simple linear scaling. glitchy at high scale
pub fn upscale_buffer(buffer: &[u32], width: usize, height: usize, scale: usize) -> Vec<u32> {
    let new_width = width * scale;
    let new_height = height * scale;
    let mut scaled_buffer = vec![0; new_width * new_height];

    for y in 0..height {
        let dest_y = y * scale;
        for x in 0..width {
            let orig_pixel = buffer[y * width + x];
            let dest_x = x * scale;
            let dest_index = dest_y * new_width + dest_x;

            for dy in 0..scale {
                let dest_offset = dest_index + dy * new_width;
                for dx in 0..scale {
                    scaled_buffer[dest_offset + dx] = orig_pixel;
                }
            }
        }
    }

    scaled_buffer
}

pub fn screenshot(data: MetaData, oversample: u32, post_proc: PostProc) {
    print!("Saving... ");
    stdout().flush().expect("terminal error");

    let buffer = render(data);
    let color_buffer = post_proc.process(&buffer);

    match save_image(
        color_buffer,
        data.width as u32,
        data.height as u32,
        IMAGE_PATH,
        oversample,
    ) {
        Ok(()) => println!("done!"),
        Err(_) => println!("failed!"),
    }
}

fn save_image(
    buffer: Vec<u32>,
    width: u32,
    height: u32,
    file_path: &str,
    oversample: u32,
) -> image::ImageResult<()> {
    let image_buffer = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = buffer[(y * width + x) as usize];
        Rgba([(pixel >> 16) as u8, (pixel >> 8) as u8, pixel as u8, 255])
    });

    let mut dynamic_image: DynamicImage = DynamicImage::ImageRgba8(image_buffer);

    if oversample > 1 {
        dynamic_image = dynamic_image.resize(
            width / oversample,
            height / oversample,
            FilterType::Lanczos3,
        );
    }

    dynamic_image.save(file_path)
}

#[derive(Debug, Clone, Copy)]
pub struct PostProc {
    pub color_scale: f64,
    pub color_shift: u32,
    pub blackwhite: bool,
    pub grayscale: bool,
    pub invert: bool,
    pub clamp: bool,
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
            .map(|&value| self.color_pixel(value))
            .collect()
    }

    fn color_pixel(&self, num_iters: u32) -> u32 {
        if num_iters == 0 { return num_iters; }

        if self.blackwhite { return 0xFFFFFF; }

        let mut val = self.color_shift + (num_iters as f64 / self.color_scale) as u32;

        if self.clamp {
            val = val.clamp(0, 255);
        }

        if self.invert {
            val = !(val as u8) as u32;
        }

        if self.grayscale {
            val %= 256;
            (val << 16) | (val << 8) | val
        } else {
            let r = val % 8 * 32;
            let g = val % 16 * 16;
            let b = val % 32 * 8;
            (r << 16) | (g << 8) | b
        }
    }
}