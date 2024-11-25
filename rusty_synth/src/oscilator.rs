
use crate::wave::constants::{
    SINE,
    HALF_WAVE,
    FULL_WAVE,
};


const MAX_VALUE: usize = 32_767;
const SAW_TOOTH_CONST: f32 = MAX_VALUE as f32/FULL_WAVE as f32;
const TRIANGLE_CONST: f32 = 2.0 * SAW_TOOTH_CONST;

pub struct Oscilator{
    phase: f32,
    step: f32,
    freq: f32,
    min_freq: f32,
    max_freq: f32,
    wave_form: WaveForm,
    wave_table: Option<&'static [i16; 1024]>,
}


pub enum WaveForm{
    Sine,
    Square,
    SawTooth,
    Triangle,
}

#[allow(dead_code)]
impl Oscilator{
    pub fn new(
        init_freq: f32,
        min_freq: f32, 
        max_freq: f32, 
        wave_form: WaveForm
    ) -> Self{
        let table = match wave_form{
            WaveForm::Sine => Some(&SINE),
            _ => None,
        };

        Oscilator{
            phase: 0.0,
            step: (init_freq * FULL_WAVE as f32)/44100.0,
            freq: init_freq,
            min_freq,
            max_freq,
            wave_form,
            wave_table: table

        }        
    }

    pub fn set_wave_form(&mut self, wave_form: WaveForm){
        self.wave_form = wave_form;
    }

    pub fn inc_phase(&mut self){
        self.phase += self.step;
        let table_len = FULL_WAVE as f32;
        if self.phase >= table_len{
            self.phase = self.phase - table_len;
        } 
    }

    pub fn gen_signal(
        &mut self, 
        buffer: &mut [i16], 
        samples: usize, 
        write_over: bool
    ){
        let wave_func: fn(usize) -> i16 = match self.wave_form{
            WaveForm::Sine => sin_i16,
            WaveForm::Square => square_i16,
            WaveForm::SawTooth => saw_tooth_i16,
            WaveForm::Triangle => triangle_i16,
        };
        
        if write_over == true{
            for i in 0..samples{
                buffer[i] = wave_func(self.phase as usize);
                self.inc_phase();
            }
        }else{
            for i in 0..samples{
                buffer[i] += wave_func(self.phase as usize);
                self.inc_phase();
            }
        }
    }

    pub fn set_frequency(&mut self, val: f32) -> bool{
        if val > self.max_freq || val < self.min_freq {return false};
        
        self.freq = val;
        self.step = (val * FULL_WAVE as f32)/44100.0; 
        return true
    }

    pub fn get_frequency(&self) -> f32{
        self.freq
    }

}

fn sin_i16(x: usize) -> i16{
    SINE[x]
}

fn square_i16(x: usize) -> i16{
    if x < HALF_WAVE {return 32_767;}
    return 0;
}

fn saw_tooth_i16(x: usize) -> i16{
    (SAW_TOOTH_CONST * (x as f32)) as i16
}

fn triangle_i16(x: usize) -> i16{
    if x < HALF_WAVE{
        return (TRIANGLE_CONST * x as f32) as i16; 
    }

    return (MAX_VALUE as f32 - (x - HALF_WAVE) as f32 * TRIANGLE_CONST) as i16;
}



