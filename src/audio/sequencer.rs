use std::sync::{Arc,Mutex};
use crate::raadbg::log;

use tinyaudio::prelude::*;
use super::audio_device::SoundRender;
use super::midi_rx_tx::MidiReceiver;
//  //  //  //  //  //  //  //  //


pub struct Sequencer {
    left_buf:  Vec<f32>,
    right_buf: Vec<f32>,
    sound_render: Option< Arc<Mutex<dyn SoundRender>> >,
}
impl Drop for Sequencer{
    fn drop(&mut self) {
//        log::drop("ProxyRender");
    }
}
impl Sequencer {
    pub fn new_arc_mutex(device_parameters: &OutputDeviceParameters) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new( Self::new(device_parameters) ))
    }
    pub fn new(device_parameters: &OutputDeviceParameters) -> Self {
//        log::create("ProxyRender");
        Self{ 
            left_buf:  vec![ 0_f32; device_parameters.channel_sample_count],
            right_buf: vec![ 0_f32; device_parameters.channel_sample_count],
            sound_render: None
        }
    }
    pub(crate) fn set_soundrender(&mut self, 
                                  new_soundrender: Option<Arc<Mutex<dyn SoundRender>>>) {
        self.sound_render = new_soundrender;
    }
    
    pub fn render_all(&mut self, data: &mut [f32]) {
        match &self.sound_render {
            None => {
                for sample in data {
                    *sample = 0_f32;
                }
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render
                    .lock().expect("FATAL: can't lock SoundRender.render!");
                sound_render_lock.render(&mut self.left_buf,&mut self.right_buf);
                for (i, samples) in data.chunks_mut(2).enumerate() {
                    samples[0] = self.left_buf[i];
                    samples[1] = self.right_buf[i];
                }
            }
        }
    }
}


//
impl MidiReceiver for Sequencer {
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
