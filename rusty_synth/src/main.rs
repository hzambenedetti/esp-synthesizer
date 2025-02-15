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
    peripherals::{ADC1, ADC2, I2S0},
    prelude::*,
    uart::{
        self,
        config::{DataBits, Parity, StopBits},
        Uart,
    },
    Blocking,
};
use lfo::Lfo;
use log::info;
use midi_parser::{midi_note_to_freq, MidiEvent, MidiParser};
use synth_inputs::{InputPins, SynthInputs};

use crate::envelope::Envelope;
use crate::leds::Leds;
use crate::oscilator::{Oscilator, WaveForm};

/*======================================= MODULES =======================================*/

mod analog_reader;
mod envelope;
mod leds;
mod lfo;
mod midi_parser;
mod oscilator;
mod synth_inputs;
mod wave;

/*======================================= CONSTANTS =======================================*/

const SAMPLING_RATE: u32 = 48000;
const TX_BUFFER_SIZE: usize = 4096;
const MIDI_BAUD: u32 = 31_250;
const STATUS_NOTE_ON: u8 = 0x90;
const STATUS_NOTE_OFF: u8 = 0x80;
// Se STEP aumenta, a frequencia aumenta tambem
// Se o número de amostras no seno aumenta a frequencia diminui

/*======================================= MAIN =======================================*/

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
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

    let adc1_config: AdcConfig<ADC1> = AdcConfig::new();
    let adc2_config: AdcConfig<ADC2> = AdcConfig::new();
    let input_pins = InputPins {
        osc1_pitch_pin: io.pins.gpio8,
        osc2_pitch_pin: io.pins.gpio3,
        osc_mix: io.pins.gpio2,
        osc1_waveform_pin: io.pins.gpio41,
        osc2_waveform_pin: io.pins.gpio42,
        attack_rate_pin: io.pins.gpio9,
        decay_rate_pin: io.pins.gpio1,
        sustain_level_pin: io.pins.gpio6,
        release_rate_pin: io.pins.gpio7,
        lfo_amp_pin: io.pins.gpio12,
        lfo_freq_pin: io.pins.gpio10,
        lfo_waveform_pin: io.pins.gpio39,
    };

    // Configure UART1 for MIDI input
    let (mut tx_pin, mut rx_pin) = (io.pins.gpio43, io.pins.gpio44);

    let config = uart::config::Config::default()
        .baudrate(MIDI_BAUD)
        .data_bits(DataBits::DataBits8)
        .stop_bits(StopBits::STOP1)
        .parity_none();
    let mut uart = Uart::new_with_config(peripherals.UART0, config, rx_pin, tx_pin).unwrap();
    let mut parser = MidiParser::new();
    let mut buffer = [0x90, 0x30, 0x7F];
    let mut synth_inputs = SynthInputs::new(
        input_pins,
        adc1_config,
        peripherals.ADC1,
        adc2_config,
        peripherals.ADC2,
    );
    let (ds_pin, stcp_pin, shcp_pin) = (io.pins.gpio36, io.pins.gpio37, io.pins.gpio38);
    let mut leds = Leds::new(ds_pin, stcp_pin, shcp_pin);

    let mut signal_buffer = [0i16; TX_BUFFER_SIZE];
    let mut env_buffer = [0.; TX_BUFFER_SIZE];
    let mut osc1_buffer = [0i16; TX_BUFFER_SIZE];
    let mut osc2_buffer = [0i16; TX_BUFFER_SIZE];
    let mut oscilator1 = Oscilator::new(60.0, 60.0, 10_000.0, WaveForm::Sine);
    let mut oscilator2 = Oscilator::new(60.0, 60.0, 10_000.0, WaveForm::Sine);
    let mut lfo = Lfo::new(3000.0, 0.5, 1000.0, WaveForm::Sine);
    let mut envelope = Envelope::new(0.2, -1.0, 0.4, -0.5).unwrap();
    let mut osc1_mix = 0.5;
    let mut osc2_mix = 0.5;

    let mut transfer = i2s_tx.write_dma_circular(&tx_buffer).unwrap();
    let mut filler = [0u8; TX_BUFFER_SIZE];

    let mut adc_counter: u32 = 0;
    let mut gate = false;
    let mut osc1_waveform = false;
    let mut osc2_waveform = false;
    let mut lfo_waveform = false;
    let notes = [
        // 329.63, 329.63, 349.23, 392.00, 392.00, 349.23, 329.63, 293.66,
        // 261.63, 261.63, 293.66, 329.63, 329.63, 293.66, 293.66,
        659.25, 622.25, 659.25, 622.25, 659.25, 493.88, 587.33, 523.25, 440.00, 261.63, 329.63,
        440.00, 493.88, 329.63, 415.30, 493.88, 523.25, 659.25, 622.25, 659.25, 622.25, 659.25,
        493.88, 587.33, 523.25, 440.00, 261.63, 329.63, 440.00, 493.88, 329.63, 659.25,
    ];
    let mut base_freq = notes[0];
    let mut note_index = 0;
    let mut note_counter = 0;
    let mut new_gate = false;

    esp_println::logger::init_logger_from_env();
    loop {
        adc_counter += 1;
        if let Ok(byte) = uart.read_byte() {
            if let Some(event) = parser.parse_byte(byte) {
                match event {
                    MidiEvent::NoteOn { note, velocity } => {
                        base_freq = midi_note_to_freq(note);
                        new_gate = true;
                    }
                    MidiEvent::NoteOff { note } => {
                        new_gate = false;
                    }
                }
            }
        }
        if adc_counter > 100 {
            adc_counter = 0;
            let read = synth_inputs.read_all();
            lfo.set_frequency(read.lfo_freq);
            let lfo_offset = lfo.accquire();
            oscilator1
                .set_frequency((base_freq + 50.0 * lfo_offset * read.lfo_amp) * read.osc1_pitch);
            oscilator2
                .set_frequency((base_freq + 50.0 * lfo_offset * read.lfo_amp) * read.osc2_pitch);
            envelope.params.attack_rate = read.attack_rate;
            envelope.params.release_rate = read.release_rate;
            envelope.params.sustain_value = read.sustain_level;
            envelope.params.decay_rate = read.decay_rate;

            osc1_mix = read.osc_mix;
            osc2_mix = 1.0 - read.osc_mix;
            leds.show(oscilator1.wave_form, oscilator2.wave_form, lfo.wave_form);
            if new_gate && !gate {
                envelope.trigger();
                gate = true;
            }
            if !new_gate && gate {
                envelope.detrigger();
                gate = false;
            }
            if read.osc1_waveform && !osc1_waveform {
                oscilator1.next_waveform();
                osc1_waveform = true;
            }
            if !read.osc1_waveform && osc1_waveform {
                osc1_waveform = false;
            }
            if read.osc2_waveform && !osc2_waveform {
                oscilator2.next_waveform();
                osc2_waveform = true;
            }
            if !read.osc2_waveform && osc2_waveform {
                osc2_waveform = false;
            }
            if read.lfo_waveform && !lfo_waveform {
                info!("Changing LFO waveform");
                lfo.next_waveform();
                lfo_waveform = true;
            }
            if !read.lfo_waveform && lfo_waveform {
                lfo_waveform = false;
            }
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
