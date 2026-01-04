use rand::Rng;
use raspberrypi_utils::LedString;
use raspberrypi_utils::urgb_u32;

// Reference: https://github.com/raspberrypi/utils/blob/master/piolib/examples/ws2812.c

fn main() {
    println!("WS281x");
    let mut leds = LedString::new(18, None, true).expect("Failed to initialize WS2812 LED strip");

    let num_pixels = 14_u32;

    let dir = 1; // 1 or -1
    let mut t = 0;
    for _ in 0..100 {
        pattern_snakes(&mut leds, num_pixels, t);
        std::thread::sleep(std::time::Duration::from_millis(10));
        t += dir;
    }

    t = 0;
    for _ in 0..100 {
        pattern_random(&mut leds, num_pixels, t);
        std::thread::sleep(std::time::Duration::from_millis(10));
        t += dir;
    }

    t = 0;
    for _ in 0..100 {
        pattern_sparkle(&mut leds, num_pixels, t);
        std::thread::sleep(std::time::Duration::from_millis(10));
        t += dir;
    }

    t = 0;
    for _ in 0..100 {
        pattern_greys(&mut leds, num_pixels, t);
        std::thread::sleep(std::time::Duration::from_millis(10));
        t += dir;
    }
}

fn pattern_snakes(leds: &mut LedString, len: u32, t: u32) {
    for i in 0..len {
        let x = (i + (t >> 1)) % 64;
        if x < 10 {
            leds.put_pixel(urgb_u32(0xff, 0, 0));
        } else if x >= 15 && x < 25 {
            leds.put_pixel(urgb_u32(0, 0xff, 0));
        } else if x >= 30 && x < 40 {
            leds.put_pixel(urgb_u32(0, 0, 0xff));
        } else {
            leds.put_pixel(0);
        }
    }
}

fn pattern_random(leds: &mut LedString, len: u32, t: u32) {
    if t % 8 != 0 {
        return;
    }

    let mut rng = rand::rng();

    for _ in 0..len {
        leds.put_pixel(rng.random::<u32>());
    }
}

fn pattern_sparkle(leds: &mut LedString, len: u32, t: u32) {
    if t % 8 != 0 {
        return;
    }

    let mut rng = rand::rng();

    for _ in 0..len {
        leds.put_pixel(if rng.random::<u32>() % 16 != 0 { 0 } else { 0xffffffff });
    }
}

fn pattern_greys(leds: &mut LedString, len: u32, mut t: u32) {
    let max: u32 = 100; // let's not draw too much current!
    t %= max;
    for _ in 0..len {
        leds.put_pixel(t * 0x10101);

        if {
            t += 1;
            t >= max
        } {
            t = 0;
        }
    }
}
