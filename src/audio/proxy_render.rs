use std::sync::{Arc,Mutex};
use crate::raadbg::log;

use crate::audio::simple_synth::*;

pub struct ProxyRender {
    pub rrender: SynthRender,
    pub rrrender: Option< Arc<Mutex<dyn CustSynthRender>> >
}
impl Drop for ProxyRender{
    fn drop(&mut self) {
        log::drop("ProxyRender");
    }
}
impl Default for ProxyRender {
    fn default() -> Self {
        Self::new(SynthRender::NoRender)
        //Self::new( SynthRender::CustomSynth(Arc::new(Mutex::new( SimpleSynth::default() ))) )
    }
}
impl ProxyRender {
    fn new( a_render: SynthRender ) -> Self {
        log::create("ProxyRender");
        Self{ 
            render: a_render,
            rrrender: None
        }
    }

    pub fn render(&mut self, data: &mut [f32]) {
        match &self.rrrender {
            None => {
                for sample in data {
                    *sample = 0_f32;
                }
            },
            Some(cust_render) => {
                let mut cust_render_lock = cust_render.lock().expect("can't lock CustomSynth");
                cust_render_lock.render(data);
            }
        }
    }
}


//

#[allow(dead_code)]
pub enum SynthRender {
    NoRender,
    CustomSynth( Arc<Mutex<dyn CustSynthRender>> ),
}

pub trait CustSynthRender: Sync + Send {
    fn render(&mut self, data: &mut [f32]);
}


