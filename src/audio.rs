use std::error::Error;
use std::sync::{Arc,Mutex};

use crate::raadbg::log;

use tinyaudio::prelude::*;
use rustysynth::*;

//mod proxy_render;
//mod simple_synth;
//mod midi_sequencer;
//use crate::proxy_render::*;

static SF_PIANO:   &'static [u8] = include_bytes!("../SoundFonts/Piano Grand.SF2");
//static SF_STRINGS: &'static [u8] = include_bytes!("../SoundFonts/String Marcato.SF2");
//static SF_ORGAN:   &'static [u8] = include_bytes!("../SoundFonts/Organ Chorus.SF2");


// tinyaudio wrapper
pub struct AudioDevice{
    sample_rate: usize,
    block_size: usize,
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
        self.stop();
        log::drop("AudioDevice");
    }
}
impl AudioDevice {
    pub fn new( sample_rate: usize, block_size: usize ) -> Self {
        log::create("AudioDevice");
        Self{ 
            sample_rate: sample_rate,
            block_size: block_size,
            device: None,
            proxy_render: Arc::new(Mutex::new( ProxyRender::default() )),
            //render: Arc::new(Mutex::new( DefaultRender::new(440.) ))
        }
    }
}



/////



pub trait AudioRender : Send {
    fn render(&mut self, data: &mut [f32], 
              left_buf: &mut [f32], right_buf: &mut [f32] );
}

//

//
impl AudioDevice{

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
                    channel_sample_count: self.block_size
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

    pub fn tst_A(&mut self) {
        let mut midi = crate::midi_sequencer::MIDISequencer::default();
        let mut fl = SF_PIANO.clone();
        let sf = Arc::new( SoundFont::new(&mut fl).unwrap() );
        let _res = midi.load( &sf ).unwrap();
            midi.tst();
        //self.render = Arc::new(Mutex::new(midi));
    }
    pub fn tst_B(&mut self) {
        //self.render = Arc::new(Mutex::new( DefaultRender::new(880.) ));
    }
}


//

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



