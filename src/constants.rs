use crate::rendering::Float;

pub const ITERATIONS: u32 = 2u32.pow(10);
pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;
pub const ZOOM_FACTOR: Float = 0.05;
pub const PAN_FACTOR: Float = 0.06;
pub const FRAME_DURATION_NS: u64 = (1.0 / 60.0 * 1_000_000_000.0) as u64;
pub const IMAGE_PATH: &str = "screenshots/mandel.png";