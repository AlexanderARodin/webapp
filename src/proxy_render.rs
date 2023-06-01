use std::sync::{Arc,Mutex};

use crate::raadbg::log;


pub struct ProxyRender {
    clck: f32,
    render: SynthRender,
}
impl Drop for ProxyRender{
    fn drop(&mut self) {
        log::drop("ProxyRender");
    }
}
impl Default for ProxyRender {
    fn default() -> Self {
        Self::new(SynthRender::NoRender)
    }
}
impl ProxyRender {
    fn new( a_render: SynthRender ) -> Self {
        log::create("ProxyRender");
        Self{ 
            clck: 0_f32,
            render: a_render
        }
    }

    fn render(&mut self, data: &mut [f32]) {
        match &self.render {
            SynthRender::NoRender => {
                    log::tick();
                for sample in data {
                    *sample = 0_f32;
                }
                //log::tick();
               // for samples in data.chunks_mut(2) {
               //     self.clck += 1.;
               //     let ampl = (self.clck * 440. * 2. * std::f32::consts::PI / 44100. ).sin();
                //    for sample in samples {
                //        *sample = ampl;
                 //   }
                //}
            },
            SynthRender::CustomSynth(cust_render) => {
                let mut cust_render_lock = cust_render.lock().expect("can't lock CustomSynth");
                cust_render_lock.render(data);
            },
            _ => {
                panic!("in progress");
            }
        }
    }
}



enum SynthRender {
    NoRender,
    RustySynth( Arc< rustysynth::Synthesizer > ),
    CustomSynth( Arc<Mutex<dyn CustSynthRender>> ),
}


pub trait CustSynthRender: Sync + Send {
    fn render(&mut self, data: &mut [f32]);
}

