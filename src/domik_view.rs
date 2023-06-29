const VERS: &str = "v0.11.01";

use std::sync::{Mutex,Arc};

use crate::midi_audio::MidiAudio;

use crate::synths::simple_synth::SimpleSynth;
use crate::synths::rusty_synth_wrapper::RustySynthWrapper;
use crate::midi_lib::MidiMessage;



pub struct DomikView {
    pub title: String,
}
impl Default for DomikView {
    fn default() -> Self {
        Self::new()
    }
}
impl DomikView {
    pub fn new() -> Self {
        Self{ 
            title: "simple DoMiKkk".to_owned(), 
        }
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui, 
                    midi_audio: &mut MidiAudio) {
            ui.label( format!("WWWapp Template {}", VERS) );
            ui.separator();
            ui.label( format!("audio device status: [active = {}]", midi_audio.is_active() ) );
            ui.horizontal( |ui| {
                    let btn = ui.button("start");
                    if btn.clicked(){
                        let _res = midi_audio.start();
                    }
                    let btnStop = ui.button( "stop" );
                    if btnStop.clicked(){
                        midi_audio.stop();
                    }
                });
            ui.separator();
            ui.label("select synthesizer:");
            ui.horizontal( |ui| {
                    let btnN = ui.button( "None" );
                    if btnN.clicked(){
                        midi_audio.install_synth(None);
                    }
                    
                    let sample_rate = midi_audio.get_sample_rate();
                    let btnS = ui.button( "SimpleSynth" );
                    if btnS.clicked(){
                        let simsyn = SimpleSynth::new( &sample_rate );
                        let simsyn_wrapper = Arc::new(Mutex::new( simsyn ));
                        midi_audio.install_synth( Some(simsyn_wrapper) );
                    }
                    let btnRA = ui.button( "RustySynt - Strings" );
                    if btnRA.clicked(){
                        if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, false ) {
                            let ryssyn_wrapper = Arc::new(Mutex::new( ryssyn ));
                            midi_audio.install_synth( Some(ryssyn_wrapper) );
                        }
                    }
                    let btnRB = ui.button( "RustySynt - Piano" );
                    if btnRB.clicked(){
                        if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, true ) {
                            let ryssyn_wrapper = Arc::new(Mutex::new( ryssyn ));
                            midi_audio.install_synth( Some(ryssyn_wrapper) );
                        }
                    }
                });
            ui.separator();
            ui.separator();
            ui.label("playing notes:");
            ui.horizontal( |ui| {
                    let btnO = ui.button( "[#]" );
                    if btnO.clicked(){
                        midi_audio.load_sequence();
                    }
                    ui.separator();
                    let btnA = ui.button( "note ON" );
                    if btnA.clicked(){
                        let midi = MidiMessage::NoteOn(1,60,127);
                        midi_audio.send_to_synth( &midi );
                    }
                    let btnA1 = ui.button( "note ON2" );
                    if btnA1.clicked(){
                        let midi = MidiMessage::NoteOn(1,67,64);
                        midi_audio.send_to_synth( &midi );
                    }
                    let btnA2 = ui.button( "note ON2" );
                    if btnA2.clicked(){
                        let midi = MidiMessage::NoteOn(1,72,1);
                        midi_audio.send_to_synth( &midi );
                    }
                    let btnB = ui.button( "note OFF" );
                    if btnB.clicked(){
                        let midi = MidiMessage::NoteOff(1,60,100);
                        midi_audio.send_to_synth( &midi );
                    }
                });
    }
}


