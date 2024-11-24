
pub struct Oscilator{
    phase: f32,
    step: f32,
    min_freq: f32,
    max_freq: f32,
    wave_form: WaveForm,
    wave_table: &'static [i16],
}

pub enum WaveForm{
    Sine,
    Square,
    SawTooth,
    Triangle,
}

impl Oscilator{
    pub fn new() -> Self{
        todo!();
    }

    pub fn set_wave_form(&mut self, wave_form: WaveForm){
        self.wave_form = wave_form;
    }

    pub fn inc_phase(&mut self){
        self.phase += self.step;
        let table_len = self.wave_table.len() as f32;
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

}

fn sin_i16(x: usize) -> i16{
    todo!();
}

fn square_i16(x: usize) -> i16{
    todo!();
}

fn saw_tooth_i16(x: usize) -> i16{
    todo!();
}

fn triangle_i16(x: usize) -> i16{
    todo!();
}



