use entity::model::ui_config::UICommonMemoryMessage;
use entity::model::ui_config::UICommonMemoryWindowMessages;
use entity::model::ui_config::UIWindowManager;

use crate::uierrors::Result;
use crate::uierrors::UiError;

slint::slint! {
    export component MainWindow inherits Window {
        width: 600px;
        height: 600px;

        VerticalLayout {
            spacing: 20px;
            Text {
                text: "Main Window";
            }
        }
    }
}

pub fn new_main_window() -> Result<MainWindow> {
    let main_window = MainWindow::new().map_err(|e| UiError::SlintError(e.to_string()))?;

    Ok(main_window)
}

// --
pub struct MainWindowShared {
    win: MainWindow,
}

impl MainWindowShared {
    pub fn new() -> Result<MainWindowShared> {
        let win = new_main_window()?;
        Ok(Self { win })
    }
}

impl UIWindowManager for MainWindowShared {
    fn run_win(&self) {
        self.win.run().unwrap()
    }

    fn hide_win(&self) {
        self.win.hide().unwrap()
    }

    fn send_message(&self, msg: UICommonMemoryMessage) {
        match msg {
            UICommonMemoryMessage::Window(wmsg) => match wmsg {
                UICommonMemoryWindowMessages::OpenMain => self.run_win(),
                _ => {}
            },
        }
    }
}
