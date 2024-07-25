use crate::slintui::SlintCommonMemory;
use crate::slintui::SlintUIWindowManager;
use crate::uierrors::Result;
use crate::uierrors::UiError;

slint::slint! {
    import { Button, HorizontalBox, VerticalBox} from "std-widgets.slint";
    import { StatusBar } from "src/slintui/forms/controls/main/statusbar.slint";

    export component MainWindow inherits Window {
        width: 800px;
        height: 600px;

        callback logout;

        GridLayout {
            padding: 0px;
            spacing: 0px;
            width: parent.width;
            height: parent.height;

            Text {
                text: "Main Window";
                row: 0;
            }

            StatusBar {
                row:10;
                backcolor: root.background.darker(1);
                height: 30px;
                logoutcallback => {
                    root.logout();
                }
            }
        }
    }
}

pub fn new_main_window(memory: SlintCommonMemory) -> Result<MainWindow> {
    let main_window = MainWindow::new().map_err(|e| UiError::SlintError(e.to_string()))?;

    main_window.on_logout(move || memory.send_message("logout"));

    Ok(main_window)
}

// --
pub struct MainWindowShared {
    win: MainWindow,
    _memory: SlintCommonMemory,
}

impl MainWindowShared {
    pub fn new(memory: SlintCommonMemory) -> Result<MainWindowShared> {
        let win = new_main_window(memory.clone())?;
        Ok(Self {
            win,
            _memory: memory,
        })
    }
}

impl SlintUIWindowManager for MainWindowShared {
    fn run_win(&self) -> Result<()> {
        self.win
            .run()
            .map_err(|e| UiError::SlintError(e.to_string()))?;
        Ok(())
    }

    fn show_win(&self) -> Result<()> {
        self.win
            .show()
            .map_err(|e| UiError::SlintError(e.to_string()))?;
        Ok(())
    }

    fn hide_win(&self) -> Result<()> {
        self.win
            .hide()
            .map_err(|e| UiError::SlintError(e.to_string()))?;
        Ok(())
    }
}
