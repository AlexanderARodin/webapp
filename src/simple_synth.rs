
use crate::raadbg::log;
use crate::audio_device::CustSynthRender;


pub struct SimpleSynth{
    clck: f32,
}
impl Default for SimpleSynth {
    fn default() -> Self {
        Self::new(  )
    }
}
impl Drop for SimpleSynth {
    fn drop(&mut self) {
        //self.stop();
        log::drop("SimpleSynth");
    }
}
impl SimpleSynth {
    pub fn new(  ) -> Self {
        log::create("SimpleSynth");
        Self{ 
            clck: 0_f32
        }
    }
}

//
//
impl CustSynthRender for SimpleSynth {
    fn render(&mut self, data: &mut [f32]) {
        
        log::tick();
        
        for samples in data.chunks_mut(2) {
            self.clck += 1.;
            let ampl = (self.clck * 880. * 2. * std::f32::consts::PI / 44100. ).sin();
            for sample in samples {
                *sample = ampl;
            }
        }
    }
}


//
//
#[cfg(test)]
mod test {
}

