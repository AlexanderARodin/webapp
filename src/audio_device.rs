use std::error::Error;

use crate::raadbg::log;

use tinyaudio::prelude::*;

// tinyaudio wrapper
pub struct AudioDevice{
    parameters: OutputDeviceParameters,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
}

//
impl Default for AudioDevice{
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
            device: None
        }
    }

    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_started() { log::error("AudioDevice", "Device is still active!");
            Err("[ AudioDevice] E: device still active!".to_string().into() )
        }else{
            log::info("AudioDevice", "start ");
            let params = self.parameters.clone();
            let dev = run_output_device( params, {
                let mut clock: f32 = 0.;
                move |data: &mut [f32]| {
                    gogo(params, &mut clock, data);
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
}


//

fn gogo(params: OutputDeviceParameters, clock: &mut f32, data: &mut [f32] ) {
    log::tick();
        for samples in data.chunks_mut(params.channels_count) {
            *clock = (*clock + 1.0) % params.sample_rate as f32;
            let value = ( 
                *clock * 440.0 * 2.0 * std::f32::consts::PI 
                / params.sample_rate as f32
                )
                .sin() * 0.2;
            for sample in samples {
                *sample = value;
            }
        }
}


