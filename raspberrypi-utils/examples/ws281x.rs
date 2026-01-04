use raspberrypi_utils::Ws2812;

fn main() {
    println!("WS281x");
    let leds = Ws2812::new(18).expect("Failed to initialize WS2812 LED strip");
    leds.program_init(true);
    leds.program_run();
}
