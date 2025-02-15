use heapless::Vec;

const STATUS_NOTE_ON: u8 = 0x90;
const STATUS_NOTE_OFF: u8 = 0x80;

#[derive(Debug)]
pub enum MidiEvent {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
}

pub struct MidiParser {
    buffer: Vec<u8, 3>,
    current_status: Option<u8>,
}

impl MidiParser {
    pub fn new() -> Self {
        MidiParser {
            buffer: Vec::new(),
            current_status: None,
        }
    }

    pub fn parse_byte(&mut self, byte: u8) -> Option<MidiEvent> {
        if byte & 0x80 != 0 {
            // Status byte
            self.current_status = Some(byte);
            self.buffer.clear();
            None
        } else {
            // Data byte
            if self.buffer.push(byte).is_err() {
                self.buffer.clear();
            }

            if let Some(status) = self.current_status {
                let msg_type = status & 0xF0;
                // esp_println::println!("message: {msg_type}, note: {}", self.buffer[0]);
                match (msg_type, self.buffer.len()) {
                    (STATUS_NOTE_ON, 2) => Some(MidiEvent::NoteOn {
                        note: self.buffer[0],
                        velocity: self.buffer[1],
                    }),
                    (STATUS_NOTE_OFF, 2) => Some(MidiEvent::NoteOff {
                        note: self.buffer[0],
                    }),
                    _ => None,
                }
            } else {
                None
            }
        }
    }
}

pub fn midi_note_to_freq(note: u8) -> f32 {
    const A4_FREQ: f32 = 440.0;
    const A4_NOTE: f32 = 69.0;

    // Use libm's powf implementation
    let exponent = (note as f32 - A4_NOTE) / 12.0;
    A4_FREQ * libm::powf(2.0, exponent)
}
