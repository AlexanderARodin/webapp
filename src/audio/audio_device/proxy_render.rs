use std::sync::{Arc,Mutex};
//use crate::raadbg::log;
//  //  //  //  //  //  //  //  //


pub struct ProxyRender {
    pub(crate) render_wrapper: Option< Arc<Mutex<dyn super::SoundRender>> >,
}
//impl Drop for ProxyRender{
//    fn drop(&mut self) {
//        log::drop("ProxyRender");
//    }
//}
impl ProxyRender {
    pub fn new() -> Arc<Mutex<Self>> {
//        log::create("ProxyRender");
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
            Some(sound_render) => {
                let mut sound_render_lock = render_wrapper.lock().expect("FATAL: can't lock SoundRender!");
                sound_render_lock.render(data);
            }
        }
    }
}

