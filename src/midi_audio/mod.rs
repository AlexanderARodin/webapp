use std::error::Error;
use std::sync::{Arc,Mutex};
use tinyaudio::prelude::*;

use crate::raadbg::log;
use super::proxy_render::*;
use super::super::midi_rx_tx::*;

use super::audio_device_parameters::AudioDeviceParameters;
//  //  //  //  //  //  //  //  //




pub struct MidiAudio {
}

impl MidiAudio {
    pub fn start(&mut self) {
    }
    pub fn stop(&mut self) {
    }
    pub fn is_active(&self) -> {
        false
    }
}

/*




pub struct AudioDevice{ 
    params: AudioDeviceParameters,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
    proxy_render: Arc<Mutex<ProxyRender>>,
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        if self.is_started() {
            self.stop();
        }
        log::drop("AudioDevice");
    }
}
impl AudioDevice {
    pub fn new( ) -> Self {
        log::create("AudioDevice");
        Self{ 
            params: Default::default(),
            device: None,
            proxy_render: ProxyRender::new_arc_mutex(),
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
        self.refresh_tick_time();
        self.run_device_loop()
    }

    fn refresh_tick_time(&self) {
        let mut render_lock = self.proxy_render.lock()
            .expect("panic on lockin PROXY_audio_render");
        render_lock.tick_time = self.params.get_tick_time();
    }

    fn run_device_loop(&mut self) -> Result< (), Box<dyn Error>> {
        let params = self.params.get_output_device_parameters();
        let proxy_render_clone = self.proxy_render.clone();

        let device = run_output_device( params, {
            let proxy_render = proxy_render_clone;
            let block_chunk = 2*self.params.block_size;
            let mut left :Vec<f32> = vec![ 0_f32; self.params.block_size ];
            let mut right:Vec<f32> = vec![ 0_f32; self.params.block_size ];
            move |data: &mut [f32]| {
                //log::tick();
                let mut proxy_render_lock = proxy_render.lock()
                    .expect("panic on locking PROXY_audio_render");
                for chunk in data.chunks_mut(block_chunk) {
                    proxy_render_lock.render( &mut left, &mut right );
                    for (i, l_sample) in left.iter().enumerate() {
                        chunk[i*2] = *l_sample;
                        chunk[i*2 + 1] = right[i];
                    }
                }
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

    pub fn get_sample_rate(&self) -> usize {
        self.params.sample_rate
    }
    pub fn set_soundrender(&mut self, new_soundrender: Option<Arc<Mutex<dyn SoundRender>>>) {
        let mut proxy_lock = self.proxy_render.lock()
            .expect("can't lock proxy_render");
        proxy_lock.sound_render = new_soundrender;
    }
}


impl MidiSender for AudioDevice {
    fn invoke_reset(&mut self) {
        log::info("AudioDevice", "midi.RESET");
    }
    fn invoke_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32) {
        log::info("AudioDevice", "midi.invoke_midi_command");
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

*/
