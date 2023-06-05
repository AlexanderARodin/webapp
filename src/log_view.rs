use crate::raadbg::log;

pub struct LogView {
}

impl Default for LogView {
    fn default() -> Self {
        Self::new()
    }
}

impl LogView {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn updateUI(&mut self, ui: &mut egui::Ui ) {
            ui.label( log::get() );
    }
}

