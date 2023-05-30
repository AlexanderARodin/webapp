use std::error::Error;
use std::sync::Arc;

use crate::raadbg::log;

use rustysynth::*;

// rustysynth wrapper
pub struct MIDISequencer{
    parameters: SynthesizerSettings,
    left_buf: Vec<f32>,
    right_buf: Vec<f32>,
    synth: Option< Box<Synthesizer> >,
}

//
impl Default for MIDISequencer {
    fn default() -> Self {
        Self::new( 44100, 4410 )
    }
}

impl Drop for MIDISequencer {
    fn drop(&mut self) {
        log::drop("MIDISequencer");
    }
}

//
impl MIDISequencer {

    pub fn new( sample_rate: i32, 
                channel_sample_count: usize ) -> Self{
        let init_params = SynthesizerSettings::new( sample_rate );
        log::create("MIDISequencer");
        MIDISequencer{ 
            parameters: init_params,
            left_buf: vec![0_f32; channel_sample_count],
            right_buf: vec![0_f32; channel_sample_count],
            synth: None
        }
    }

    pub fn load(&mut self, sound_font: SoundFont) -> Result< (), SynthesizerError > {
            log::info("MIDISequencer", "start ");
            let new_synth = Synthesizer::new( &Arc::new(sound_font), &self.parameters );
            match new_synth {
                Err(e) => {
                    let errmsg: String;
                    match e {
                        SynthesizerError::SampleRateOutOfRange(sample_rate) => {
                            errmsg = format!("SynthesizerError {}", sample_rate);
                        },
                        SynthesizerError::BlockSizeOutOfRange(size) => {
                            errmsg = format!("SynthesizerError {}", size);
                        },
                        SynthesizerError::MaximumPolyphonyOutOfRange(size) => {
                            errmsg = format!("SynthesizerError {}", size);
                        },
                        _ => {
                            errmsg = format!("SynthesizerError");
                        },
                    }
                    log::error("MIDISequencer", &errmsg);
                    log::error("MIDISequencer", e.description());
                    return Err(e);
                },
                Ok(loaded_synth) => self.synth = Some( Box::new(loaded_synth) ),
            }
            Ok(())
    }

}


//
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn default_sample_rate() {
        let midi: MIDISequencer = Default::default();
        assert!(midi.parameters.sample_rate == 44100);
    }
    #[test]
    fn default_none_synthesizer() {
        let midi: MIDISequencer = Default::default();
        assert!(midi.synth.is_none() );
    }
}

