use std::sync::{Arc,Mutex};
//use crate::raadbg::log;
//  //  //  //  //  //  //  //  //


pub struct ProxyRender {
    pub(crate) sound_render: Option< Arc<Mutex<dyn super::SoundRender>> >,
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
    
    pub fn render(&mut self, data: &mut [f32]) {
        match &self.sound_render {
            None => {
                for sample in data {
                    *sample = 0_f32;
                }
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render.lock().expect("FATAL: can't lock SoundRender!");
                sound_render_lock.render(data);
            }
        }
    }
}

