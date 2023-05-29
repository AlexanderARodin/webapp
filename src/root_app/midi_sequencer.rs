use std::error::Error;


use tinyaudio::prelude::*;

// tinyaudio wrapper
pub struct AudioDevice{
    parameters: OutputDeviceParameters,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
}
impl AudioDevice{
    pub fn new( channels_count: usize, 
            sample_rate: usize, 
            channel_sample_count: usize ) -> Self{
        let init_params = OutputDeviceParameters {
            channels_count: channels_count,
            sample_rate: sample_rate,
            channel_sample_count: channel_sample_count
        };
        println!("[+AudioDevice]");
        AudioDevice{ 
            parameters: init_params,
            device: None
        }
    }
    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        println!("[ AudioDevice] start..");
        if self.is_started() {
            Err("[ AudioDevice] E: device still active!".to_string().into() )
        }else{
            let params = self.parameters.clone();
            let dev = run_output_device( params, {
                let mut clock = 0f32;
                move |data: &mut [f32]| {
                    gogo(params, &mut clock, data);
                }
            });
            match dev {
                Err(e) => return Err(e),
                Ok(running_dev) => self.device = Some(running_dev),
            }
            Ok(())
        }
    }
    pub fn stop(&mut self) {
        self.device = None;
        println!("[ AudioDevice] stop!");
    }
    pub fn is_started(&self) -> bool {
        match self.device {
            None => false,
            _ => true
        }
    }
}
impl Default for AudioDevice{
    fn default() -> Self {
        //Self::new( 2, 44100, 4410 )
        Self::new( 2, 100, 10 )
    }
}
impl Drop for AudioDevice {
    fn drop(&mut self) {
        self.stop();
        println!("[-AudioDevice]");
    }
}



fn gogo(params: OutputDeviceParameters, clock: &mut f32, data: &mut [f32] ) {
        for samples in data.chunks_mut(params.channels_count) {
            *clock = (*clock + 1.0) % params.sample_rate as f32;
            let value =
                (*clock * 440.0 * 2.0 * std::f32::consts::PI 
                    / params.sample_rate as f32).sin();
            for sample in samples {
                *sample = value;
            }
        }
}


pub fn tst() {
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };
    let _device = run_output_device(params, {
        let mut clock = 0f32;
        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                clock = (clock + 1.0) % params.sample_rate as f32;
                let value =
                    (clock * 440.0 * 2.0 * std::f32::consts::PI / params.sample_rate as f32).sin();
                for sample in samples {
                    *sample = value;
                }
            }
        }
    })
    .unwrap();
    
    std::thread::sleep(std::time::Duration::from_secs(2));
}
