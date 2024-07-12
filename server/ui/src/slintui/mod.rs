use std::{rc::Rc, sync::Arc};

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

    let login_shared = Rc::new(LoginWindowShared::new(common_memory.clone())?);
    let main_window = MainWindowShared::new(common_memory.clone())?;
    let login_shared_clone1 = Rc::clone(&login_shared);
    let login_shared_clone2 = Rc::clone(&login_shared);

    common_memory.register_observer(
        "login".into(),
        Box::new(move |msg: &str| {
            println!("common memory message: '{msg}'");

            match msg {
                "open_main" => {
                    login_shared_clone1.hide_win().unwrap();
                    main_window.show_win().unwrap();
                }
                _ => (),
            }
        }),
    );
    common_memory.register_observer("main".into(), Box::new(move |_msg: &str| {}));

    login_shared_clone2.run_win()?;

    Ok(())
}
