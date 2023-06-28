use std::sync::{Arc,Mutex};
use crate::raadbg::log;
use super::super::super::midi_lib::{MidiMessage,MidiReceiver,MidiSequence};


pub trait SoundRender: MidiReceiver + Sync + Send {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]);
    fn get_as_midi_receiver(&mut self) -> &mut dyn MidiReceiver;
}

pub struct ProxyRender {
    test_seq: MidiSequence,
    pub(crate) tick_time: f32,
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
        seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
        seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
        seq.push( 0., &MidiMessage::NoteOn( 1,92,80) );
        seq.push( 1., &MidiMessage::NoteOff(1,92,80) );
        seq.push( 1., &MidiMessage::NoteOff(1,92,80) );
        Self{ 
            test_seq: seq,
            tick_time: 0.,
            sound_render: None
        }
    }
    
    pub fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        match &self.sound_render {
            None => {
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
                let midi_recevier: &mut dyn MidiReceiver = sound_render_lock.get_as_midi_receiver();
                self.test_seq.send_next_sequence( &self.tick_time, midi_recevier );
                sound_render_lock.render(left, right);
                if self.test_seq.is_finished() {
                    self.test_seq.restart();
                }
            }
        }
    }
}

impl Drop for ProxyRender {
    fn drop(&mut self) {
        log::drop("ProxyRender");
    }
}

