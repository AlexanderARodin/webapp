#![allow(non_snake_case)]

mod root_app;
use root_app::{RootApp};

mod midi_sequencer;
use midi_sequencer::*;

mod dbg_utils;
use dbg_utils::{appendInterLog};


#[ cfg(not(target_arch = "wasm32")) ]
fn main() -> Result<(), eframe::Error> {
    println!("MAIN has beed entered..");
    appendInterLog("NORM pseudo log:");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(200., 300.)),
        ..Default::default()
    };

    {
        eframe::run_native(
            "egui CrossApp",
            options,
            Box::new( |cc| Box::new(RootApp::new(cc)) )
        )
    }
}


#[ cfg(target_arch = "wasm32") ]
fn main() {
    println!("in WASM doen't work..");
    appendInterLog("WASM pseudo log:");

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "raa_canvas_id",
            options,
            Box::new( |cc| Box::new(RootApp::new(cc)) ),
        )
        .await
        .expect("failure with starting EFRAME");
    });
}


