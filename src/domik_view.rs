const VERS: &str = "v7.01";

use crate::audio_device::{MidiDevice};

pub struct DomikView {
    pub title: String,
    audio: MidiDevice,
}
impl Default for DomikView {
    fn default() -> Self {
        Self{ title: "simple DoMiKkk".to_owned(), audio: MidiDevice::new( 22050,4410) }
    }
}
impl DomikView {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui, 
                    audio_device: &mut MidiDevice) {
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
                audio_device.tst_A();
            }
            let btnB = ui.button( "tst B" );
            if btnB.clicked(){
                audio_device.tst_B();
            }
            ui.separator();
            let alt_btn = ui.button( "alt-btn" );
            if alt_btn.clicked(){
                let _ = self.audio.start();
            }
    }
}


