use std::sync::{Arc,Mutex};
use crate::raadbg::log;

use super::SoundRender;
use super::super::midi_rx_tx::MidiReceiver;
//  //  //  //  //  //  //  //  //


pub struct ProxyRender {
    sound_render: Option< Arc<Mutex<dyn SoundRender>> >,
}
impl Drop for ProxyRender{
    fn drop(&mut self) {
//        log::drop("ProxyRender");
    }
}
impl ProxyRender {
    pub fn new_arc_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new( Self::new() ))
    }
    pub fn new() -> Self {
//        log::create("ProxyRender");
        Self{ 
            sound_render: None
        }
    }
    pub(crate) fn set_soundrender(&mut self, 
                                  new_soundrender: Option<Arc<Mutex<dyn SoundRender>>>) {
        self.sound_render = new_soundrender;
    }
    
    pub fn render(&mut self, data: &mut [f32]) {
        match &self.sound_render {
            None => {
                for sample in data {
                    *sample = 0_f32;
                }
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render
                    .lock().expect("FATAL: can't lock SoundRender.render!");
                sound_render_lock.render(data);
            }
        }
    }
}


//
impl MidiReceiver for ProxyRender {
    fn reset(&mut self) {
        log::info("ProxyRender", "MIDI.reset");
        match &self.sound_render {
            None => {
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render.lock()
                    .expect("FATAL: can't lock SoundRender.reset)");
                sound_render_lock.reset();
            }
        }
    }
    fn process_midi_command(&mut self, 
                            channel: i32, command: i32, 
                            data1: i32, data2: i32) {
        log::info("ProxyRender", "MIDI.process_midi_command");
        match &self.sound_render {
            None => {
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render.lock()
                    .expect("FATAL: can't lock SoundRender.process_midi_command");
                sound_render_lock.process_midi_command( channel, command, data1, data2 );
            }
        }
    }
}
