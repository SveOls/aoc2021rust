use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
 
    println!("day 6a result:  {}", rounds(lines, 80)?);

    Ok(())
}
pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
 
    println!("day 6b result:  {}", rounds(lines, 256)?);

    Ok(())
}

fn rounds(lines: Lines<BufReader<File>>, rounds: usize) -> Result<u128, Box<dyn std::error::Error>> {

    // let mut new;

    let mut old = vec![0u128; 9];
    lines.for_each(|x| x.unwrap().split(',').filter_map(|x| x.parse::<usize>().ok()).for_each(|x| old[x] += 1));

    for _ in 0..rounds {
        old.rotate_left(1);
        old[6] += old[8];
    }
    // (0..rounds).for_each(|_| { old.rotate_left(1); old[6] += old[8] }); // one line, but slower

    Ok(old.iter().sum())
}