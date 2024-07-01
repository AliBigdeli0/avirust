use std::rc::Rc;

use entity::model::ui_config::UICommonMemory;
use forms::{login::LoginWindowShared, main_form::MainWindowShared};

use crate::uierrors::{Result, UiError};

pub mod forms;

pub type SlintCommonMemory = Rc<UICommonMemory<LoginWindowShared, MainWindowShared>>;

pub fn run_slive_gui() -> Result<()> {
    let login_shared = LoginWindowShared::new()?;
    let main_window = MainWindowShared::new()?;

    let uiconfig = config::read_ui_config()
        .map_err(|_| UiError::SlintError("error in load config".to_string()))?;

    let common_memory: SlintCommonMemory =
        Rc::new(UICommonMemory::new(uiconfig, login_shared, main_window));

    common_memory.run_login();
    common_memory.run_main();

    Ok(())
}
