use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut temp: i64;
    let mut result: i64 = 0;
    let mut prev = -1;


    while let Some(line) = lines.next() {
        if let Ok(a) = line {
            temp = a.parse()?;
            if prev == -1 {
                prev = temp;
                continue
            } else if temp > prev {
                result += 1;
            }
            prev = temp;
        } else {
            panic!()
        }
    }
    println!("day 1a result:  {}", result);
    Ok(())

}

pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {


    let mut result = 0;
    let mut prev = 0;
    let mut temp; 

    let data: Vec<i64> = lines.filter_map(|x| x.ok()).filter_map(|x| x.parse().ok()).collect();

    for i in 0..3 {
        prev += data[i];
    }

    for i in 3..data.len() {
        temp = 0;
        for j in 0..3 {
            temp += data[i-j]; 
        }
        if temp > prev {
            result += 1;
        }
        prev = temp;
    }


    println!("day 1b result:  {}", result);
    Ok(())


}