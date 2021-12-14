use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
 
    let data: Vec<usize> = lines.flat_map(|x| x.unwrap().split(',').filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>()).collect();
    let &max = data.iter().max().unwrap();
    let &min = data.iter().min().unwrap();
    let mut cost = None;
    let mut tc;
    for i in min..max {
        tc = 0;
        data.iter().for_each(|x| tc += x.abs_diff(i));
        match cost {
            Some((_, value)) => {
                if tc < value {
                    cost = Some((i, tc));
                }
            }
            None => {
                cost = Some((i, tc))
            }
        }
    }   
    println!("day 7a result:  {}", cost.unwrap().1);
    Ok(())
}
pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let data: Vec<usize> = lines.flat_map(|x| x.unwrap().split(',').filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>()).collect();
    let mut cost = None;
    let mut tc;
    for i in *data.iter().min().unwrap()..*data.iter().max().unwrap() {
        tc = 0;
        data.iter().for_each(|x| tc += (x.abs_diff(i).pow(2) + x.abs_diff(i))/2);
        if let Some((_, value)) = cost {
            if tc < value {
                cost = Some((i, tc));
            }
        } else {
            cost = Some((i, tc))
        }
    }   
    println!("day 7b result:  {}", cost.unwrap().1);
    Ok(())
}

