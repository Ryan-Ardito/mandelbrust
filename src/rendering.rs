use rayon::prelude::*;

pub type Float = f64;

// approximation range for cycle detection
const PERIODICITY_THRESHOLD: Float = 1e-9;
const CYCLE_DETECTION_DELAY: u32 = 40;

#[derive(Debug, Clone, Copy)]
pub struct MetaData {
    pub width: usize,
    pub height: usize,
    pub x_pos: Float,
    pub y_pos: Float,
    pub zoom: Float,
    pub iterations: u32,
}

impl MetaData {
    pub fn new(
        width: usize,
        height: usize,
        x_pos: Float,
        y_pos: Float,
        zoom: Float,
        iterations: u32,
    ) -> Self {
        Self { width, height, x_pos, y_pos, zoom, iterations }
    }
}

/// Calculate the escape time for a given position in the Mandelbrot set.
/// Return the number of iterations taken to leave the bounds.
/// Return 0 if bounds not left (representing 'in set' up to `iterations`).
#[inline(always)]
fn escape_time(x_pos: Float, y_pos: Float, iterations: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;

    let mut x_old = 0.0;
    let mut y_old = 0.0;

    // HOT loop
    let mut i = 0;
    while i < iterations {
        // sub loop to avoid branching in cycle detection, reduce loop overhead
        // 10 in sub loop * 2 in unroll = coords stored every 20 iterations
        for _ in 0..10 {
            // ~5% faster on my machine with this unroll
            for _ in 0..2 {
                if x2 + y2 > 4.0 { return i; }

                y = (x + x) * y + y_pos;
                x = x2 - y2 + x_pos;
                x2 = x * x;
                y2 = y * y;

                i += 1;
            }

            // cycle detection
            if i >= CYCLE_DETECTION_DELAY {
                let x_visited = (x - x_old).abs() < PERIODICITY_THRESHOLD;
                let y_visited = (y - y_old).abs() < PERIODICITY_THRESHOLD;

                if x_visited && y_visited { return 0; }
            }
        }
        // store visited values
        x_old = x;
        y_old = y;
    }

    0
}

// Helper function to skip iterating the largest portions of the set.
// Major speedups when areas covered are in frame. ~2% slower when not.
fn is_in_cardioid_or_bulb(x_pos: Float, y_pos: Float) -> bool {
    let y2 = y_pos.powi(2);
    let q = (x_pos - 0.25).powi(2) + y2;
    let in_cardioid = q * (q + (x_pos - 0.25)) < 0.25 * y2;
    let in_bulb = (x_pos + 1.0).powi(2) + y2 < 0.0625;
    in_cardioid || in_bulb
}

fn calc_pixel(x_pos: Float, y_pos: Float, iterations: u32) -> u32 {
    match is_in_cardioid_or_bulb(x_pos, y_pos) {
        true => 0,
        false => escape_time(x_pos, y_pos, iterations),
    }
}

// Return a Vec<u32> buffer representing iterations reached for each pixel.
pub fn render(data: MetaData) -> Vec<u32> {
    let base_view_height = 2.0 / (data.width as Float / data.height as Float);

    let x_min = data.x_pos - (2.0 / data.zoom);
    let y_min = data.y_pos - (base_view_height / data.zoom);

    let pixel_width = 4.0 / (data.zoom * (data.width - 1) as Float);

    let mut buffer = vec![0; data.width * data.height];

    buffer
        .par_chunks_exact_mut(data.width)
        .enumerate()
        .for_each(|(y, row)| {
            let y_curr = y_min + (y as Float) * pixel_width;

            let mut x_curr = x_min;
            for pixel in row.iter_mut() {
                *pixel = calc_pixel(x_curr, y_curr, data.iterations);
                x_curr += pixel_width;
            }
        });

    buffer
}