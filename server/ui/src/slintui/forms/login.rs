use crate::{
    slintui::{SlintCommonMemory, SlintUIWindowManager},
    uierrors::{Result, UiError},
};

slint::slint! {
    import { Button } from "std-widgets.slint";

    export component LoginWindow inherits Window {
        width: 250px;
        height: 180px;
        title: "Login";

        property<string> username : "Username";
        property<string> password : "Password";
        property <length> control_height: 20px;
        property <color> border_color : root.background.darker(20%);

        callback close_login;
        callback apply_login;

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
                        padding: 5px;
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
                        padding: 5px;
                    }
                }

                HorizontalLayout {
                    spacing: 5px;
                    Button {
                        height: control_height + 10px;
                        text: "Login";
                        clicked => {
                            root.apply_login();
                        }
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

pub fn new_login_window(memory: SlintCommonMemory) -> Result<LoginWindow> {
    let login_window = LoginWindow::new().map_err(|e| UiError::SlintError(e.to_string()))?;
    let weak_login_window = login_window.as_weak();

    login_window.on_close_login(move || {
        std::process::exit(0);
    });
    login_window.on_apply_login(move || {
        println!("apply on login has been clicked");

        if let Some(wl) = weak_login_window.upgrade() {
            let wwl = wl.hide().map_err(|e| UiError::SlintError(e.to_string()));

            memory.send_message("open_main");

            if let Err(_) = wwl {
                println!("error in hiding program");
            }
        }
    });

    Ok(login_window)
}

// --
pub struct LoginWindowShared {
    win: LoginWindow,
    _memory: SlintCommonMemory,
}

impl LoginWindowShared {
    pub fn new(memory: SlintCommonMemory) -> Result<LoginWindowShared> {
        let login_win = new_login_window(memory.clone())?;

        Ok(Self {
            win: login_win,
            _memory: memory,
        })
    }
}

impl SlintUIWindowManager for LoginWindowShared {
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
