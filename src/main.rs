#![allow(warnings)]
use crate::days::*;
use std::{env, fs};
use std::fmt::Display;
use std::path::Path;
use std::time::Instant;
use clap::Parser;


pub mod days;
mod tools;
mod domain;

static ANSI_ITALIC: &str = "\x1b[3m";
static ANSI_BOLD: &str = "\x1b[1m";
static ANSI_RESET: &str = "\x1b[0m";

#[derive(Parser)]
struct RunArgument {
    day: Option<i32>
}

macro_rules! print_style_result {
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


fn main() {
    let parse_result = RunArgument::parse();

    match parse_result.day {
        Some(day) => print_specific_day(day),
        None => print_all_days()
    }
    env::set_var("RUST_BACKTRACE", "1");
}

fn print_all_days(){
    for i in 1..25 {
        print_specific_day(i)
    }
}
fn print_specific_day(day: i32) {
    match day {
        1 => print_style_result!(day_01, load_file("day01_input.txt"), "Day 1"),
        2 => print_style_result!(day_02, load_file("day02_input.txt"), "Day 2"),
        3 => print_style_result!(day_03, load_file("day03_input.txt"), "Day 3"),
        4 => print_style_result!(day_04, load_file("day04_input.txt"), "Day 4"),
        5 => print_style_result!(day_05, load_file("day05_input.txt"), "Day 5"),
        6 => print_style_result!(day_06, load_file("day06_input.txt"), "Day 6"),
        7 => print_style_result!(day_07, load_file("day07_input.txt"), "Day 7"),
        8 => print_style_result!(day_08, load_file("day08_input.txt"), "Day 8"),
        9 => print_style_result!(day_09, load_file("day09_input.txt"), "Day 9"),
        10 => print_style_result!(day_10, load_file("day10_input.txt"), "Day 10"),
        11 => print_style_result!(day_11, load_file("day11_input.txt"), "Day 11"),
        12 => print_style_result!(day_12, load_file("day12_input.txt"), "Day 12"),
        13 => print_style_result!(day_13, load_file("day13_input.txt"), "Day 13"),
        14 => print_style_result!(day_14, load_file("day14_input.txt"), "Day 14"),
        _ => {}
    }
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
    let file_path = Path::new("puzzle_inputs").join(path);

    if !file_path.exists() {
        panic!("failure");
    }

    return fs::read_to_string(file_path).unwrap();
}