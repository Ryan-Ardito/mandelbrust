use rayon::prelude::*;

// Main calculation. Return the number of iterations taken to leave the bounds.
// Return 0 if bounds not left. 0 represents 'in set' (up to iterations.)
#[inline(always)]
pub fn calc_pixel(x_pos: f64, y_pos: f64, iterations: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;

    // HOT loop
    for i in 0..iterations {
        if x2 + y2 > 4.0 { return i; }

        let x_temp = x2 - y2 + x_pos;
        y = 2.0 * x * y + y_pos;
        x = x_temp;
        x2 = x * x;
        y2 = y * y;
    }

    0
}

// Helper function to skip iterating the largest portions of the set.
// Major speedups when areas covered are in frame. ~4% slowdown when not.
#[inline(always)]
fn is_in_cardioid_or_bulb(x_pos: f64, y_pos: f64) -> bool {
    let y2 = y_pos.powi(2);
    let q = (x_pos - 0.25).powi(2) + y2;
    let in_cardioid = q * (q + (x_pos - 0.25)) < 0.25 * y2;
    let in_bulb = (x_pos + 1.0).powi(2) + y2 < 0.0625;
    in_cardioid || in_bulb
}

// Return a Vec<u32> buffer representing iterations reached for each pixel.
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
                *pixel = match is_in_cardioid_or_bulb(x_scaled, y_scaled) {
                    true => 0,
                    false => calc_pixel(x_scaled, y_scaled, iterations),
                };
                x_scaled += x_exp;
            }
        });

    buffer
}