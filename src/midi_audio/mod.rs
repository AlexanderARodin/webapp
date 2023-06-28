use crate::raadbg::log;

mod proxy_render;
use proxy_render::ProxyRender;
pub use proxy_render::SoundRender as SoundRender;

mod audio_device_parameters;
use audio_device_parameters::AudioDeviceParameters;

use std::error::Error;
use std::sync::{Arc,Mutex};
use tinyaudio::prelude::*;
//  //  //  //  //  //  //  //  //




pub struct MidiAudio {
    params: AudioDeviceParameters,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
    proxy_render: Arc<Mutex<ProxyRender>>,
}

impl MidiAudio {
    pub fn new( ) -> Self {
        log::create("MidiAudio");
        Self{ 
            params: Default::default(),
            device: None,
            proxy_render: ProxyRender::new_arc_mutex(),
        }
    }
    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_active() {
            self.stop();
            log::info("MidiAudio", "restarting");
        }else{
            log::info("MidiAudio", "starting");
        }
        //self.refresh_tick_time();
        self.run_device_loop()
    }
    pub fn stop(&mut self) {
        self.device = None;
        log::info("MidiAudio", "stop");
    }
    pub fn is_active(&self) -> bool {
        match self.device {
            None => false,
            _ => true
        }
    }

    pub fn install_synth(&mut self, new_synth: Option<Arc<Mutex<dyn SoundRender>>>) {
        //let mut proxy_lock = self.proxy_render.lock()
        //    .expect("can't lock proxy_render");
        //proxy_lock.sound_render = new_soundrender;
    }
    pub fn send_to_synth(&self) {
    }
    pub fn load_sequence(&mut self) {
    }

    pub fn get_sample_rate(&self) -> usize {
        self.params.sample_rate
    }
}


impl Drop for MidiAudio {
    fn drop(&mut self) {
        if self.is_active() {
            self.stop();
        }
        log::drop("MidiAudio");
    }
}




impl MidiAudio {
    fn run_device_loop(&mut self) -> Result< (), Box<dyn Error>> {
        /*
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
        */
        Ok(())
    }
}






/*
impl AudioDevice {

    fn refresh_tick_time(&self) {
        let mut render_lock = self.proxy_render.lock()
            .expect("panic on lockin PROXY_audio_render");
        render_lock.tick_time = self.params.get_tick_time();
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
