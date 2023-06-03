use rustysynth::*;
//  //  //  //  //  //  //

use crate::raadbg::log;

use super::audio_device::SoundRender;
use super::midi_rx_tx::MidiReceiver;
//  //  //  //  //  //  //

pub struct RustySynthWrapper{
    parameters: SynthesizerSettings,
    left_buf:  Vec<f32>,
    right_buf: Vec<f32>,
    synth: Synthesizer,
}
impl Drop for RustySynthWrapper {
    fn drop(&mut self) {
        self.reset();
        log::drop("RustySynthWrapper");
    }
}
impl RustySynthWrapper {
    pub fn new( sample_rate: i32, channel_sample_count: usize ) -> Self {
        log::create("RustySynthWrapper");
        let mut init_params = SynthesizerSettings::new( sample_rate );
        let mut file = super::SF_PIANO.clone();
        let snd_fnt = Arc::new( SoundFont::new(&mut file).unwrap() );
        Self{
            parameters: init_params,
            left_buf:  vec![ 0_f32; channel_sample_count],
            right_buf: vec![ 0_f32; channel_sample_count],
            synth: Synthesizer::new(snd_fnt, &init_params).unwrap()
        }
    }


//    pub fn tst_AB(&mut self) {
//        let mut midi = crate::audio::midi_sequencer::MIDISequencer::default();
//        let mut fl = super::SF_PIANO.clone();
//        let sf = Arc::new( SoundFont::new(&mut fl).unwrap() );
//        let _res = midi.load( &sf ).unwrap();
//            midi.tst();
//        //self.render = Arc::new(Mutex::new(midi));
//    }
}

//
//
impl SoundRender for RustySynthWrapper {
    fn render(&mut self, data: &mut [f32]) {
        log::tick();
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
        self.synth.process_midi_message(channel, command, 
                            data1, data2)
    }
}

//
//
//
//



/*

// rustysynth wrapper
pub struct MIDISequencer{
    parameters: SynthesizerSettings,
    synth: Option< Box<Synthesizer> >,
}

//
impl MIDISequencer {
    pub fn new( sample_rate: i32 ) -> Self{
        let mut init_params = SynthesizerSettings::new( sample_rate );
        log::create("MIDISequencer");
        MIDISequencer{ 
            parameters: init_params,
            synth: None
        }
    }
}

//
impl MIDISequencer {
    pub fn tst(&mut self) {
        if self.synth.is_some() {
            self.synth.as_mut().unwrap().note_on(0,60,100);
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


impl AudioRender for MIDISequencer {
    fn render(&mut self, data: &mut [f32], 
              left_buf: &mut [f32], right_buf: &mut [f32] ) {

        log::tick();

        if self.synth.is_some() {
            self.synth.as_mut().unwrap().render(&mut left_buf[..], &mut right_buf[..]);
        }

        for (i, lvalue) in left_buf.iter().enumerate() {
            data[i * 2] = *lvalue;
            data[i * 2 + 1] = right_buf[i];
        }
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
        let mut midi = MIDISequencer::new( 0 );
        let mut file = File::open("Horn.SF2").unwrap();
        let sf = Arc::new( SoundFont::new(&mut file).unwrap() );
        let _res = midi.load( &sf );
        assert!(midi.synth.is_some() );
    }
}

*/
