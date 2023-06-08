#![allow(non_snake_case)]

use crate::log_view::LogView;
use crate::domik_view::*;
use crate::audio::audio_device::AudioDevice;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MainApp {
    txt: String,
    #[serde(skip)]
    pressed: bool,

    #[serde(skip)]
    log_view: LogView,
    
    #[serde(skip)]
    audio_device: AudioDevice,
    #[serde(skip)]
    domikView: DomikView,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            txt:"<empty>".to_owned(), pressed:false, 
            log_view: LogView::new(),
            audio_device: AudioDevice::new( 44100/2, 441* 50),
            domikView: DomikView::new()
        }
    }
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}


impl eframe::App for MainApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {

        egui::Window::new("logs").show( ctx, |ui| {
            self.log_view.updateUI( ui );
        });

        egui::Window::new(self.domikView.title.clone()).show( ctx, |ui| {
            self.domikView.updateUI( ui, &mut self.audio_device );
        });

        egui::CentralPanel::default()
            .show( ctx, |ui| {
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


