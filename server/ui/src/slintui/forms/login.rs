use crate::uierrors::UiError;

slint::slint! {
    import { Button } from "std-widgets.slint";

    export  component LoginWindow inherits Window {
        width: 400px;
        height: 300px;
        title: "Login";

        property<string> username : "Username";
        property<string> password : "Password";

        VerticalLayout {
            spacing: 20px;
            padding: 20px;

            Text {
                text: "Login";
                font-size: 24px;
                horizontal-alignment: center;
            }

            VerticalLayout {
                spacing: 10px;

                TextInput {
                    text: username;
                }

                TextInput {
                    text: password;
                }

                Button {
                    text: "Login";
                }
            }
        }
    }
}

pub fn run_login_window() -> Result<(), UiError> {
    let main_window = LoginWindow::new().map_err(|e| UiError::SlintError(e.to_string()))?;

    main_window
        .run()
        .map_err(|e| UiError::SlintError(e.to_string()))?;

    Ok(())
}
