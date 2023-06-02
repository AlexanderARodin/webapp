use std::sync::{Arc,Mutex};
use crate::raadbg::log;

use crate::audio::simple_synth::*;
use crate::audio::midi_rx_tx::*;
//  //  //  //  //  //  //  //  //


pub struct ProxyRender {
    pub render_wrapper: Option< Arc<Mutex<dyn SoundRender>> >,
}
impl Drop for ProxyRender{
    fn drop(&mut self) {
        log::drop("ProxyRender");
    }
}
impl Default for ProxyRender {
    fn default() -> Self {
        Self::new(None)
    }
}
impl ProxyRender {
    fn new( render_wrapper: Option< Arc<Mutex<dyn SoundRender>> > ) -> Self {
        log::create("ProxyRender");
        Self{ 
            render_wrapper: render_wrapper
        }
    }

    pub fn render(&mut self, data: &mut [f32]) {
        match &self.render_wrapper {
            None => {
                for sample in data {
                    *sample = 0_f32;
                }
            },
            Some(render_wrapper) => {
                let mut render_wrapper_lock = render_wrapper.lock().expect("can't lock RenderWrapper");
                render_wrapper_lock.render(data);
            }
        }
    }
}

