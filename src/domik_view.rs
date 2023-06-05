const VERS: &str = "v0.10.01";

use std::sync::{Mutex,Arc};

use crate::audio::audio_device::{AudioDevice, SoundRender};
use crate::audio::midi_rx_tx::MidiSender;

use crate::audio::simple_synth::SimpleSynth;
use crate::audio::rusty_synth_wrapper::RustySynthWrapper;



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
                    audio_device: &mut AudioDevice ) {
            ui.label( format!("WWWapp Template {}", VERS) );
            ui.separator();
            ui.label( format!("audio device status: [active = {}]", audio_device.is_started() ) );
            ui.horizontal( |ui| {
                    let btn = ui.button("start");
                    if btn.clicked(){
                        let _res = audio_device.start();
                    }
                    let btnStop = ui.button( "stop" );
                    if btnStop.clicked(){
                        audio_device.stop();
                    }
                });
            ui.separator();
            ui.label("select synthesizer:");
            ui.horizontal( |ui| {
                    let btnN = ui.button( "None" );
                    if btnN.clicked(){
                        audio_device.set_soundrender(None);
                    }
                    let device_params= audio_device.get_parameters();
                    let btnS = ui.button( "SimpleSynth" );
                    if btnS.clicked(){
                        let simsyn = SimpleSynth::new( &device_params );
                        let simsyn_wrapper = Arc::new(Mutex::new( simsyn ));
                        audio_device.set_soundrender( Some(simsyn_wrapper) );
                    }
                    let btnRA = ui.button( "RustySynt - Strings" );
                    if btnRA.clicked(){
                        if let Ok(ryssyn) = RustySynthWrapper::new( &device_params, false ) {
                            let ryssyn_wrapper = Arc::new(Mutex::new( ryssyn ));
                            audio_device.set_soundrender( Some(ryssyn_wrapper) );
                        }
                    }
                    let btnRB = ui.button( "RustySynt - Piano" );
                    if btnRB.clicked(){
                        if let Ok(ryssyn) = RustySynthWrapper::new( &device_params, true ) {
                            let ryssyn_wrapper = Arc::new(Mutex::new( ryssyn ));
                            audio_device.set_soundrender( Some(ryssyn_wrapper) );
                        }
                    }
                });
            ui.separator();
            ui.separator();
            ui.label("playing notes:");
            ui.horizontal( |ui| {
                    let btnA = ui.button( "note ON" );
                    if btnA.clicked(){
                        audio_device.invoke_note_on(1,60,127);
                    }
                    let btnA1 = ui.button( "note ON2" );
                    if btnA1.clicked(){
                        audio_device.invoke_note_on(1,67,64);
                    }
                    let btnA2 = ui.button( "note ON2" );
                    if btnA2.clicked(){
                        audio_device.invoke_note_on(1,72,1);
                    }
                    let btnB = ui.button( "note OFF" );
                    if btnB.clicked(){
                        audio_device.invoke_note_off(1,60);
                    }
                });
    }
}


