use std::error::Error;
use std::fs::*;
use std::sync::{Arc,Mutex};

use crate::raadbg::log;

use tinyaudio::prelude::*;
use rustysynth::*;


// tinyaudio wrapper
pub struct AudioDevice{
    parameters: OutputDeviceParameters,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
    pub render: Arc<Mutex<dyn AudioRender>>,
}

pub trait AudioRender : Send {
    fn render(&mut self, data: &mut [f32]);
}

//
impl Default for AudioDevice {
    fn default() -> Self {
        Self::new( 2, 44100, 4410 )
    }
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        self.stop();
        log::drop("AudioDevice");
    }
}

//
impl AudioDevice{

    pub fn new( channels_count: usize, sample_rate: usize, 
            channel_sample_count: usize ) -> Self{
        let init_params = OutputDeviceParameters {
            channels_count: channels_count,
            sample_rate: sample_rate,
            channel_sample_count: channel_sample_count
        };
        log::create("AudioDevice");
        AudioDevice{ 
            parameters: init_params,
            device: None,
            render: Arc::new(Mutex::new( DefaultRender::new(440.) ))
        }
    }

    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_started() { log::error("AudioDevice", "Device is still active!");
            Err("[ AudioDevice] E: device still active!".to_string().into() )
        }else{
            log::info("AudioDevice", "start ");
            let params = self.parameters.clone();
            let render_clone = self.render.clone();
            let dev = run_output_device( params, {
                let render = render_clone;
                move |data: &mut [f32]| {
                    let mut render_lock = render.lock().expect("panic on locking audio_render");
                    render_lock.render(data);
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
        let mut midi = crate::midi_sequencer::MIDISequencer::default();
        let mut file = File::open("Horn.SF2").unwrap();
        let sf = Arc::new( SoundFont::new(&mut file).unwrap() );
        let _res = midi.load( &sf ).unwrap();
            midi.tst();
        self.render = Arc::new(Mutex::new(midi));
    }

    pub fn is_started(&self) -> bool {
        match self.device {
            None => false,
            _ => true
        }
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
    fn render(&mut self, data: &mut [f32]) {

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

