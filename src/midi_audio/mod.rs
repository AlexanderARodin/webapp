use std::error::Error;
use std::sync::{Arc,Mutex};
use tinyaudio::prelude::*;

use crate::raadbg::log;
use crate::midi_lib::MidiMessage;

mod render_holder;
use render_holder::RenderHolder;
pub use render_holder::SoundRender as SoundRender;

mod audio_device_parameters;
use audio_device_parameters::AudioDeviceParameters;
//  //  //  //  //  //  //  //  //


pub struct MidiAudio {
    params: AudioDeviceParameters,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
    render_holder: Arc<Mutex<RenderHolder>>,
}

impl MidiAudio {
    pub fn new( ) -> Self {
        log::create("MidiAudio");
        Self{ 
            params: Default::default(),
            device: None,
            render_holder: RenderHolder::new_arc_mutex(),
        }
    }
    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_active() {
            self.stop();
            log::info("MidiAudio", "restarting");
        }else{
            log::info("MidiAudio", "starting");
        }
        self.refresh_tick_time();
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
        let mut holder_lock = self.render_holder.lock()
            .expect("can't lock hoder_lock");
        holder_lock.sound_render = new_synth;
    }
    pub fn send_to_synth(&self, midi_msg: &MidiMessage) {
        let mut holder_lock = self.render_holder.lock()
            .expect("can't lock hoder_lock");
        match &holder_lock.sound_render {
            None => {
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render.lock()
                    .expect("panic on locking Some(sound_render)");
                let midi = midi_msg.to_midi_general();
                sound_render_lock.process_midi_command( midi.channel, 
                                                        midi.command, 
                                                        midi.data1, 
                                                        midi.data2 );
          }
        }
    }
    pub fn load_sequence(&mut self) {
    }

    pub fn get_sample_rate(&self) -> usize {
        self.params.sample_rate
    }
}


impl MidiAudio {
    fn run_device_loop(&mut self) -> Result< (), Box<dyn Error>> {
        let params = self.params.get_output_device_parameters();
        let render_holder_clone = self.render_holder.clone();

        let device = run_output_device( params, {
            let render_holder = render_holder_clone;
            let block_chunk = 2*self.params.block_size;
            let mut left :Vec<f32> = vec![ 0_f32; self.params.block_size ];
            let mut right:Vec<f32> = vec![ 0_f32; self.params.block_size ];
            move |data: &mut [f32]| {
                let mut render_holder_lock = render_holder.lock()
                    .expect("panic on locking render_holder_lock");
                for chunk in data.chunks_mut(block_chunk) {
                    render_holder_lock.render( &mut left, &mut right );
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

    fn refresh_tick_time(&self) {
        let mut holder_lock = self.render_holder.lock()
            .expect("panic on lockin holder_lock");
        holder_lock.tick_time = self.params.get_tick_time();
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


