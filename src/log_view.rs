use crate::raadbg::log;

pub struct LogView {
    last_time: f64,
}

impl Default for LogView {
    fn default() -> Self {
        Self::new()
    }
}

impl LogView {
    pub fn new() -> Self {
        Self {last_time: 0.}
    }

    pub fn updateUI(&mut self, ui: &mut egui::Ui ) {
            ui.label( log::get() );

            ui.ctx().request_repaint();
            let time = ui.input(|i| i.time);
            let d_time = time - self.last_time;
            //println!("time: {}", d_time);
            self.last_time = time;
    }
}

