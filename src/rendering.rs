use rayon::prelude::*;
use image::{DynamicImage, ImageBuffer, Rgba, imageops::FilterType};

#[inline(always)]
fn is_in_cardioid_or_bulb(x_pos: f64, y_pos: f64) -> bool {
    let y2 = y_pos.powi(2);
    let q = (x_pos - 0.25).powi(2) + y2;
    let in_cardioid = q * (q + (x_pos - 0.25)) < 0.25 * y2;
    let in_bulb = (x_pos + 1.0).powi(2) + y2 < 0.0625;
    in_cardioid || in_bulb
}

#[inline(always)]
pub fn calc_pixel(x_pos: f64, y_pos: f64, iterations: u32) -> u32 {
    if is_in_cardioid_or_bulb(x_pos, y_pos) { return 0; }

    let mut x2 = 0.0;
    let mut y2 = 0.0;
    let mut w = 0.0;
    let mut i = 0;

    while x2 + y2 <= 4.0 {
        if i >= iterations { return 0; }
        let x = x2 - y2 + x_pos;
        let y = w - x2 - y2 + y_pos;
        x2 = x.powi(2);
        y2 = y.powi(2);
        w = (x + y).powi(2);
        i += 1;
    }

    i
}

pub fn render(
    x_pos: f64,
    y_pos: f64,
    width: usize,
    height: usize,
    base_view_height: f64,
    zoom: f64,
    iterations: u32,
) -> Vec<u32> {
    let x_min = x_pos - (2.0 / zoom);
    let x_max = x_pos + (2.0 / zoom);
    let y_min = y_pos - (base_view_height / zoom);
    let y_max = y_pos + (base_view_height / zoom);
    let x_exp = (x_max - x_min) / (width - 1) as f64;
    let y_exp = (y_max - y_min) / (height - 1) as f64;

    let mut buffer = vec![0; width * height];

    buffer
        .par_chunks_exact_mut(width)
        .enumerate()
        .for_each(|(y, row)| {
            let y_scaled = y_min + (y as f64) * y_exp;

            let mut x_scaled = x_min + x_exp;
            for pixel in row.iter_mut() {
                *pixel = calc_pixel(x_scaled, y_scaled, iterations);
                x_scaled += x_exp;
            }
        });

    buffer
}

pub fn scale_image(image: Vec<u32>, width: usize, height: usize) -> Vec<u32> {
    let new_width = width * 4;
    let new_height = height * 4;
    let mut scaled_image = vec![0; new_width * new_height];

    for y in 0..height {
        for x in 0..width {
            let orig_pixel = image[y * width + x];
            let dest_x = x * 4;
            let dest_y = y * 4;
            let dest_index = dest_y * new_width + dest_x;

            // Copy the original pixel to the 16 pixels in the scaled image
            for dy in 0..4 {
                for dx in 0..4 {
                    scaled_image[dest_index + dy * new_width + dx] = orig_pixel;
                }
            }
        }
    }

    scaled_image
}

pub fn save_image(
    buffer: Vec<u32>,
    width: u32,
    height: u32,
    file_path: &str,
    oversample: u32,
) -> image::ImageResult<()> {
    // Create an ImageBuffer from the u32 buffer
    let image_buffer = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = buffer[(y * width + x) as usize];
        Rgba([(pixel >> 16) as u8, (pixel >> 8) as u8, pixel as u8, 255])
    });

    // Convert ImageBuffer to DynamicImage
    let mut dynamic_image: DynamicImage = DynamicImage::ImageRgba8(image_buffer);

    // Sample down
    if oversample > 1 {
        dynamic_image = dynamic_image.resize(
            width / oversample,
            height / oversample,
            FilterType::Lanczos3,
        );
    }

    // Save the DynamicImage as PNG
    dynamic_image.save(file_path)
}