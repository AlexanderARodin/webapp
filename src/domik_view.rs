const VERS: &str = "v7.02";

use crate::audio::audio_device::{AudioDevice};
use crate::audio::midi_rx_tx::*;

pub struct DomikView {
    pub title: String,
    //audio: AudioDevice,
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
            //audio: AudioDevice::new( 22050,4410) 
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
                })
            ui.separator();
            ui.horizontal( |ui| {
                    let btnA = ui.button( "note ON" );
                    if btnA.clicked(){
                        audio_device.invoke_note_on(1,60,0);
                    }
                    let btnA2 = ui.button( "note ON2" );
                    if btnA2.clicked(){
                        audio_device.invoke_note_on(1,72,0);
                    }
                    let btnB = ui.button( "note OFF" );
                    if btnB.clicked(){
                        audio_device.invoke_note_off(1,60);
                    }
                })
            ui.separator();
            let alt_btn = ui.button( "invoke RESET" );
            if alt_btn.clicked(){
                //let _ = self.audio.start();
                audio_device.invoke_reset();
            }
    }
}


