use crate::{
    wave::constants::{FULL_WAVE, HALF_WAVE, SINE},
    SAMPLING_RATE,
};

const MAX_VALUE: usize = 32_767;
const SAW_TOOTH_CONST: f32 = MAX_VALUE as f32 / FULL_WAVE as f32;
const TRIANGLE_CONST: f32 = 2.0 * SAW_TOOTH_CONST;

pub struct Oscilator {
    phase: f32,
    step: f32,
    freq: f32,
    min_freq: f32,
    max_freq: f32,
    pub wave_form: WaveForm,
    wave_table: Option<&'static [i16; 1024]>,
}

#[derive(Copy, Clone)]
pub enum WaveForm {
    Sine,
    Square,
    SawTooth,
    Triangle,
}

#[allow(dead_code)]
impl Oscilator {
    pub fn new(init_freq: f32, min_freq: f32, max_freq: f32, wave_form: WaveForm) -> Self {
        let table = match wave_form {
            WaveForm::Sine => Some(&SINE),
            _ => None,
        };

        Oscilator {
            phase: 0.0,
            step: (init_freq * FULL_WAVE as f32) / SAMPLING_RATE as f32,
            freq: init_freq,
            min_freq,
            max_freq,
            wave_form,
            wave_table: table,
        }
    }

    pub fn set_wave_form(&mut self, wave_form: WaveForm) {
        self.wave_form = wave_form;
    }
    pub fn next_waveform(&mut self) {
        self.wave_form = match self.wave_form {
            WaveForm::Sine => WaveForm::Triangle,
            WaveForm::Triangle => WaveForm::Square,
            WaveForm::Square => WaveForm::SawTooth,
            WaveForm::SawTooth => WaveForm::Sine,
        };
    }

    pub fn inc_phase(&mut self) {
        self.phase += self.step;
        let table_len = FULL_WAVE as f32;
        if self.phase >= table_len {
            self.phase -= table_len;
        }
    }

    pub fn gen_signal(&mut self, buffer: &mut [i16], samples: usize, write_over: bool) {
        let wave_func: fn(usize) -> i16 = match self.wave_form {
            WaveForm::Sine => sin_i16,
            WaveForm::Square => square_i16,
            WaveForm::SawTooth => saw_tooth_i16,
            WaveForm::Triangle => triangle_i16,
        };

        if write_over {
            for i in (0..samples).step_by(2) {
                buffer[i] = wave_func(self.phase as usize);
                buffer[i + 1] = buffer[i];
                self.inc_phase();
            }
        } else {
            for i in (0..samples).step_by(2) {
                buffer[i] += wave_func(self.phase as usize);
                buffer[i + 1] = buffer[i];
                self.inc_phase();
            }
        }
    }

    pub fn set_frequency(&mut self, val: f32) -> bool {
        if val > self.max_freq || val < self.min_freq {
            return false;
        };

        self.freq = val;
        self.step = (val * FULL_WAVE as f32) / SAMPLING_RATE as f32;
        return true;
    }

    pub fn get_frequency(&self) -> f32 {
        self.freq
    }
}

pub fn sin_i16(x: usize) -> i16 {
    SINE[x]
}

pub fn square_i16(x: usize) -> i16 {
    if x < HALF_WAVE {
        return 32_767;
    }
    return 0;
}

pub fn saw_tooth_i16(x: usize) -> i16 {
    (SAW_TOOTH_CONST * (x as f32)) as i16
}

pub fn triangle_i16(x: usize) -> i16 {
    if x < HALF_WAVE {
        return (TRIANGLE_CONST * x as f32) as i16;
    }

    return (MAX_VALUE as f32 - (x - HALF_WAVE) as f32 * TRIANGLE_CONST) as i16;
}
