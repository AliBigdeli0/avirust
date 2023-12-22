use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub const YES: char = 'Y';
pub const NO: char = 'N';

fn print_default(msg: &str, color: Color) {
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
    print_default(&format!("{}\n", msg), Color::Red)
}

pub fn warning(msg: &str) {
    print_default(msg, Color::Yellow)
}

pub fn warningln(msg: &str) {
    print_default(&format!("{}\n", msg), Color::Yellow)
}

pub fn success(msg: &str) {
    print_default(msg, Color::Green)
}

pub fn successln(msg: &str) {
    print_default(&format!("{}\n", msg), Color::Green)
}

pub fn draw_progressbar(current_value: usize, total: usize) {
    let progress = current_value * 100 / total;
    let bar_width: usize = 40;
    let progress_width = ((progress * bar_width / 100) as f64).round() as usize;

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    print!("┃");

    for i in 0..bar_width {
        if i <= progress_width {
            success("━");
        } else {
            print!("─");
        }
    }

    stdout.reset().expect("Failed to reset color");

    print!("┃ {:.1}%\r", progress + 1);

    io::stdout().flush().expect("Failed to flush stdout");
}

pub fn confirm_message(msg: &str) -> char {
    print!("{}", msg);
    warning("(Y/n) ");
    io::stdout().flush().unwrap();

    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("failed to read line");

    match input_string.trim() {
        "Y" => YES,
        _ => NO,
    }
}

pub fn select_from_list<'a, T, F>(list: &'a Vec<T>, print_operation: F) -> Option<&'a T>
where
    F: Fn(usize, &'a T) -> (),
{
    let mut input_string = String::new();

    for (index, item) in list.iter().enumerate() {
        print_operation(index, item);
    }

    warning("select a number from following name: ? ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input_string)
        .expect("failed to read line");

    let selected_index = input_string
        .trim()
        .parse::<usize>()
        .expect("can not convert entered number to integer");

    list.get(selected_index - 1)
}
