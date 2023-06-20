use std::sync::{Arc,Mutex};
use crate::raadbg::log;
use super::super::midi_rx_tx::MidiReceiver;


pub trait SoundRender: Sync + Send + MidiReceiver {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]);
}

pub struct ProxyRender {
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
        Self{ 
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



