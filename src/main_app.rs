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
            audio_device: Default::default(),
            domikView: DomikView::new()
        }
    }
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        ttt();
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




use tinyaudio::prelude::*;

fn ttt() {
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    let _device = run_output_device(params, {
        let mut clock = 0f32;
        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                clock = (clock + 1.0) % params.sample_rate as f32;
                let value =
                    (clock * 440.0 * 2.0 * std::f32::consts::PI / params.sample_rate as f32).sin();
                for sample in samples {
                    *sample = value;
                }
            }
        }
    })
    .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}

