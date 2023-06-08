use std::error::Error;
use std::sync::{Arc,Mutex};

use crate::raadbg::log;

use tinyaudio::prelude::*;
//  //  //  //  //  //  //
mod proxy_render;
use proxy_render::*;

use super::midi_rx_tx::*;
//  //  //  //  //  //  //


// TinyAudio wrapper
pub struct AudioDevice{
    device: Option< Box<dyn BaseAudioOutputDevice> >,
}


impl Drop for AudioDevice {
    fn drop(&mut self) {
        self.stop();
        log::drop("AudioDevice");
    }
}
impl AudioDevice {
    pub fn new( ) -> Self {
        log::create("AudioDevice");
        Self{ 
            device: None,
        }
    }
    pub fn is_started(&self) -> bool {
        match self.device {
            None => false,
            _ => true
        }
    }
    pub fn stop(&mut self) {
        self.device = None;
        log::info("AudioDevice", "stop");
    }
    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_started() {
            self.stop();
            log::info("AudioDevice", "restarting");
        }else{
            log::info("AudioDevice", "starting");
        }
        let params = OutputDeviceParameters{ 
                channels_count: 2,
                sample_rate: 44100,
                channel_sample_count: 4410
            };
        let device = run_output_device( params, {
            //let proxy_render = proxy_render_clone;
            move |data: &mut [f32]| {
                log::tick();
                //let mut proxy_render_lock = proxy_render.lock()
                //l    .expect("panic on locking PROXY_audio_render");
                //proxy_render_lock.render( data );
            }
        });
        match device {
            Err(e) => {
                let errmsg = format!("{:?}",e);
                log::error("AudioDevice", &errmsg);
                return Err(e)
            },
            Ok(running_device) => self.device = Some(running_device),
        }
        Ok(())
    }
}


impl MidiSender for AudioDevice {
    fn invoke_reset(&mut self) {
        log::info("AudioDevice", "midi.RESET");
    }
    fn invoke_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32) {
        log::info("AudioDevice", "midi.invoke_midi_command");
    }
}


//










pub struct AudioDevice_OLD{
    sample_rate: usize,
    channel_sample_count: usize,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
    proxy_render: Arc<Mutex<ProxyRender>>,
}

//

impl MidiSender for AudioDevice_OLD {
    fn invoke_reset(&mut self) {
        log::info("AudioDevice", "midi.RESET");
        let proxy_lock = self.proxy_render.lock()
            .expect("can't lock proxy_render");
        match &proxy_lock.sound_render {
            None => {
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render.lock()
                    .expect("panic on locking Some(sound_render)");
                sound_render_lock.reset();
            }
        }
    }
    fn invoke_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32) {
        //log::info("AudioDevice", "midi.invoke_midi_command");
        let proxy_lock = self.proxy_render.lock()
            .expect("can't lock proxy_render");
        match &proxy_lock.sound_render {
            None => {
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render.lock()
                    .expect("panic on locking Some(sound_render)");
                sound_render_lock.process_midi_command( channel, command, data1, data2 );
          }
        }
    }
}


//
impl AudioDevice_OLD {
    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_started() {
            log::error("AudioDevice", "Device is still active!");
            Err("[ AudioDevice] E: device still active!".to_string().into() )
        }else{
            log::info("AudioDevice", "starting");
            let proxy_render_clone = self.proxy_render.clone();
            let params = OutputDeviceParameters{ 
                    channels_count: 2,
                    sample_rate: self.sample_rate,
                    channel_sample_count: self.channel_sample_count
                };
            let dev = run_output_device( params, {
                let proxy_render = proxy_render_clone;
                move |data: &mut [f32]| {
                    //log::tick();
                    let mut proxy_render_lock = proxy_render.lock()
                        .expect("panic on locking PROXY_audio_render");
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
        self.invoke_reset();
        self.device = None;
        log::info("AudioDevice", "stop!");
    }

    pub fn is_started(&self) -> bool {
        match self.device {
            None => false,
            _ => true
        }
    }

    pub fn get_parameters(&self) -> OutputDeviceParameters  {
        OutputDeviceParameters {
            sample_rate: self.sample_rate, 
            channels_count: 2,
            channel_sample_count: self.channel_sample_count
        }
    }

    pub fn set_soundrender(&mut self, new_soundrender: Option<Arc<Mutex<dyn SoundRender>>>) {
        let mut proxy_lock = self.proxy_render.lock()
            .expect("can't lock proxy_render");
        proxy_lock.sound_render = new_soundrender;
    }
}


//  //  //  //  //  //  //  //  //
pub trait SoundRender: Sync + Send + MidiReceiver {
    fn render(&mut self, data: &mut [f32]);
}


//
/*
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
*/
