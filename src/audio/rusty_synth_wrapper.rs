use std::sync::Arc;
use rustysynth::*;

use crate::raadbg::log;
use super::super::midi_audio::SoundRender;
use super::super::midi_lib::MidiReceiver;
//  //  //  //  //  //  //


pub struct RustySynthWrapper{
    synth: Synthesizer,
}
impl Drop for RustySynthWrapper {
    fn drop(&mut self) {
        self.reset();
        log::drop("RustySynthWrapper");
    }
}
impl RustySynthWrapper {
    pub fn new( sample_rate: &usize, font_type: bool ) -> Result<Self, SynthesizerError> {
        log::create("RustySynthWrapper");
        let init_params = SynthesizerSettings::new( *sample_rate as i32 );
        let mut file = match font_type {
            true => super::SF_PIANO.clone(),
            false => super::SF_STRINGS.clone()
        };
        let snd_fnt = Arc::new( SoundFont::new(&mut file).unwrap() );
        let new_synth = Synthesizer::new(&snd_fnt, &init_params);
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
                Err(e)
                },
            Ok(loaded_synth) => Ok(
                    Self{
                        synth: loaded_synth
                    }
            )
        }
    }
}

//
//
impl SoundRender for RustySynthWrapper {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        //log::tick();
        self.synth.render(&mut left[..], &mut right[..]);
    }
    fn get_as_midi_receiver(&mut self) -> &mut dyn MidiReceiver {
        self
    }
}


//
//
impl MidiReceiver for RustySynthWrapper {
    fn reset(&mut self) {
        log::info("SimpleSynth", "reset");
        self.synth.reset();
    }
    fn process_midi_command(&mut self, 
                            channel: i32, command: i32, 
                            data1: i32, data2: i32) 
    {
        //log::info("SimpleSynth", "process_midi_command");
        self.synth.process_midi_message(channel, command, 
                            data1, data2)
    }
}

//
//
//
//



/*

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
        let mut midi = MIDISequencer::new( 0 );
        let mut file = File::open("Horn.SF2").unwrap();
        let sf = Arc::new( SoundFont::new(&mut file).unwrap() );
        let _res = midi.load( &sf );
        assert!(midi.synth.is_some() );
    }
}

*/
