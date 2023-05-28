use std::error::Error;


pub struct MidiSequencer {
//    audio: Result< Box<dyn BaseAudioOutputDevice>, Box<dyn Error> >,
}

impl MidiSequencer {
    pub fn new() -> Self {
       Default::default() 
    }
    pub fn tst(&mut self){
        println!("no yet");
    }
}

impl Default for MidiSequencer {
    fn default() -> Self {
        Self{ 
//            audio: create_audio()
        }
    }
}




use tinyaudio::prelude::*;


fn create_audio() -> Result< Box<dyn BaseAudioOutputDevice>, Box<dyn Error> > {
    
    println!("^^^^^");
    return Err( "rrr".to_string().into() );

    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    let device = run_output_device(params, {
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
    });
    return device;
    //std::thread::sleep(std::time::Duration::from_secs(5));
    //
}
