mod app;
mod forms;
mod utils;

pub fn get_version() -> String {
    "1.0.0".to_string()
}

// ----------------------------
pub use app::AppTemplate;
