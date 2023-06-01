
pub mod audio_device;
mod proxy_render;
mod simple_synth;
mod midi_sequencer;


static SF_PIANO:   &'static [u8] = include_bytes!("../SoundFonts/Piano Grand.SF2");
//static SF_STRINGS: &'static [u8] = include_bytes!("../SoundFonts/String Marcato.SF2");
//static SF_ORGAN:   &'static [u8] = include_bytes!("../SoundFonts/Organ Chorus.SF2");
