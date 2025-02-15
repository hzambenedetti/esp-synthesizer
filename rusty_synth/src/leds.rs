use esp_hal::{
    delay::Delay,
    gpio::{GpioPin, Level, Output},
};

use crate::oscilator::WaveForm;

fn wf_to_bits(wf1: WaveForm, wf2: WaveForm, wf3: WaveForm) -> u16 {
    let mut output: u16 = 0;
    let wf1_bits = match wf1 {
        WaveForm::Sine => 1,
        WaveForm::Triangle => 2,
        WaveForm::Square => 4,
        WaveForm::SawTooth => 8,
    };
    let wf2_bits = match wf2 {
        WaveForm::Sine => 1,
        WaveForm::Triangle => 2,
        WaveForm::Square => 4,
        WaveForm::SawTooth => 8,
    };
    let wf3_bits = match wf3 {
        WaveForm::Sine => 1,
        WaveForm::Triangle => 2,
        WaveForm::Square => 4,
        WaveForm::SawTooth => 8,
    };
    output |= wf1_bits;
    output |= wf2_bits << 4;
    output |= wf3_bits << 8;
    output
}

pub struct Leds<'a> {
    ds: Output<'a>,
    stcp: Output<'a>,
    shcp: Output<'a>,
    delay: Delay,
}

impl<'a> Leds<'a> {
    pub fn new(ds_pin: GpioPin<36>, stcp_pin: GpioPin<37>, shcp_pin: GpioPin<38>) -> Leds<'a> {
        Leds {
            ds: Output::new(ds_pin, Level::Low),
            stcp: Output::new(stcp_pin, Level::Low),
            shcp: Output::new(shcp_pin, Level::Low),
            delay: Delay::new(),
        }
    }

    pub fn show(&mut self, wf1: WaveForm, wf2: WaveForm, wf3: WaveForm) {
        let output = wf_to_bits(wf1, wf2, wf3);
        // esp_println::println!("output: {output}");
        for i in (0..16).rev() {
            match (output >> i) & 1 {
                0 => self.ds.set_low(),
                1 => self.ds.set_high(),
                _ => (),
            }
            self.shcp.set_high();
            self.delay.delay_micros(1);
            self.shcp.set_low();
            self.delay.delay_micros(1);
        }
        self.stcp.set_high();
        self.delay.delay_micros(1);
        self.stcp.set_low();
        self.delay.delay_micros(1);
    }
}
