const annot: &str = "v5.04";

use crate::root_app::midi_sequencer;


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
                    sequencer: &mut midi_sequencer::MidiSequencer ) {
            ui.label( format!("WWWapp Template {}", annot) );
            ui.separator();
            let btn = ui.button( "try to save TEXT" );
            if btn.clicked(){
                println!("clicked simple!");
                sequencer.tst();
            }
    }
}


