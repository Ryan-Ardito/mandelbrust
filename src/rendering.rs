use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct MetaData {
    pub width: usize,
    pub height: usize,
    pub x_pos: f64,
    pub y_pos: f64,
    pub zoom: f64,
    pub iterations: u32,
}

impl MetaData {
    pub fn new(
        width: usize,
        height: usize,
        x_pos: f64,
        y_pos: f64,
        zoom: f64,
        iterations: u32,
    ) -> Self {
        Self { width, height, x_pos, y_pos, zoom, iterations }
    }
}

/// Calculate the escape time for a given pixel in the Mandelbrot set.
/// Return the number of iterations taken to leave the bounds.
/// Return 0 if bounds not left (representing 'in set' up to `iterations`).
#[inline(always)]
pub fn escape_time(x_pos: f64, y_pos: f64, iterations: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;

    // HOT loop
    for i in 0..iterations {
        if x2 + y2 > 4.0 { return i; }

        y = (x + x) * y + y_pos;
        x = x2 - y2 + x_pos;
        x2 = x * x;
        y2 = y * y;
    }

    0
}

// Helper function to skip iterating the largest portions of the set.
// Major speedups when areas covered are in frame. ~2% slower when not.
#[inline(always)]
fn is_in_cardioid_or_bulb(x_pos: f64, y_pos: f64) -> bool {
    let y2 = y_pos.powi(2);
    let q = (x_pos - 0.25).powi(2) + y2;
    let in_cardioid = q * (q + (x_pos - 0.25)) < 0.25 * y2;
    let in_bulb = (x_pos + 1.0).powi(2) + y2 < 0.0625;
    in_cardioid || in_bulb
}

// Return a Vec<u32> buffer representing iterations reached for each pixel.
pub fn render(data: MetaData) -> Vec<u32> {
    let base_view_height = 2.0 / (data.width as f64 / data.height as f64);
    let x_min = data.x_pos - (2.0 / data.zoom);
    let x_max = data.x_pos + (2.0 / data.zoom);
    let y_min = data.y_pos - (base_view_height / data.zoom);
    let y_max = data.y_pos + (base_view_height / data.zoom);
    let x_exp = (x_max - x_min) / (data.width - 1) as f64;
    let y_exp = (y_max - y_min) / (data.height - 1) as f64;

    let mut buffer = vec![0; data.width * data.height];

    buffer
        .par_chunks_exact_mut(data.width)
        .enumerate()
        .for_each(|(y, row)| {
            let y_scaled = y_min + (y as f64) * y_exp;

            let mut x_scaled = x_min + x_exp;
            for pixel in row.iter_mut() {
                *pixel = match is_in_cardioid_or_bulb(x_scaled, y_scaled) {
                    true => 0,
                    false => escape_time(x_scaled, y_scaled, data.iterations),
                };
                x_scaled += x_exp;
            }
        });

    buffer
}