use std::sync::{Arc,Mutex};
use crate::raadbg::log;

//use crate::audio::midi_rx_tx::*;
//  //  //  //  //  //  //  //  //


pub struct ProxyRender {
    pub(crate) render_wrapper: Option< Arc<Mutex<dyn super::SoundRender>> >,
}
impl Drop for ProxyRender{
    fn drop(&mut self) {
        log::drop("ProxyRender");
    }
}
//impl Default for ProxyRender {
//    fn default() -> Self {
//        Self::new()
//    }
//}
impl ProxyRender {
    fn new() -> Arc<Mutex<Self>> {
        log::create("ProxyRender");
        Arc::new(Mutex::new(Self{ 
            render_wrapper: None
        }))
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

