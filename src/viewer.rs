use crate::constants::ITERATIONS;
use crate::rendering::{render, MetaData};
use crate::imaging::{PostProc, screenshot, upscale_buffer};

#[derive(Debug, Clone, Copy)]
pub struct Viewer {
    pub width: usize,
    pub height: usize,
    pub iterations: u32,
    downsample_exp: u32,
    x_pos: f64,
    y_pos: f64,
    zoom: f64,
}

impl Viewer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            iterations: ITERATIONS,
            downsample_exp: 2,
            x_pos: -0.5,
            y_pos: 0.0,
            zoom: 1.0,
        }
    }

    pub fn view_buffer(&mut self) -> Vec<u32> {
        let data = MetaData::new(
            self.width,
            self.height,
            self.x_pos,
            self.y_pos,
            self.zoom,
            self.iterations,
        );
        render(data)
    }

    pub fn low_res_view_buffer(&mut self) -> Vec<u32> {
        let downsample_scale = 2usize.pow(self.downsample_exp);
        let width = self.width / downsample_scale;
        let height = self.height / downsample_scale;
        let iterations = self.iterations / 2;

        let data = MetaData::new(
            width,
            height,
            self.x_pos,
            self.y_pos,
            self.zoom,
            iterations,
        );
        let buffer = render(data);

        upscale_buffer(buffer, width, height, downsample_scale)
    }

    pub fn screenshot(
        &self,
        width: usize,
        height: usize,
        oversample: u32,
        post_proc: PostProc,
    ) {
        let data = MetaData::new(
            width * oversample as usize,
            height * oversample as usize,
            self.x_pos,
            self.y_pos,
            self.zoom,
            self.iterations,
        );

        screenshot(
            data,
            oversample,
            post_proc
        )
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.width, self.height);
    }

    pub fn iter_up(&mut self) {
        self.iterations *= 2;
        println!("Iterations: {}", self.iterations);
    }

    pub fn iter_down(&mut self) {
        if self.iterations > 1 { self.iterations /= 2; }
        println!("Iterations: {}", self.iterations);
    }

    pub fn downsample_up(&mut self) {
        if self.downsample_exp < 4 {
            self.downsample_exp += 1;
        }
    }

    pub fn downsample_down(&mut self) {
        if self.downsample_exp > 0 {
            self.downsample_exp -= 1;
        }
    }

    pub fn zoom(&mut self, factor: f64) {
        // limit zooming out
        if self.zoom * factor < 0.4 { return; }
        self.zoom *= factor;
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        // bound movement to bounds of the set
        if self.out_of_bounds(dx, dy) { return; }
        self.x_pos += dx / self.zoom;
        self.y_pos += dy / self.zoom;
    }

    fn out_of_bounds(&self, dx: f64, dy: f64) -> bool {
        self.x_pos + dx / self.zoom < -2.0
        || self.x_pos + dx / self.zoom > 2.0
        || self.y_pos + dy / self.zoom < -2.0
        || self.y_pos + dy / self.zoom > 2.0
    }
}