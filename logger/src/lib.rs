use env_logger::{Builder, Target};
use log::{LevelFilter, info, error};
use std::fs::{OpenOptions};

pub fn get_version() -> String {
    String::from("1.0.0")
}

pub fn init() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(common::file_helper::log_file_path()).unwrap();

    Builder::new()
        .target(Target::Pipe(Box::new(file)))
        .write_style(env_logger::WriteStyle::Always)
        .filter_level(LevelFilter::Debug)
        .format_target(false)
        .init();
}

pub fn info(msg: &'static str) {
    info!("{}",msg);
}

pub fn error(msg: &'static str) {
    error!("{}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let ver = get_version();
        assert_eq!(&ver, "1.0.0");
    }
}
