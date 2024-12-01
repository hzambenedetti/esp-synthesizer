pub struct Envelope {
    value: f32,
    state: EnvelopeState,
    step: f32,
    params: EnvelopeParameters,
}

struct EnvelopeParameters {
    attack_rate: f32,
    decay_rate: f32,
    sustain_value: f32,
    release_rate: f32,
}

impl Envelope {
    pub fn new(
        attack_rate: f32,
        decay_rate: f32,
        sustain_value: f32,
        release_rate: f32,
    ) -> Result<Envelope, u8> {
        if attack_rate <= 0.0 {
            return Err(0);
        }
        if decay_rate >= 0.0 {
            return Err(1);
        }
        if !(0.0..1.0).contains(&sustain_value) {
            return Err(2);
        }
        if release_rate >= 0.0 {
            return Err(3);
        }
        Ok(Envelope {
            value: 0.0,
            state: EnvelopeState::Off,
            step: 0.0,
            params: EnvelopeParameters {
                attack_rate: attack_rate / 44100.0,
                decay_rate: decay_rate / 44100.0,
                sustain_value,
                release_rate: release_rate / 44100.0,
            },
        })
    }

    pub fn trigger(&mut self) {
        self.state = EnvelopeState::Attack;
        self.step = self.params.attack_rate;
    }

    pub fn detrigger(&mut self) {
        self.state = EnvelopeState::Release;
        self.step = self.params.release_rate;
    }

    pub fn gen_signal(&mut self, buffer: &mut [f32], samples: usize) {
        for i in 0..samples {
            buffer[i] = self.value;
            self.value += self.step;
            match self.state {
                EnvelopeState::Off => (),
                EnvelopeState::Attack => {
                    if self.value > 1.0 {
                        self.value = 1.0;
                        self.state = EnvelopeState::Decay;
                        self.step = self.params.decay_rate;
                    }
                }
                EnvelopeState::Decay => {
                    if self.value < self.params.sustain_value {
                        self.value = self.params.sustain_value;
                        self.state = EnvelopeState::Sustain;
                        self.step = 0.0;
                    }
                }
                EnvelopeState::Sustain => (),
                EnvelopeState::Release => {
                    if self.value < 0.0 {
                        self.value = 0.0;
                        self.state = EnvelopeState::Off;
                        self.step = 0.0;
                    }
                }
            }
        }
    }
}

enum EnvelopeState {
    Off,
    Attack,
    Decay,
    Sustain,
    Release,
}
