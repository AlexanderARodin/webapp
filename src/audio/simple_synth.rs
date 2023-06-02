use crate::raadbg::log;

use crate::audio::proxy_render::*;
use crate::audio::midi_rx_tx::*;

const PI2: f32 = 2. * std::f32::consts::PI;

pub struct SimpleSynth{
    sample_rate: f32,
    counter: f32,
    frequency: f32,
    amplitude: f32,
}
impl Default for SimpleSynth {
    fn default() -> Self {
        Self::new( 44100 )
    }
}
impl Drop for SimpleSynth {
    fn drop(&mut self) {
        self.reset();
        log::drop("SimpleSynth");
    }
}
impl SimpleSynth {
    pub fn new( sample_rate: usize ) -> Self {
        log::create("SimpleSynth");
        Self{
            sample_rate: sample_rate as f32,
            counter: 0_f32,
            frequency: 440.,
            amplitude: 1.
        }
    }
}

//
//
impl RenderWrapper for SimpleSynth {
    fn render(&mut self, data: &mut [f32]) {
        //log::tick();
        let mult = self.frequency * PI2 / self.sample_rate;
        for samples in data.chunks_mut(2) {
            let ampl = self.amplitude*(self.counter * mult ).sin();
            for sample in samples {
                *sample = ampl;
            }
            self.counter += 1.;
        }
    }
}


//
//
impl MidiReceiver for SimpleSynth {
    fn reset(&mut self) {
        log::info("SimpleSynth", "reset");
    }
    fn process_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32) {
        log::info("SimpleSynth", "invoke_midi_command");
    }
}

//
//
impl SimpleSynth {
}
//
//
//
//
#[cfg(test)]
mod test {
}

