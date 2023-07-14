use rayon::prelude::*;
use image::{DynamicImage, ImageBuffer, Rgba};

pub fn render(x: f64, y: f64, width: usize, height: usize, base_view_height: f64, zoom: f64, iterations: u32) -> Vec<u32> {
    let x_min = x - (2.0 / zoom);
    let x_max = x + (2.0 / zoom);
    let y_min = y - (base_view_height / zoom);
    let y_max = y + (base_view_height / zoom);
    let x_exp = (x_max - x_min) / (width - 1) as f64;
    let y_exp = (y_max - y_min) / (height - 1) as f64;

    let mut buffer = vec![0; width * height];

    buffer
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row)| {
            let y_scaled = y_min + (y as f64) * y_exp;

            let mut x_scaled = x_min + 1.0 * x_exp;
            for pixel in row.iter_mut() {
                *pixel = calc_pixel(x_scaled, y_scaled, iterations);
                x_scaled += x_exp;
            }
        });
    buffer
}

pub fn get_color(value: u32, scale: f64, shift: u32) -> u32 {
    match value {
        0 => 0,
        _ => {
            let r = (shift + (value as f64 / scale) as u32 % 8) * 32;
            let g = (shift + (value as f64 / scale) as u32 % 16) * 16;
            let b = (shift + (value as f64 / scale) as u32 % 32) * 8;
            (r << 16) | (g << 8) | b
        }
    }
}

pub fn calc_pixel(x1: f64, y1: f64, iterations: u32) -> u32 {
    if is_in_cardioid(x1, y1) || is_in_bulb(x1, y1) { return 0; }

    let mut x2 = 0.0;
    let mut y2 = 0.0;
    let mut w = 0.0;
    let mut i = 0;

    while x2 + y2 <= 4.0 {
        let x = x2 - y2 + x1;
        let y = w - x2 - y2 + y1;
        x2 = x * x;
        y2 = y * y;
        w = (x + y) * (x + y);
        i += 1;
        if i > iterations { return 0; }
    }
    i
}

fn is_in_cardioid(x0: f64, y0: f64) -> bool {
    let q = (x0 - 0.25).powi(2) + y0.powi(2);
    q * (q + (x0 - 0.25)) < 0.25 * y0.powi(2)
}

fn is_in_bulb(x0: f64, y0: f64) -> bool {
    (x0 + 1.0).powi(2) + y0.powi(2) < 0.0625
}

pub fn save_image(buffer: Vec<u32>, width: u32, height: u32, file_path: &str, oversample: u32) -> Result<(), image::ImageError> {
    // Create an ImageBuffer from the u32 buffer
    let image_buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = buffer[(y * width + x) as usize];
        Rgba([(pixel >> 16) as u8, (pixel >> 8) as u8, pixel as u8, 255])
    });

    // Convert ImageBuffer to DynamicImage
    let mut dynamic_image: DynamicImage = DynamicImage::ImageRgba8(image_buffer);

    dynamic_image = dynamic_image.resize(width / oversample, height / oversample, image::imageops::FilterType::Lanczos3);

    // Save the DynamicImage as PNG
    dynamic_image.save(file_path)
}