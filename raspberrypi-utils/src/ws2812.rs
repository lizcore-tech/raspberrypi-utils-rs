use raspberrypi_utils_sys::{Ws2812};

pub use raspberrypi_utils_sys::urgb_u32;

pub struct LedString {
    hw: Ws2812,
}

impl LedString {
    pub fn new(gpio: u32, frequency: Option<u32>, is_rgbw: bool) -> Result<Self, String> {
        let runner = Ws2812::new(gpio)?;

        runner.program_init(frequency, is_rgbw);

        Ok(LedString { hw: runner })
    }

    pub fn put_pixel(&mut self, pixel_grb: u32) {
        self.hw.put_pixel(pixel_grb);
    }

    pub fn put_pixel_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.hw.put_pixel(urgb_u32(r, g, b))
    }

    pub fn put_all_pixels(&mut self, pixel_grb: u32, num_pixels: u32) {
        for _ in 0..num_pixels {
            self.hw.put_pixel(pixel_grb);
        }
    }

    pub fn put_all_pixels_rgb(&mut self, r: u8, g: u8, b: u8, num_pixels: u32) {
        let pixel_grb = urgb_u32(r, g, b);

        self.put_all_pixels(pixel_grb, num_pixels);
    }
}
