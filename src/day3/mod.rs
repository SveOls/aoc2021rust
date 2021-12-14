use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data: Vec<usize> = Vec::new();
    let mut length = 0;


    let mut alloc = false;
    while let Some(a) = lines.next() {
        let line = a.unwrap();
        if !alloc {
            data = vec![0; line.len()];
            alloc = true;
        }
        length += 1;
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '1' => data[i] += 1,
                '0' => {}
                _ => panic!()
            }
        }
    }

    for i in data.iter_mut() {
        if *i > length/2 {
            *i = 1
        } else {
            *i = 0
        }
    }

    let gamma = isize::from_str_radix(&data.iter().map(|x| x.to_string()).collect::<String>(), 2)?;

    for i in data.iter_mut() {
        if *i == 1 {
            *i = 0 
        } else {
            *i = 1
        }
    }
    let epsilon = isize::from_str_radix(&data.iter().map(|x| x.to_string()).collect::<String>(), 2)?;

    println!("day 3a result:  {}", gamma * epsilon);

    Ok(())

}

pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {


    let data: Vec<Vec<char>> = lines.filter_map(|x| x.ok()).map(|x| x.chars().collect()).collect();

    let datalen = data[0].len();
    let mut common1;
    let mut common2;


    let mut part1 = data.clone();
    let mut part2 = data.clone();
    for i in 0..datalen {
        common1 = most_common(&part1, i);
        common2 = most_common(&part2, i);
        if part1.len() > 1 {
            part1.retain(|x: &Vec<char>| x[i] == '1' && common1 || x[i] == '0' && !common1);
        }
        if part2.len() > 1 {
            part2.retain(|x: &Vec<char>| x[i] == '1' && !common2 || x[i] == '0' && common2);
        }
        if part2.len() == 1 && part1.len() == 1 {
            break;
        }
    }

    let ox_gen    = usize::from_str_radix(&part1[0].iter().collect::<String>(), 2)?;
    let co2_scrub = usize::from_str_radix(&part2[0].iter().collect::<String>(), 2)?;

    println!("day 3b result:  {}", ox_gen * co2_scrub);

    Ok(())

}

fn most_common(data: &Vec<Vec<char>>, pos: usize) -> bool {
    let mut z = 0;
    let mut o = 0;
    for i in data.iter() {
        match i[pos] {
            '0' => z += 1,
            '1' => o += 1,
            _ => panic!()
        }
    }
    !(z > o)
}