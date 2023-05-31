const VERS: &str = "v6.04";

use crate::raadbg::log;
use crate::audio_device::{AudioDevice};

pub struct DomikView {
    pub title: String,
    audio: AudioDevice,
}
impl Default for DomikView {
    fn default() -> Self {
        Self{ title: "simple DoMiKkk".to_owned(), audio: AudioDevice::new( 44100,4410) }
    }
}
impl DomikView {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui, 
                    audio_device: &mut AudioDevice) {
            ui.label( format!("WWWapp Template {}", VERS) );
            ui.separator();
            let btn = ui.button( format!("audio status = {}", audio_device.is_started()) );
            if btn.clicked(){
                let res = audio_device.start();
                println!("result: {:?}", res);
            }
            let btnStop = ui.button( "stop" );
            if btnStop.clicked(){
                audio_device.stop();
            }
            ui.separator();
            let alt_btn = ui.button( "alt-btn" );
            if alt_btn.clicked(){
                println!("-------->");
                self.audio.start();
            }
            ui.separator();
            ui.label( log::get() );
    }
}


