pub const ITERATIONS: u32 = 2u32.pow(10);
pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;
pub const ZOOM_FACTOR: f64 = 0.06;
pub const PAN_FACTOR: f64 = 0.1;
pub const FRAME_DURATION_NS: u32 = (1.0 / 60.0 * 1_000_000_000.0) as u32;
pub const IMAGE_PATH: &str = "screenshots/mandel.png";