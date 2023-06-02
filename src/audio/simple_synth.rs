use crate::raadbg::log;

use crate::audio::proxy_render::*;
use crate::audio::midi_rx_tx::*;

const PI2: f32 = 2. * std::f32::consts::PI;

pub struct SimpleSynth{
    sample_rate: f32,
    clck: f32,
    pub tone_hz: f32
}
impl Default for SimpleSynth {
    fn default() -> Self {
        Self::new( 44100 )
    }
}
impl Drop for SimpleSynth {
    fn drop(&mut self) {
        //self.stop();
        log::drop("SimpleSynth");
    }
}
impl SimpleSynth {
    pub fn new( sample_rate: usize ) -> Self {
        log::create("SimpleSynth");
        Self{
            sample_rate: sample_rate as f32,
            clck: 0_f32,
            tone_hz: 440.
        }
    }
}

//
//
impl RenderWrapper for SimpleSynth {
    fn render(&mut self, data: &mut [f32]) {
        
        log::tick();
        let mult = self.tone_hz * PI2 / self.sample_rate;
        for samples in data.chunks_mut(2) {
            let ampl = (self.clck * mult ).sin();
            for sample in samples {
                *sample = ampl;
            }
            self.clck += 1.;
        }
    }
}


//
//
//impl MidiReceiver for SimpleSynth

//
//
#[cfg(test)]
mod test {
}

