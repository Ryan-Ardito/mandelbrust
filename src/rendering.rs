use rayon::prelude::*;

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

    // HOT loop
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