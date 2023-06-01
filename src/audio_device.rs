use std::error::Error;
use std::sync::{Arc,Mutex};

use crate::raadbg::log;

use tinyaudio::prelude::*;
use rustysynth::*;

static SF_PIANO:   &'static [u8] = include_bytes!("../SoundFonts/Piano Grand.SF2");
static SF_STRINGS: &'static [u8] = include_bytes!("../SoundFonts/String Marcato.SF2");
static SF_ORGAN:   &'static [u8] = include_bytes!("../SoundFonts/Organ Chorus.SF2");

#[cfg(test)]
mod test {
//    use super::*;
//    use std::fs::*;
//    #[should_panic]

    #[test]
    fn basic() {
        let mut audio_2 = super::AudioDevice::new(100,4410);
        let mut audio = super::AudioDevice::new(44100,4410);
        audio.start();
        audio_2.start();
    }
    #[test]
    fn load_internal_sf2() {
        assert!(false);
    }
    #[test]
    #[should_panic]
    fn error_load_internal_sf2() {
    }

}




// tinyaudio wrapper
pub struct AudioDevice{
    sample_rate: usize,
    block_size: usize,

    device: Option< Box<dyn BaseAudioOutputDevice> >,

    pub proxy_render: Arc<Mutex<ProxyRender>>,
    pub render: Arc<Mutex<dyn AudioRender>>,
}

pub struct ProxyRender {
    render: SynthRender,
}
impl ProxyRender {
    fn new() -> Self {
        Self{ render: SynthRender::NoRender }
    }
}



enum SynthRender {
    NoRender,
    RustySynth( rustysynth::Synthesizer ),
    CustomSynth( Arc<dyn CustSynthRender> ),
}


pub trait CustSynthRender: Send {
    fn render(&mut self, data: &mut [f32], 
              left_buf: &mut [f32], right_buf: &mut [f32] );
}


pub trait AudioRender : Send {
    fn render(&mut self, data: &mut [f32], 
              left_buf: &mut [f32], right_buf: &mut [f32] );
}

//
impl Default for AudioDevice {
    fn default() -> Self {
        Self::new( 44100, 441*2 )
    }
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        self.stop();
        log::drop("MidiDevice");
    }
}

//
impl AudioDevice{

    pub fn new( sample_rate: usize, block_size: usize ) -> Self {
        log::create("MidiDevice");
        Self{ 
            sample_rate: sample_rate,
            block_size: block_size,
            device: None,
            proxy_render: Arc::new(Mutex::new( ProxyRender::new() )),
            render: Arc::new(Mutex::new( DefaultRender::new(440.) ))
        }
    }

    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_started() { log::error("AudioDevice", "Device is still active!");
            Err("[ AudioDevice] E: device still active!".to_string().into() )
        }else{
            log::info("MidiDevice", "start ");
            let render_clone = self.render.clone();
            let params = OutputDeviceParameters{ 
                    channels_count: 2,
                    sample_rate: self.sample_rate,
                    channel_sample_count: self.block_size
                };
            let dev = run_output_device( params, {
                let render = render_clone;
                let mut left_buf = vec![ 0f32; self.block_size];
                let mut right_buf = vec![ 0f32; self.block_size];
                move |data: &mut [f32]| {
                    let mut render_lock = render.lock().expect("panic on locking audio_render");
                    render_lock.render(data, &mut left_buf, &mut right_buf);
                }
            });
            match dev {
                Err(e) => {
                    let errmsg = format!("{:?}",e);
                    log::error("AudioDevice", &errmsg);
                    return Err(e)
                },
                Ok(running_dev) => self.device = Some(running_dev),
            }
            Ok(())
        }
    }

    pub fn stop(&mut self) {
        self.device = None;
        log::info("AudioDevice", "stop!");
    }

    pub fn is_started(&self) -> bool {
        match self.device {
            None => false,
            _ => true
        }
    }

    pub fn tst_A(&mut self) {
        let mut midi = crate::midi_sequencer::MIDISequencer::default();
        let mut fl = SF_PIANO.clone();
        let sf = Arc::new( SoundFont::new(&mut fl).unwrap() );
        let _res = midi.load( &sf ).unwrap();
            midi.tst();
        self.render = Arc::new(Mutex::new(midi));
    }
    pub fn tst_B(&mut self) {
        self.render = Arc::new(Mutex::new( DefaultRender::new(880.) ));
    }
}


//
struct DefaultRender {
    clock: f32,
    tone_hz: f32,
    channels_count: usize,
    sample_rate: usize,
}

impl DefaultRender {
    fn new(tone_hz: f32) -> Self {
        DefaultRender{
            clock: 0.,
            tone_hz: tone_hz,
            channels_count: 2,
            sample_rate: 44100
        }
    }
}
impl AudioRender for DefaultRender {
    fn render(&mut self, data: &mut [f32], 
              _left_buf: &mut [f32], _right_buf: &mut [f32] ) {

        log::tick();

        for samples in data.chunks_mut(self.channels_count) {
            self.clock = (self.clock + 1.0) % self.sample_rate as f32;
            let value = ( 
                self.clock * self.tone_hz * 2.0 * std::f32::consts::PI 
                / self.sample_rate as f32
                )
                .sin() * 0.2;
            for sample in samples {
                *sample = value;
            }
        }
    }
}

