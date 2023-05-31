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

    pub fn load(&mut self, sound_font: &Arc<SoundFont>) -> Result< (), SynthesizerError > {
            log::info("MIDISequencer", "start ");
            let new_synth = Synthesizer::new( sound_font, &self.parameters );
            match new_synth {
                Err(e) => {
                    let errmsg: String;
                    match e {
                        SynthesizerError::SampleRateOutOfRange(sample_rate) => {
                            errmsg = format!("SynthesizerError.SampleRateOutOfRange: {}", sample_rate);
                        },
                        SynthesizerError::BlockSizeOutOfRange(size) => {
                            errmsg = format!("SynthesizerError.BlockSizeOutOfRange: {}", size);
                        },
                        SynthesizerError::MaximumPolyphonyOutOfRange(size) => {
                            errmsg = format!("SynthesizerError.MaximumPolyphonyOutOfRange: {}", size);
                        },
                        _ => {
                            errmsg = format!("SynthesizerError.<unknown>");
                        },
                    }
                    log::error("MIDISequencer", &errmsg);
                    return Err(e);
                },
                Ok(loaded_synth) => self.synth = Some( Box::new(loaded_synth) ),
            }
            Ok(())
    }

}


impl crate::audio_device::AudioRender for MIDISequencer {
    fn render(&mut self, data: &mut [f32]) {

        log::tick();

        if let Some(synthesizer) = self.synth {
            synthesizer.render(&mut self.left_buf[..], &mut self.right_buf[..]);
        }
        for (i, value) in interleave!(self.left_buf.iter(),self.right_buf.iter()).enumerate() {
            data[i] = *value;
        }
        /*for (i, value) in self.left_buf.iter().interleave(self.right_buf.iter()).enumerate() {
            data[i] = *value;
        }*/
    }
}


//

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::*;
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
    #[test]
    fn load_sound_font() {
        let mut midi: MIDISequencer = Default::default();
        let mut file = File::open("Horn.SF2").unwrap();
        let sf = Arc::new( SoundFont::new(&mut file).unwrap() );
        let _res = midi.load( &sf );
        assert!(midi.synth.is_some() );
    }

    #[test]
    #[should_panic]
    fn error_sample_rate() {
        let mut midi = MIDISequencer::new( 0, 4410 );
        let mut file = File::open("Horn.SF2").unwrap();
        let sf = Arc::new( SoundFont::new(&mut file).unwrap() );
        let _res = midi.load( &sf );
        assert!(midi.synth.is_some() );
    }
    #[test]
    #[should_panic]
    fn error_sample_count() {
        let mut midi = MIDISequencer::new( 44100, 12345678901234567890 );
        let mut file = File::open("Horn.SF2").unwrap();
        let sf = Arc::new( SoundFont::new(&mut file).unwrap() );
        let _res = midi.load( &sf );
        assert!(midi.synth.is_some() );
    }
}

