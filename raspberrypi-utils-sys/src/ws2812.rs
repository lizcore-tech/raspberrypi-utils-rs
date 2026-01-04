use crate::*;

const WS2812_WRAP_TARGET: u32 = 0;
const WS2812_WRAP: u32 = 3;

const WS2812_T1: u32 = 3;
const WS2812_T2: u32 = 4;
const WS2812_T3: u32 = 3;
const FREQ: u32 = 800_000;

pub struct Ws2812 {
    pio: *mut pio_instance,
    sm: u32,
    gpio_pin: u32,
    offset: u32,
}

// extern "C" {
//     // These match the symbols the linker was complaining about.
//     // We declare them here to ensure Rust knows they are external.
//     fn stdio_init_all();
//     fn pio_claim_unused_sm(pio: *mut pio_instance, required: bool) -> i32;
//     fn pio_add_program(pio: *mut pio_instance, program: *const pio_program) -> u32;
// }

impl Ws2812 {
    pub fn new(gpio: u32) -> Result<Self, String> {
        unsafe {
            let pio0: PIO = pio_open_helper(0); // Already defined by C library

            stdio_init_all();

            // In some bindings, pio0 is a pointer, in others a macro.
            // We use the pio_hw_t pointer from the lib.
            let pio = pio0;

            let sm = pio_claim_unused_sm(pio, true);

            if sm < 0 {
                return Err("No unused state machine available".to_string());
            }

            let pio_program_instructions: [u16; 4] = [
                //     .wrap_target
                0x6221, //  0: out    x, 1            side 0 [2]
                0x1223, //  1: jmp    !x, 3           side 1 [2]
                0x1300, //  2: jmp    0               side 1 [3]
                0xa342, //  3: nop                    side 0 [3]
                        //     .wrap
            ];
            let ws2812_program = pio_program {
                instructions: pio_program_instructions.as_ptr(),
                length: 4,
                origin: -1,
                pio_version: 0,
            };

            let offset = pio_add_program(pio, &ws2812_program);

            println!("WS2812, using pin {:?}", gpio);
            println!("Loaded program at {:?}, using sm {:?}", offset, sm);

            // You'll need to link the ws2812.pio.h equivalent logic here
            // Often pio programs are converted to headers with a struct
            // For now, this is the structural equivalent of your C main()

            Ok(Self {
                pio,
                sm: sm as u32,
                gpio_pin: gpio,
                offset,
            })
        }
    }

    fn program_get_default_config(&self) -> pio_sm_config {
        unsafe {
            let mut c = pio_get_default_sm_config();
            sm_config_set_wrap(
                &mut c,
                self.offset + WS2812_WRAP_TARGET,
                self.offset + WS2812_WRAP,
            );
            sm_config_set_sideset(&mut c, 1, false, false);

            c
        }
    }

    pub fn program_init(&self, rgbw: bool) {
        unsafe {
            pio_gpio_init(self.pio, self.gpio_pin);
            pio_sm_set_consecutive_pindirs(self.pio, self.sm, self.gpio_pin, 1, true);
            let mut c: pio_sm_config = self.program_get_default_config();
            sm_config_set_sideset_pins(&mut c, self.gpio_pin);
            sm_config_set_out_shift(&mut c, false, true, if rgbw { 32 } else { 24 });
            sm_config_set_fifo_join(&mut c, pio_fifo_join_PIO_FIFO_JOIN_TX);
            let cycles_per_bit = WS2812_T1 + WS2812_T2 + WS2812_T3;
            let div = clock_get_hz(clock_index_clk_sys) as f32 / (FREQ * cycles_per_bit) as f32;
            sm_config_set_clkdiv(&mut c, div);
            pio_sm_init(self.pio, self.sm, self.offset, &mut c);
            pio_sm_set_enabled(self.pio, self.sm, true);
        }
    }

    fn pattern_snakes(&self, len: u32, t: u32) {
        for i in 0..len {
            let x = (i + (t >> 1)) % 64;
            if x < 10 {
                self.put_pixel(urgb_u32(0xff, 0, 0));
            } else if x >= 15 && x < 25 {
                self.put_pixel(urgb_u32(0, 0xff, 0));
            } else if x >= 30 && x < 40 {
                self.put_pixel(urgb_u32(0, 0, 0xff));
            } else {
                self.put_pixel(0);
            }
        }
    }

    pub fn program_run(&self) {
        let num_pixels = 150;
        let dir = 1;
        let mut t = 0;
        for _ in 0..100 {
            self.pattern_snakes(num_pixels, t);
            std::thread::sleep(std::time::Duration::from_millis(10));
            t += dir;
        }
    }

    pub fn put_pixel(&self, pixel_grb: u32) {
        unsafe {
            pio_sm_put_blocking(self.pio, self.sm, pixel_grb << 8);
        }
    }
}

pub fn urgb_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 8) | ((g as u32) << 16) | (b as u32)
}
