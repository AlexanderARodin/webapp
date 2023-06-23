use std::sync::{Arc,Mutex};
use crate::raadbg::log;
//use super::super::super::midi_lib::{MidiMessage,MidiReceiver,MidiSequence};
use super::super::super::midi_lib::{MidiMessage,MidiSequence};
use crate::midi_lib::MidiReceiver;


pub trait SoundRender: crate::midi_lib::MidiReceiver + Sync + Send {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]);
}

pub struct ProxyRender {
    test_seq: MidiSequence,
    pub(crate) tick_time: f32,
    elapsed_time: f32,
    pub(crate) sound_render: Option< Arc<Mutex<dyn SoundRender>> >,
}
impl ProxyRender {
    pub fn new_arc_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new( Self::new() ))
    }
    pub fn new() -> Self {
        log::create("ProxyRender");
        let mut seq = MidiSequence::new();
        seq.push( 0.0, &MidiMessage::NoteOn( 1,90,80) );
        seq.push( 0.5, &MidiMessage::NoteOff(1,90,80) );
        seq.push( 0.5, &MidiMessage::NoteOn( 1,91,80) );
        seq.push( 1.0, &MidiMessage::NoteOff(1,91,80) );
        seq.push( 1.0, &MidiMessage::NoteOn( 1,92,80) );
        seq.push( 1.5, &MidiMessage::NoteOff(1,92,80) );
        seq.push( 2.0, &MidiMessage::NoteOff(1,92,80) );
        Self{ 
            test_seq: seq,
            tick_time: 0.,
            elapsed_time: 0.,
            sound_render: None
        }
    }
    
    pub fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.elapsed_time += self.tick_time;
        match &self.sound_render {
            None => {
                //log::tick();
                for sample in left {
                    *sample = 0_f32;
                }
                for sample in right {
                    *sample = 0_f32;
                }
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render.lock()
                    .expect("FATAL: can't lock SoundRender!");
                let mut midi_rec: &mut dyn MidiReceiver = &sound_render_lock;
                self.test_seq.send_next_sequence( self.elapsed_time, midi_rec );
                return;
                match self.elapsed_time {
                    x if x < 1. => {
                        log::tick();
                        sound_render_lock.process_midi_command( 1, 0x80, 90, 80);
                        sound_render_lock.process_midi_command( 1, 0x80, 91, 80);
                        sound_render_lock.process_midi_command( 1, 0x80, 92, 80);
                        sound_render_lock.process_midi_command( 1, 0x90, 90, 80);
                    },
                    x if x < 2. => {
                        sound_render_lock.process_midi_command( 1, 0x80, 90, 80);
                        sound_render_lock.process_midi_command( 1, 0x80, 91, 80);
                        sound_render_lock.process_midi_command( 1, 0x80, 92, 80);
                        sound_render_lock.process_midi_command( 1, 0x90, 91, 80);
                    },
                    x if x < 3. => {
                        sound_render_lock.process_midi_command( 1, 0x80, 90, 80);
                        sound_render_lock.process_midi_command( 1, 0x80, 91, 80);
                        sound_render_lock.process_midi_command( 1, 0x80, 92, 80);
                        sound_render_lock.process_midi_command( 1, 0x90, 92, 80);
                    },
                    x if x < 4. => {
                        sound_render_lock.process_midi_command( 1, 0x80, 90, 80);
                        sound_render_lock.process_midi_command( 1, 0x80, 91, 80);
                        sound_render_lock.process_midi_command( 1, 0x80, 92, 80);
                    },
                    _ => {
                        self.elapsed_time = 0.;
                    }
                }
                sound_render_lock.render(left, right);
            }
        }
    }
}

impl Drop for ProxyRender {
    fn drop(&mut self) {
        log::drop("ProxyRender");
    }
}



