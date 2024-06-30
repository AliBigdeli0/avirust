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

pub fn run_main_window() -> Result<(), UiError> {
    let main_window = MainWindow::new().map_err(|e| UiError::SlintError(e.to_string()))?;

    main_window
        .run()
        .map_err(|e| UiError::SlintError(e.to_string()))?;

    Ok(())
}
