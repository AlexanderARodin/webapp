use std::error::Error;
use std::sync::{Arc,Mutex};
use tinyaudio::prelude::*;

use crate::raadbg::log;

//  //  //  //  //  //  //
use super::sequencer::*;

use super::midi_rx_tx::*;
//  //  //  //  //  //  //  //  //

pub trait SoundRender: Sync + Send + MidiReceiver {
    //fn render(&mut self, data: &mut [f32]);
    fn render(&mut self, left: &mut [f32], right: &mut [f32]);
}


//  //  //  //  //  //  //


// TinyAudio wrapper
pub struct AudioDevice{
    sample_rate: usize,
    channel_sample_count: usize,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
    sequencer: Arc<Mutex<Sequencer>>,
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        self.stop();
        log::drop("AudioDevice");
    }
}
impl AudioDevice {
    pub fn new( sample_rate: usize, channel_sample_count: usize ) -> Self {
        log::create("AudioDevice");
        let params = OutputDeviceParameters {
                sample_rate: sample_rate, 
                channels_count: 2,
                channel_sample_count: channel_sample_count
            };
        Self{ 
            sample_rate,
            channel_sample_count,
            device: None,
            sequencer: Sequencer::new_arc_mutex(&params),
        }
    }
}

//
impl AudioDevice{
    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_started() {
            log::error("AudioDevice", "Device is still active!");
            Err("[ AudioDevice] E: device still active!".to_string().into() )
        }else{
            log::info("AudioDevice", "starting");
            let sequencer_clone = self.sequencer.clone();
            let params = OutputDeviceParameters{ 
                    channels_count: 2,
                    sample_rate: self.sample_rate,
                    channel_sample_count: self.channel_sample_count
                };
            let dev = run_output_device( params, {
                let sequencer = sequencer_clone;
                move |data: &mut [f32]| {
                    //log::tick();
                    for chunk in data.chunks_mut(441*2) {
                        let mut sequencer_lock = sequencer.lock()
                            .expect("panic on locking PROXY_audio_render");
                        sequencer_lock.render_all( chunk );
                    }
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
        let mut seq_lock = self.sequencer.lock()
            .expect("can't lock proxy_render");
        seq_lock.set_soundrender( new_soundrender );
    }
    pub fn get_sequencer(&self) -> Arc<Mutex<Sequencer>> {
        self.sequencer.clone()
    }
}


//
impl MidiSender for AudioDevice {
    fn invoke_reset(&mut self) {
        let mut seq_lock = self.sequencer.lock()
            .expect("can't lock proxy_render");
        seq_lock.reset();
    }
    fn invoke_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32) {
        let mut seq_lock = self.sequencer.lock()
            .expect("can't lock proxy_render");
        seq_lock.process_midi_command( channel, command, data1, data2 );
    }
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
