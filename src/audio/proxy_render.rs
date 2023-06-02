use std::sync::{Arc,Mutex};
use crate::raadbg::log;

use crate::audio::simple_synth::*;
//  //  //  //  //  //  //  //  //



pub struct ProxyRender {
    pub render: Option< Arc<Mutex<dyn RenderWrapper>> >
    
    pub trait RenderWrapper: Sync + Send {
        fn render(&mut self, data: &mut [f32]);
    }
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
    fn new( render: Option< Arc<Mutex<dyn RenderWrapper>> > ) -> Self {
        log::create("ProxyRender");
        Self{ 
            render: render
        }
    }

    pub fn render(&mut self, data: &mut [f32]) {
        match &self.render {
            None => {
                for sample in data {
                    *sample = 0_f32;
                }
            },
            Some(cust_render) => {
                let mut cust_render_lock = cust_render.lock().expect("can't lock RenderWrapper");
                cust_render_lock.render(data);
            }
        }
    }
}



