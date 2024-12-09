/*======================================= IMPORTS =======================================*/
#![no_std]
#![no_main]

use analog_reader::AnalogReader;
#[allow(unused)]
use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig},
    dma::{Dma, DmaPriority},
    dma_circular_buffers,
    gpio::{Input, Io, Pull},
    i2s::{DataFormat, I2s, I2sTx, I2sWriteDma, Standard},
    peripherals::{ADC1, I2S0},
    prelude::*,
    Blocking,
};

use crate::envelope::Envelope;
use crate::oscilator::{Oscilator, WaveForm};

/*======================================= MODULES =======================================*/

mod analog_reader;
mod envelope;
mod oscilator;
mod wave;

/*======================================= CONSTANTS =======================================*/

const SAMPLING_RATE: u32 = 48000;
const TX_BUFFER_SIZE: usize = 4096;
// Se STEP aumenta, a frequencia aumenta tambem
// Se o número de amostras no seno aumenta a frequencia diminui

/*======================================= MAIN =======================================*/

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut adc1_config: AdcConfig<ADC1> = AdcConfig::new();
    let mut pitch1_reader = AnalogReader::new(
        io.pins.gpio8,
        |x| x as f32 * (1.5 / 3000.0) + 0.5,
        &mut adc1_config,
    );
    let mut pitch2_reader = AnalogReader::new(
        io.pins.gpio3,
        |x| x as f32 * (1.5 / 3000.0) + 0.5,
        &mut adc1_config,
    );
    let mut adc1_driver = Adc::new(peripherals.ADC1, adc1_config);

    let button = io.pins.gpio10;
    let button = Input::new(button, Pull::Up);

    //I2S and DMA setup
    let dma = Dma::new(peripherals.DMA);
    let dma_channel = dma.channel0;

    let (_, rx_descriptors, tx_buffer, tx_descriptors) = dma_circular_buffers!(0, TX_BUFFER_SIZE);

    let i2s = I2s::new(
        peripherals.I2S0,
        Standard::Philips,
        DataFormat::Data16Channel16,
        SAMPLING_RATE.Hz(),
        dma_channel.configure(false, DmaPriority::Priority0),
        rx_descriptors,
        tx_descriptors,
    );

    let mut i2s_tx: I2sTx<I2S0, Blocking> = i2s
        .i2s_tx
        .with_bclk(io.pins.gpio4)
        .with_ws(io.pins.gpio5)
        .with_dout(io.pins.gpio15)
        .build();

    let mut signal_buffer = [0i16; TX_BUFFER_SIZE];
    let mut env_buffer = [0.; TX_BUFFER_SIZE];
    let mut osc1_buffer = [0i16; TX_BUFFER_SIZE];
    let mut osc2_buffer = [0i16; TX_BUFFER_SIZE];
    let mut oscilator1 = Oscilator::new(60.0, 60.0, 10_000.0, WaveForm::Sine);
    let mut oscilator2 = Oscilator::new(60.0, 60.0, 10_000.0, WaveForm::Square);
    let mut envelope = Envelope::new(1.0, -1.0, 0.4, -0.5).unwrap();
    let osc1_mix = 0.5;
    let osc2_mix = 0.5;

    let mut transfer = i2s_tx.write_dma_circular(&tx_buffer).unwrap();
    let mut filler = [0u8; TX_BUFFER_SIZE];
    let base_freq = 130.8;

    let mut freq1_offset = pitch1_reader.read(&mut adc1_driver);
    let mut freq2_offset = pitch2_reader.read(&mut adc1_driver);
    let mut adc_counter: u32 = 0;
    oscilator1.set_frequency(base_freq * freq1_offset);
    oscilator2.set_frequency(base_freq * freq2_offset);
    let mut gate = false;
    loop {
        adc_counter += 1;
        if button.is_low() && !gate {
            envelope.trigger();
            gate = true;
        }
        if button.is_high() && gate {
            envelope.detrigger();
            gate = false;
        }
        let avail = transfer.available();
        if avail > 0 {
            let avail = usize::min(TX_BUFFER_SIZE, avail);
            oscilator1.gen_signal(&mut osc1_buffer, avail / 2, true);
            oscilator2.gen_signal(&mut osc2_buffer, avail / 2, true);
            envelope.gen_signal(&mut env_buffer, avail / 2);
            for i in 0..TX_BUFFER_SIZE {
                signal_buffer[i] = ((osc1_buffer[i] as f32 * osc1_mix
                    + osc2_buffer[i] as f32 * osc2_mix)
                    * env_buffer[i]) as i16;
            }
            copy_bytes(&signal_buffer, &mut filler, avail);
            transfer.push(&filler[0..avail]).unwrap();
        }

        if adc_counter > 100 {
            adc_counter = 0;
            freq1_offset = pitch1_reader.read(&mut adc1_driver);
            freq2_offset = pitch2_reader.read(&mut adc1_driver);
            oscilator1.set_frequency(base_freq * freq1_offset);
            oscilator2.set_frequency(base_freq * freq2_offset);
        }
    }
}

fn copy_bytes(signal_buffer: &[i16], filler: &mut [u8], size: usize) {
    let signal_buffer = unsafe {
        core::slice::from_raw_parts(
            signal_buffer as *const _ as *const u8,
            signal_buffer.len() * 2,
        )
    };

    filler[..size].copy_from_slice(&signal_buffer[..size]);
}
