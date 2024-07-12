use crate::slintui::SlintCommonMemory;
use crate::slintui::SlintUIWindowManager;
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

pub fn new_main_window(memory: SlintCommonMemory) -> Result<MainWindow> {
    let main_window = MainWindow::new().map_err(|e| UiError::SlintError(e.to_string()))?;

    println!(
        "main port is: {}",
        memory
            .get_port()
            .map_err(|_| UiError::SlintError("error in load port from config".into()))?
    );
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
