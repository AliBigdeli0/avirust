use crate::uierrors::UiError;

slint::slint! {
    import { Button } from "std-widgets.slint";

    export  component LoginWindow inherits Window {
        width: 250px;
        height: 180px;
        title: "Login";

        property<string> username : "Username";
        property<string> password : "Password";
        property <length> control_height: 20px;
        property <color> border_color : root.background.darker(20%);
        callback close_login();

        VerticalLayout {
            spacing: 20px;
            padding: 20px;
            height: root.height;

            Text {
                text: "Login";
                font-size: 24px;
                horizontal-alignment: center;
            }

            VerticalLayout {
                spacing: 10px;

                Rectangle {
                    border-color: border_color;
                    border-width: 2px;
                    height: control_height;
                    TextInput {
                        text: "username";
                    }
                }

                Rectangle {
                    border-color: border_color;
                    border-width: 2px;
                    height: control_height;
                    TextInput {
                        text: "password";
                        input-type: InputType.password;
                        font-size: 10px;
                    }
                }

                HorizontalLayout {
                    spacing: 5px;
                    Button {
                        height: control_height + 10px;
                        text: "Login";
                    }

                    Button {
                        height: control_height + 10px;
                        text: "Cancel";
                        clicked => {
                            root.close_login();
                        }
                    }
                }
            }
        }
    }
}

pub fn run_login_window() -> Result<(), UiError> {
    let login_window = LoginWindow::new().map_err(|e| UiError::SlintError(e.to_string()))?;
    login_window
        .run()
        .map_err(|e| UiError::SlintError(e.to_string()))?;

    login_window.on_close_login(move || todo!("implement to loging window"));
    Ok(())
}
