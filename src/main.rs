#![allow(non_snake_case)]

mod log_view;
mod main_app;
use main_app::MainApp;

mod domik_view;
mod audio;

mod raadbg;
use raadbg::log;


#[ cfg(not(target_arch = "wasm32")) ]
fn main() -> Result<(), eframe::Error> {
    log::simple("MAIN has beed entered..");
    ttt();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(200., 300.)),
        ..Default::default()
    };

    {
        eframe::run_native(
            "egui CrossApp",
            options,
            Box::new( |cc| Box::new(MainApp::new(cc)) )
        )
    }
}


#[ cfg(target_arch = "wasm32") ]
fn main() {
    log::simple("wasmMAIN has beed entered..");
    ttt();

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "raa_canvas_id",
            options,
            Box::new( |cc| Box::new(MainApp::new(cc)) ),
        )
        .await
        .expect("failure with starting EFRAME");
    });
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
