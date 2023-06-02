use std::error::Error;
use std::sync::{Arc,Mutex};

use crate::raadbg::log;

use tinyaudio::prelude::*;
use rustysynth::SoundFont;

use crate::audio::proxy_render::*;
use crate::audio::midi_controller::*;
use crate::audio::simple_synth::*;


// TinyAudio wrapper
pub struct AudioDevice{
    sample_rate: usize,
    channel_sample_count: usize,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
    pub proxy_render: Arc<Mutex<ProxyRender>>,
}

impl Default for AudioDevice {
    fn default() -> Self {
        Self::new( 44100, 441*2 )
    }
}
impl Drop for AudioDevice {
    fn drop(&mut self) {
        self.reset();
        self.stop();
        log::drop("AudioDevice");
    }
}
impl AudioDevice {
    pub fn new( sample_rate: usize, channel_sample_count: usize ) -> Self {
        log::create("AudioDevice");
        Self{ 
            sample_rate: sample_rate,
            channel_sample_count: channel_sample_count,
            device: None,
            proxy_render: Arc::new(Mutex::new( ProxyRender::default() )),
            //render: Arc::new(Mutex::new( DefaultRender::new(440.) ))
        }
    }
}

//

impl MidiController for AudioDevice {
    fn reset(&mut self) {
        log::info("AudioDevice", "midi.RESET");
        let mut proxy_lock = self.proxy_render.lock().expect("can't lock proxy_render");
        match &proxy_lock.render {
            SynthRender::NoRender => {
                let simsyn = SimpleSynth::new( self.sample_rate );
                proxy_lock.render = SynthRender::CustomSynth(Arc::new(Mutex::new( simsyn )));
            },
            _ => {
            }
        }
    }
    fn note_on(&mut self, channel: i32, key: i32, velocity: i32) {
        log::info("AudioDevice", "midi.NoteOn");
    }
    fn note_off(&mut self, channel: i32, key: i32) {
        log::info("AudioDevice", "midi.NoteOff");
    }
}


//
impl AudioDevice{
    pub fn get_params(&self) -> OutputDeviceParameters  {
        OutputDeviceParameters {
            sample_rate: self.sample_rate, 
            channels_count: 2,
            channel_sample_count: self.channel_sample_count
        }
    }

    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_started() {
            log::error("AudioDevice", "Device is still active!");
            Err("[ AudioDevice] E: device still active!".to_string().into() )
        }else{
            log::info("AudioDevice", "start ");
            let proxy_render_clone = self.proxy_render.clone();
            let params = OutputDeviceParameters{ 
                    channels_count: 2,
                    sample_rate: self.sample_rate,
                    channel_sample_count: self.channel_sample_count
                };
            let dev = run_output_device( params, {
                let proxy_render = proxy_render_clone;
                //let mut left_buf  = vec![ 0_f32; self.block_size];
                //let mut right_buf = vec![ 0_f32; self.block_size];
                move |data: &mut [f32]| {
                    let mut proxy_render_lock = proxy_render.lock().expect("panic on locking PROXY_audio_render");
                    proxy_render_lock.render( data );
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

//    pub fn tst_AB(&mut self) {
//        let mut midi = crate::audio::midi_sequencer::MIDISequencer::default();
//        let mut fl = super::SF_PIANO.clone();
//        let sf = Arc::new( SoundFont::new(&mut fl).unwrap() );
//        let _res = midi.load( &sf ).unwrap();
//            midi.tst();
//        //self.render = Arc::new(Mutex::new(midi));
//    }
}


//

#[cfg(test)]
mod test {
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



