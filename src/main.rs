use std::{env, fs};
use std::fmt::Display;
use std::path::Path;
use std::time::Instant;
use clap::Parser;


static ANSI_ITALIC: &str = "\x1b[3m";
static ANSI_BOLD: &str = "\x1b[1m";
static ANSI_RESET: &str = "\x1b[0m";

#[derive(Parser)]
struct RunArgument {
    day: Option<i32>
}

fn main() {
    let parse_result = RunArgument::parse();

    match parse_result.day {
        Some(day) => print_specific_day(day),
        None => print_all_days()
    }
    env::set_var("RUST_BACKTRACE", "1");
}

fn print_all_days(){

}
fn print_specific_day(day: i32) {
    match day {
        _ => panic!("Day hasnt happened yet")
    }
}

macro_rules! print_result {
    ($day:path, $input:expr, $day_name:expr) => {{
        use $day::*;
        println!("----");
        println!("🎄 {}{}{} 🎄", ANSI_BOLD, $day_name, ANSI_RESET);
        println!("🎄 {}Part 1{} 🎄", ANSI_BOLD, ANSI_RESET);
        print_result(part_one, $input);
        println!("🎄 {}Part 2{} 🎄", ANSI_BOLD, ANSI_RESET);
        print_result(part_two, $input);
        println!("----");
    }};
}


fn print_result<T: Display>(func: impl FnOnce(String) -> T, input: String) {
    let timer = Instant::now();
    let result = func(input);
    let time = timer.elapsed();
    println!(
        "{} {}(elapsed: {:.2?}){}",
        result, ANSI_ITALIC, time, ANSI_RESET
    );
}

fn load_file(path: &str) -> String {
    let file_path = Path::new(path);

    if !file_path.exists() {
        panic!("failure");
    }

    return fs::read_to_string(file_path).unwrap();
}