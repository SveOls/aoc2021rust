#![feature(iter_advance_by)]
#![feature(int_abs_diff)]

use std::time::Instant;
// let now = Instant::now();
// let after = now.elapsed();
// println!("{}s, {}ms, {}μs, {}ns", after.as_secs()%1000, after.as_millis()%1000, after.as_micros()%1000, after.as_nanos()%1000);


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
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    println!("[day]t = run example");
    println!("[day]long = run long test (if available)");
    println!("[day][no file recognized] = run main task");
    println!("Which day:");
    std::io::stdin().read_line(&mut line)?;
    line = line.split_whitespace().collect();
   let now = Instant::now();

    match line.chars().filter(|x| x.is_digit(10)).collect::<String>().parse()? {
        1 => day1:: run(&line)?,
        2 => day2:: run(&line)?,
        3 => day3:: run(&line)?,
        4 => day4:: run(&line)?,
        5 => day5:: run(&line)?,
        6 => day6:: run(&line)?,
        7 => day7:: run(&line)?,
        8 => day8:: run(&line)?,
        9 => day9:: run(&line)?,
        10=> day10::run(&line)?,
        11=> day11::run(&line)?,
        12=> day12::run(&line)?,
        13=> day13::run(&line)?,
        14=> day14::run(&line)?,
        15=> day15::run(&line)?,
        16=> day16::run(&line)?,
        17=> day17::run(&line)?,
        18=> day18::run(&line)?,
        19=> day19::run(&line)?,
        20=> day20::run(&line)?,
        21=> day21::run(&line)?,
        22=> day22::run(&line)?,
        23=> day23::run(&line)?,
        24=> day24::run(&line)?,
        25=> day25::run(&line)?,
        99=> {
            day1:: run(&"1" )?;
            day2:: run(&"2" )?;
            day3:: run(&"3" )?;
            day4:: run(&"4" )?;
            day5:: run(&"5" )?;
            day6:: run(&"6" )?;
            day7:: run(&"7" )?;
            day8:: run(&"8" )?;
            day9:: run(&"9" )?;
            day10::run(&"10")?;
            day11::run(&"11")?;
            day12::run(&"12")?;
            day13::run(&"13")?;
            day14::run(&"14")?;
            day15::run(&"15")?;
            day16::run(&"16")?;
            day17::run(&"17")?;
            day18::run(&"18")?;
            day19::run(&"19")?;
            day20::run(&"20")?;
            day21::run(&"21")?;
            day22::run(&"22")?;
            day23::run(&"23")?;
            day24::run(&"24")?;
            day25::run(&"25")?;
        }
        _ => println!("Not covered"),
    };

    let after = now.elapsed();
    println!("{}s, {}ms, {}μs, {}ns", after.as_secs()%1000, after.as_millis()%1000, after.as_micros()%1000, after.as_nanos()%1000);
    Ok(())
}
