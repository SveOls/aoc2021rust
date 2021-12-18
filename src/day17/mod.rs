use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashSet;

use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let data: Vec<i64> = lines.flat_map(|x| x.unwrap().split(|x| x == ',' || x == '=' || x == '.').filter_map(|x| x.parse().ok()).collect::<Vec<i64>>()).collect();
   
    let x_range = (data[0], data[1]);
    let y_range = (data[2], data[3]);
    
    let y_pos    = |y: i64, after: usize| { (0..after).fold(0, |tot, i| tot + y - i as i64) };


    let min_x = {
        let mut sum = 0;
        let mut ret = 0;
        for i in 0.. {
            sum += i;
            if sum > x_range.0 {
                ret = i;
                break
            }
        }
        ret
    };

    let mut result: HashSet<i64> = HashSet::new();

    for x in min_x..=x_range.1 {
        for y in y_range.0.min(0)..=y_range.1.max(-y_range.0) {
            let mut curr_y = 0;
            let mut curr_x = 0; 
            for after in 0.. {
                
                if curr_x > x_range.1 || curr_y < y_range.0 {
                    break;
                } else if (x_range.1 - curr_x)*(curr_x - x_range.0) >= 0 && (y_range.1 - curr_y)*(curr_y - y_range.0) >= 0 {
                    result.insert(y);
                    break;
                }  
                curr_x += (x - after).max(0);
                curr_y += y - after;
            }
        }
    }

    println!("day 17a result: {}", y_pos(*result.iter().max().unwrap(), *result.iter().max().unwrap() as usize));


    Ok(())
}


pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let data: Vec<i64> = lines.flat_map(|x| x.unwrap().split(|x| x == ',' || x == '=' || x == '.').filter_map(|x| x.parse().ok()).collect::<Vec<i64>>()).collect();
   
    let x_range = (data[0], data[1]);
    let y_range = (data[2], data[3]);

    let min_x = {
        let mut sum = 0;
        let mut ret = 0;
        for i in 0.. {
            sum += i;
            if sum > x_range.0 {
                ret = i;
                break
            }
        }
        ret
    };

    let mut result: HashSet<(i64, i64)> = HashSet::new();

    for x in min_x..=x_range.1 {
        for y in y_range.0.min(0)..=y_range.1.max(-y_range.0) {
            let mut curr_y = 0;
            let mut curr_x = 0; 
            for after in 0.. {
                
                if curr_x > x_range.1 || curr_y < y_range.0 {
                    break;
                } else if (x_range.1 - curr_x)*(curr_x - x_range.0) >= 0 && (y_range.1 - curr_y)*(curr_y - y_range.0) >= 0 {
                    result.insert((x, y));
                    break;
                }  
                curr_x += (x - after).max(0);
                curr_y += y - after;
            }
        }
    }

    println!("day 17b result: {}", result.len());

    Ok(())
}

