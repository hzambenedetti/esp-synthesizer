use esp_hal::{
    analog::adc::{Adc, AdcConfig},
    gpio::{GpioPin, Input, Pull},
    peripherals::{ADC1, ADC2},
};

use crate::analog_reader::{AnalogReader, AnalogReaderAdc2};

pub struct InputPins {
    pub decay_rate_pin: GpioPin<1>,
    pub osc_mix: GpioPin<2>,
    pub osc2_pitch_pin: GpioPin<3>,
    pub sustain_level_pin: GpioPin<6>,
    pub release_rate_pin: GpioPin<7>,
    pub osc1_pitch_pin: GpioPin<8>,
    pub attack_rate_pin: GpioPin<9>,
    pub lfo_freq_pin: GpioPin<10>,
    pub lfo_amp_pin: GpioPin<12>,
    pub lfo_waveform_pin: GpioPin<39>,
    pub osc1_waveform_pin: GpioPin<41>,
    pub osc2_waveform_pin: GpioPin<42>,
}
pub struct Read {
    pub osc2_pitch: f32,
    pub release_rate: f32,
    pub osc1_pitch: f32,
    pub attack_rate: f32,
    pub sustain_level: f32,
    pub decay_rate: f32,
    pub osc1_waveform: bool,
    pub osc2_waveform: bool,
    pub osc_mix: f32,
    pub lfo_freq: f32,
    pub lfo_amp: f32,
    pub lfo_waveform: bool,
}

pub struct SynthInputs<'a> {
    osc2_pitch: AnalogReader<GpioPin<3>, fn(u16) -> f32>,
    sustain_level: AnalogReader<GpioPin<6>, fn(u16) -> f32>,
    release_rate: AnalogReader<GpioPin<7>, fn(u16) -> f32>,
    osc1_pitch: AnalogReader<GpioPin<8>, fn(u16) -> f32>,
    attack_rate: AnalogReader<GpioPin<9>, fn(u16) -> f32>,
    decay_rate: AnalogReader<GpioPin<1>, fn(u16) -> f32>,
    osc_mix: AnalogReader<GpioPin<2>, fn(u16) -> f32>,
    lfo_freq: AnalogReader<GpioPin<10>, fn(u16) -> f32>,
    lfo_amp: AnalogReaderAdc2<GpioPin<12>, fn(u16) -> f32>,
    osc1_waveform: Input<'a>,
    osc2_waveform: Input<'a>,
    lfo_waveform: Input<'a>,
    adc1_driver: Adc<'a, ADC1>,
    adc2_driver: Adc<'a, ADC2>,
}
fn mix_activation(x: u16) -> f32 {
    f32::max(f32::min(x as f32 * (1.0 / 3000.0), 1.0), 0.0)
}
fn pitch_activation(x: u16) -> f32 {
    x as f32 * (1.5 / 3000.0) + 0.5
}
fn attack_activation(x: u16) -> f32 {
    x as f32 * (1.0 / 300.0) + 0.2
}
fn release_activation(x: u16) -> f32 {
    -1.0 * (x as f32 * (1.0 / 3000.0))
}
fn sustain_activation(x: u16) -> f32 {
    x as f32 * (1.0 / 3000.0)
}
fn lfo_amp_activation(x: u16) -> f32 {
    f32::max(x as f32 * (2.0 / 3000.0) - 0.1, 0.0)
}
fn lfo_freq_activation(x: u16) -> f32 {
    x as f32 + 25.0
}

impl<'a> SynthInputs<'a> {
    pub fn new(
        pins: InputPins,
        mut adc1_config: AdcConfig<ADC1>,
        adc1: ADC1,
        mut adc2_config: AdcConfig<ADC2>,
        adc2: ADC2,
    ) -> SynthInputs<'a> {
        let osc1_pitch: AnalogReader<GpioPin<8>, fn(u16) -> f32> =
            AnalogReader::new(pins.osc1_pitch_pin, pitch_activation, &mut adc1_config);
        let osc2_pitch: AnalogReader<GpioPin<3>, fn(u16) -> f32> =
            AnalogReader::new(pins.osc2_pitch_pin, pitch_activation, &mut adc1_config);
        let attack_rate: AnalogReader<GpioPin<9>, fn(u16) -> f32> =
            AnalogReader::new(pins.attack_rate_pin, attack_activation, &mut adc1_config);
        let release_rate: AnalogReader<GpioPin<7>, fn(u16) -> f32> =
            AnalogReader::new(pins.release_rate_pin, release_activation, &mut adc1_config);
        let sustain_level: AnalogReader<GpioPin<6>, fn(u16) -> f32> =
            AnalogReader::new(pins.sustain_level_pin, sustain_activation, &mut adc1_config);
        let decay_rate: AnalogReader<GpioPin<1>, fn(u16) -> f32> =
            AnalogReader::new(pins.decay_rate_pin, release_activation, &mut adc1_config);
        let osc_mix: AnalogReader<GpioPin<2>, fn(u16) -> f32> =
            AnalogReader::new(pins.osc_mix, mix_activation, &mut adc1_config);
        let lfo_freq: AnalogReader<GpioPin<10>, fn(u16) -> f32> =
            AnalogReader::new(pins.lfo_freq_pin, lfo_freq_activation, &mut adc1_config);
        let lfo_amp: AnalogReaderAdc2<GpioPin<12>, fn(u16) -> f32> =
            AnalogReaderAdc2::new(pins.lfo_amp_pin, lfo_amp_activation, &mut adc2_config);
        let adc1_driver = Adc::new(adc1, adc1_config);
        let adc2_driver = Adc::new(adc2, adc2_config);
        let osc1_waveform = pins.osc1_waveform_pin;
        let osc1_waveform = Input::new(osc1_waveform, Pull::Up);
        let osc2_waveform = pins.osc2_waveform_pin;
        let osc2_waveform = Input::new(osc2_waveform, Pull::Up);
        let lfo_waveform = pins.lfo_waveform_pin;
        let lfo_waveform = Input::new(lfo_waveform, Pull::Up);
        SynthInputs {
            osc1_pitch,
            release_rate,
            osc2_pitch,
            attack_rate,
            sustain_level,
            adc1_driver,
            adc2_driver,
            decay_rate,
            osc_mix,
            osc1_waveform,
            osc2_waveform,
            lfo_waveform,
            lfo_freq,
            lfo_amp,
        }
    }
    pub fn read_all(&mut self) -> Read {
        Read {
            osc1_pitch: self.osc1_pitch.read(&mut self.adc1_driver),
            osc2_pitch: self.osc2_pitch.read(&mut self.adc1_driver),
            attack_rate: self.attack_rate.read(&mut self.adc1_driver),
            osc1_waveform: self.osc1_waveform.is_low(),
            osc2_waveform: self.osc2_waveform.is_low(),
            release_rate: self.release_rate.read(&mut self.adc1_driver),
            sustain_level: self.sustain_level.read(&mut self.adc1_driver),
            decay_rate: self.decay_rate.read(&mut self.adc1_driver),
            osc_mix: self.osc_mix.read(&mut self.adc1_driver),
            lfo_waveform: self.lfo_waveform.is_low(),
            lfo_freq: self.lfo_freq.read(&mut self.adc1_driver),
            lfo_amp: self.lfo_amp.read(&mut self.adc2_driver),
        }
    }
}
