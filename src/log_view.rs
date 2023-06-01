use crate::raadbg::log;

pub struct LogView {
}

impl LogView {
    fn new() -> Self {
        Self {}
    }

    pub fn updateUI(&mut self, ui: &mut egui::Ui ) {
            ui.label( log::get() );
    }
}

