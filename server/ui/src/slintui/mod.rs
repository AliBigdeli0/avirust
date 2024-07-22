use std::sync::Arc;

use entity::model::ui_config::UICommonMemory;
use forms::{login::LoginWindowShared, main_form::MainWindowShared};

use crate::uierrors::{Result, UiError};

pub mod forms;

pub type SlintCommonMemory = Arc<UICommonMemory>;

pub trait SlintUIWindowManager {
    fn run_win(&self) -> Result<()>;
    fn hide_win(&self) -> Result<()>;
    fn show_win(&self) -> Result<()>;
}

pub fn run_slive_gui() -> Result<()> {
    let uiconfig = config::read_ui_config()
        .map_err(|_| UiError::SlintError("error in load config".to_string()))?;

    let common_memory: SlintCommonMemory = Arc::new(UICommonMemory::new(uiconfig));

    let login_shared = Arc::new(LoginWindowShared::new(common_memory.clone())?);
    let main_shared = Arc::new(MainWindowShared::new(common_memory.clone())?);

    let login_memory_login = Arc::clone(&login_shared);
    let login_memory_main = Arc::clone(&main_shared);
    common_memory.register_observer(
        "login".into(),
        Box::new(move |msg: &str| match msg {
            "open_main" => {
                login_memory_login.hide_win().unwrap();
                login_memory_main.show_win().unwrap();
            }
            _ => (),
        }),
    );

    let main_memory_login = Arc::clone(&login_shared);
    let main_memory_main = Arc::clone(&main_shared);
    common_memory.register_observer(
        "main".into(),
        Box::new(move |msg: &str| match msg {
            "logout" => {
                main_memory_main.hide_win().unwrap();
                main_memory_login.show_win().unwrap();
            }
            _ => (),
        }),
    );

    Arc::clone(&login_shared).run_win()?;

    Ok(())
}
