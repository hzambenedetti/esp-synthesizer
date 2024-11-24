
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
        todo!();
    }


}
