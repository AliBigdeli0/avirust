
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

fn print_default(msg: &str, color: Color)
{
    let mut binding = ColorSpec::new();
    let spec = binding.set_fg(Some(color));
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(&spec).expect("Failed to set color");

    write!(&mut stdout, "{}", msg).unwrap();

    stdout.reset().unwrap();
}

pub fn error(msg: &str) {
    print_default(msg, Color::Red)
}   

pub fn errorln(msg: &str) {
    print_default(&format!("{}\n",msg), Color::Red)
}

pub fn warning(msg: &str) {
    print_default(msg, Color::Yellow)
}

pub fn warningln(msg: &str) {
    print_default(&format!("{}\n",msg), Color::Yellow)
}

pub fn success(msg: &str) {
    print_default(msg, Color::Green)
}

pub fn successln(msg: &str) {
    print_default(&format!("{}\n",msg), Color::Green)
}