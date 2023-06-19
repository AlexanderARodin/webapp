use std::sync::{Arc,Mutex};
use crate::raadbg::log;
use super::super::midi_rx_tx::MidiReceiver;


pub trait SoundRender: Sync + Send + MidiReceiver {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]);
}

pub struct ProxyRender {
    pub(crate) sound_render: Option< Arc<Mutex<dyn SoundRender>> >,
}
impl ProxyRender {
    pub fn new_arc_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new( Self::new() ))
    }
    pub fn new() -> Self {
        log::create("ProxyRender");
        Self{ 
            sound_render: None
        }
    }
    
    pub fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
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
                sound_render_lock.render(left, right);
            }
        }
    }
}

impl Drop for ProxyRender{
    fn drop(&mut self) {
        log::drop("ProxyRender");
    }
}

