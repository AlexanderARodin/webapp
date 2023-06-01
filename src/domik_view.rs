const VERS: &str = "v7.02";

use crate::audio::audio_device::{AudioDevice};
use crate::audio::midi_controller::*;

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
                    audio_device: &mut AudioDevice) {
            ui.label( format!("WWWapp Template {}", VERS) );
            ui.separator();
            let btn = ui.button( format!("audio status = {}", audio_device.is_started()) );
            if btn.clicked(){
                let _res = audio_device.start();
            }
            let btnStop = ui.button( "stop" );
            if btnStop.clicked(){
                audio_device.stop();
            }
            ui.separator();

            let btnA = ui.button( "tst A" );
            if btnA.clicked(){
                //audio_device.tst_A();
                audio_device.note_on(0,0,0);
            }
            let btnB = ui.button( "tst B" );
            if btnB.clicked(){
                audio_device.note_off(2,0,0);
            }
            ui.separator();
            let alt_btn = ui.button( "alt-btn" );
            if alt_btn.clicked(){
                //let _ = self.audio.start();
                audio_device.reset();
            }
    }
}


