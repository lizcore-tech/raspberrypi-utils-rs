use raspberrypi_utils_sys::Ws2812;

pub struct Leds {
    runner: Ws2812,
}

impl Leds {
    pub fn new(gpio: u32) -> Result<Self, String> {
        let mut runner = Ws2812::new(gpio)?;
        
        Ok(Leds { runner })
    }

    pub fn program_init(&self, rgbw: bool) {
        println!("Program Init");
    }

    pub fn program_run(&self) {
        println!("Running");
    }
}
