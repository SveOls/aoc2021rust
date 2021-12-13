#![feature(iter_advance_by)]
#![feature(int_abs_diff)]

use std::time::Instant;


mod file_handling;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;


fn main() {
    let mut line = String::new();
    println!("Which day:");
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.split_whitespace().collect();
    // println!(">_{}_<", line);
   let now = Instant::now();

    match line.parse().unwrap() {
        1 => day1 ::run(&line).unwrap(), // done
        2 => day2:: run(&line).unwrap(),
        3 => day3:: run(&line).unwrap(),
        4 => day4:: run(&line).unwrap(),
        5 => day5:: run(&line).unwrap(),
        6 => day6:: run(&line).unwrap(),
        7 => day7:: run(&line).unwrap(),
        8 => day8:: run(&line).unwrap(),
        9 => day9:: run(&line).unwrap(),
        10=> day10::run(&line).unwrap(),
        11=> day11::run(&line).unwrap(),
        12=> day12::run(&line).unwrap(),
        13=> day13::run(&line).unwrap(),
        // 14=> day14::run(&line).unwrap(),
        // 15=> day15::run(&line).unwrap(),
        // 16=> day16::run(&line).unwrap(),
        // 17=> day17::run(&line).unwrap(),
        // 18=> day18::run(&line).unwrap(),
        // 19=> day19::run(&line).unwrap(),
        // 20=> day20::run(&line).unwrap(),
        // 21=> day21::run(&line).unwrap(),
        // 22=> day22::run(&line).unwrap(),
        // 23=> day23::run(&line).unwrap(),
        // 24=> day24::run(&line).unwrap(),
        // 25=> day25::run(&line).unwrap(),
        _ => println!("Not covered"),
    };

    let after = now.elapsed();
    println!("{}s, {}ms, {}μs, {}ns", after.as_secs()%1000, after.as_millis()%1000, after.as_micros()%1000, after.as_nanos()%1000);

}
