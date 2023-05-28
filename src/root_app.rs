#![allow(non_snake_case)]

mod midi_sequencer;
use crate::root_app::midi_sequencer::*;

mod domik_view;
use crate::root_app::domik_view::*;


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RootApp {
    txt: String,

    #[serde(skip)]
    pressed: bool,

    #[serde(skip)]
    sequencer: MidiSequencer,

    #[serde(skip)]
    domikView: DomikView,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {
            txt:"<empty>".to_owned(), pressed:false, 
            sequencer: MidiSequencer::new(),
            domikView: DomikView::new()
        }
    }
}

impl RootApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}


impl eframe::App for RootApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {

        egui::Window::new(self.domikView.title.clone()).show( ctx, |ui| {
            self.domikView.updateUI( ui, &mut self.sequencer );
        });

        egui::Window::new("tst wnd").show( ctx, |ui| {
            ui.horizontal( |ui| {
                let btn = ui.button( "try to save TEXT" );
                ui.label( format!(" <{}>", self.pressed) );
                if btn.clicked(){
                    println!("clicked with PRESSURE!!!");
                    self.pressed = true;
            }
            });
            ui.text_edit_singleline(&mut self.txt);
            ui.label( format!("just edited: [{}]", self.txt) );
        });
    }
}
