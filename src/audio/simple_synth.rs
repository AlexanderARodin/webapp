use crate::raadbg::log;

use crate::audio::proxy_render::*;
use crate::audio::midi_rx_tx::*;

const PI2: f32 = 2. * std::f32::consts::PI;
const VELO_PAR: f32 = 1.;
//  //  //  //  //  //  //

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
            frequency: 1_f32,
            amplitude: 0_f32
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
        match command {
            0x80 => self.note_off(channel, data1),       // Note Off
            0x90 => self.note_on(channel, data1, data2), // Note On
            _ => log::info("SimpleSynth", "W: unknown midi command")
        }
    }
}

//
//
impl SimpleSynth {
    pub fn note_on(&mut self, channel: i32, key: i32, velocity: i32) {
        log::info("SimpleSynth", "note ON");
        self.amplitude = SimpleSynth::amplitudeFrom( velocity );
        self.frequency = SimpleSynth::frequencyFrom( key );
    }
    pub fn note_off(&mut self, channel: i32, key: i32) {
        log::info("SimpleSynth", "note OFF");
        self.amplitude = 0_f32;
        self.counter = 0_f32;
    }
    
    fn frequencyFrom( key: i32 ) -> f32 {
        440. * 2_f32.powf( ((key as f32) - 69.)/12. )
    }
    fn amplitudeFrom( velocity: i32 ) -> f32 {
        let norm = (velocity as f32) / 127_f32;
        norm / 2_f32.powf(norm * VELO_PAR)
    }
}
//
//
//
//
#[cfg(test)]
mod test {
}

