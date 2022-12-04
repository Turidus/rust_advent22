extern crate core;

mod day_one;
mod day_two;
mod day_three;
mod day_four;

use std::{env, fs};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = String::from("Usage: rust_advent[.exe] day:int puzzle:int fileName:String. Example: rust_advent.exe 1 2 1_2");
    println!("{:?}", args);

    let day: u8 = args.get(1).expect(&*usage).parse().expect(&*usage);

    if day < 1 || day > 24 {panic!("{}", usage)}

    let puzzle: u8 = args.get(2).expect(&*usage).parse().expect(&*usage);

    if puzzle < 1  || puzzle > 2 {panic!("{}", usage)}

    let f_name = args.get(3).expect(&*usage);

    let base = env::current_dir().unwrap();
    let path= match base.parent(){
        None => {base.join("input_files").join(f_name)}
        Some(parent) => {parent.join("input_files").join(f_name)}
    };

    match day {
        1 => {
            if puzzle == 1 {
                println! ("The answer is: {:?}", day_one::puzzle_one::run(read_path(&*path)))
            }
            else {
                println! ("The answer is: {:?}", day_one::puzzle_two::run(read_path(&*path)))
            }
        }
        2 => {
            if puzzle == 1 {
                println! ("The answer is: {:?}", day_two::puzzle_one::run(read_path(&*path)))
            }
            else {
                println! ("The answer is: {:?}", day_two::puzzle_two::run(read_path(&*path)))
            }
        }
        3 => {
            if puzzle == 1 {
                println! ("The answer is: {:?}", day_three::puzzle_one::run(read_path(&*path)))
            }
            else {
                println! ("The answer is: {:?}", day_three::puzzle_two::run(read_path(&*path)))
            }
        }
        4 => {
            if puzzle == 1 {
                println! ("The answer is: {:?}", day_four::puzzle_one::run(read_path(&*path)))
            }
            else {
                println! ("The answer is: {:?}", day_four::puzzle_two::run(read_path(&*path)))
            }
        }
        _ => {panic!("{}", usage)}
    }
}

fn read_path(path: &Path) -> String {
   return fs::read_to_string(path)
        .expect("Reading the file was not possible.");
}
