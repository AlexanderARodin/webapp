const VERS: &str = "v8.01";

use std::sync::{Mutex,Arc};

use crate::audio::audio_device::{AudioDevice};
use crate::audio::simple_synth::{SimpleSynth};
use crate::audio::midi_rx_tx::MidiSender;

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
            ui.horizontal( |ui| {
                    let btn = ui.button(
                        format!("START! [status = {}]", 
                        audio_device.is_started()) 
                        );
                    if btn.clicked(){
                        let _res = audio_device.start();
                    }
                    let btnStop = ui.button( "STOP" );
                    if btnStop.clicked(){
                        audio_device.stop();
                    }
                });
            ui.separator();
            ui.horizontal( |ui| {
                    let btnA = ui.button( "note ON" );
                    if btnA.clicked(){
                        audio_device.invoke_note_on(1,60,127);
                    }
                    let btnA1 = ui.button( "note ON2" );
                    if btnA1.clicked(){
                        audio_device.invoke_note_on(1,60,64);
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
            ui.separator();
            ui.horizontal( |ui| {
                    let btnN = ui.button( "None" );
                    if btnN.clicked(){
                        audio_device.set_soundrender(None);
                    }
                    let settings = audio_device.get_params();
                    let btnS = ui.button( "SimpleSynth" );
                    if btnS.clicked(){
                        let simsyn = SimpleSynth::new( settings.sample_rate );
                        let mut dt: [f32; 10] = [0_f32; 10];
                        simsyn.render(&mut dt);
                        let simsyn_wrapper = Some(Arc::new(Mutex::new( simsyn as SoundRender )));
                        audio_device.set_soundrender(simsyn_wrapper);
                    }
                    let btnRA = ui.button( "RustySynt - A" );
                    if btnRA.clicked(){
                        audio_device.set_soundrender(None);
                    }
                    let btnRB = ui.button( "RustySynt - B" );
                    if btnRB.clicked(){
                        audio_device.set_soundrender(None);
                    }
                });
    }
}


