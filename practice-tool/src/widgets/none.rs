use practice_tool_core::crossbeam_channel;

use practice_tool_core::widgets::Widget;
pub struct NoneWidget {
}

impl NoneWidget {
}

impl Widget for NoneWidget {
    fn render(&mut self, _ui: &imgui::Ui) {
        ();
    }

    fn interact(&mut self, _ui: &imgui::Ui) {
        ()
    }

    fn action(&mut self) {
      ()
    }

    fn log(&mut self, _tx: crossbeam_channel::Sender<String>) {
      ()
    }
}
