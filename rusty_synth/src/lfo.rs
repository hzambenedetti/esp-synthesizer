use core::f32::consts::TAU;

use crate::{
    oscilator::{saw_tooth_i16, sin_i16, square_i16, triangle_i16, WaveForm},
    wave::constants::{FULL_WAVE, SINE},
    SAMPLING_RATE,
};

const MAX_VALUE: usize = 32_767;
const SAW_TOOTH_CONST: f32 = MAX_VALUE as f32 / FULL_WAVE as f32;
const TRIANGLE_CONST: f32 = 2.0 * SAW_TOOTH_CONST;

pub struct Lfo {
    phase: f32,
    step: f32,
    freq: f32,
    min_freq: f32,
    max_freq: f32,
    pub wave_form: WaveForm,
    wave_table: Option<&'static [i16; 1024]>,
}

#[allow(dead_code)]
impl Lfo {
    pub fn new(init_freq: f32, min_freq: f32, max_freq: f32, wave_form: WaveForm) -> Self {
        let table = match wave_form {
            WaveForm::Sine => Some(&SINE),
            _ => None,
        };

        Lfo {
            phase: 0.0,
            step: (init_freq * FULL_WAVE as f32) / SAMPLING_RATE as f32,
            freq: init_freq,
            min_freq,
            max_freq,
            wave_form,
            wave_table: table,
        }
    }

    pub fn inc_phase(&mut self) {
        self.phase += self.step;
        let table_len = FULL_WAVE as f32;
        if self.phase >= table_len {
            self.phase -= table_len;
        }
    }

    pub fn set_wave_form(&mut self, wave_form: WaveForm) {
        self.wave_form = wave_form;
    }

    pub fn accquire(&mut self) -> f32 {
        let wave_func: fn(usize) -> i16 = match self.wave_form {
            WaveForm::Sine => sin_i16,
            WaveForm::Square => square_i16,
            WaveForm::SawTooth => saw_tooth_i16,
            WaveForm::Triangle => triangle_i16,
        };
        let time = esp_hal::time::now().ticks();
        let phase = ((time as f32 / 1_000_000.0) * self.freq * TAU) as usize % FULL_WAVE;

        wave_func(phase) as f32 / i16::MAX as f32
    }
    pub fn gen_signal(&mut self, sample: &mut i16, samples: usize, write_over: bool) {
        let wave_func: fn(usize) -> i16 = match self.wave_form {
            WaveForm::Sine => sin_i16,
            WaveForm::Square => square_i16,
            WaveForm::SawTooth => saw_tooth_i16,
            WaveForm::Triangle => triangle_i16,
        };

        if write_over {
            *sample = wave_func(self.phase as usize);
            self.inc_phase();
        } else {
            *sample += wave_func(self.phase as usize);
            self.inc_phase();
        }
    }

    pub fn next_waveform(&mut self) {
        self.wave_form = match self.wave_form {
            WaveForm::Sine => WaveForm::Triangle,
            WaveForm::Triangle => WaveForm::Square,
            WaveForm::Square => WaveForm::SawTooth,
            WaveForm::SawTooth => WaveForm::Sine,
        };
    }

    pub fn set_frequency(&mut self, val: f32) -> bool {
        if val > self.max_freq || val < self.min_freq {
            return false;
        };

        self.freq = val;
        return true;
    }

    pub fn get_frequency(&self) -> f32 {
        self.freq
    }
}
