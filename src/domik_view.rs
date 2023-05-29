const annot: &str = "v5.21";

use crate::dbg_utils::{getInterLog};
use crate::midi_sequencer::{AudioDevice};

pub struct DomikView {
    pub title: String,
}
impl Default for DomikView {
    fn default() -> Self {
        Self{ title: "simple DoMiKkk".to_owned() }
    }
}
impl DomikView {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui, 
                    audio_device: &mut AudioDevice) {
            ui.label( format!("WWWapp Template {}", annot) );
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
            ui.label( getInterLog() );
    }
}


