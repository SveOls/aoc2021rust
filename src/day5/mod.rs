use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut points = HashMap::new();
    lines.map(|x| x.unwrap()
            .split(|x| x == ' ' || x == ',')
            .filter_map(|x| x.parse().ok())
            .collect())
        .filter(|x: &Vec<usize>| x[0] == x[2] || x[1] == x[3])
        .for_each(|x| add_points(&x, &mut points)); 
    println!("day 5a result:  {}", points.iter().filter(|(_, &val)| val > 1).count());
    Ok(())
}

pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut points = HashMap::new();
    lines.map(|x| x.unwrap()
            .split(|x| x == ' ' || x == ',')
            .filter_map(|x| x.parse().ok())
            .collect())
        .for_each(|x| add_points_b(&x, &mut points)); 
    println!("day 5b result:  {}", points.iter().filter(|(_, &val)| val > 1).count());
    Ok(())
}


fn add_points(inp: &Vec<usize>, set: &mut std::collections::HashMap<(usize, usize), usize>) {
    if inp[0] == inp[2] {
        for i in inp[1].min(inp[3])..=inp[1].max(inp[3]) {
            *set.entry((inp[0], i)).or_insert(0) += 1;
        }
    } else if inp[1] == inp[3] {
        for i in inp[0].min(inp[2])..=inp[0].max(inp[2]) {
            *set.entry((i, inp[1])).or_insert(0) += 1;
        }
    } else {
        panic!()
    }
}

fn add_points_b(inp: &Vec<usize>, set: &mut std::collections::HashMap<(usize, usize), usize>) {
    if inp[0] == inp[2] {
        for i in inp[1].min(inp[3])..=inp[1].max(inp[3]) {
            *set.entry((inp[0], i)).or_insert(0) += 1;
        }
    } else if inp[1] == inp[3] {
        for i in inp[0].min(inp[2])..=inp[0].max(inp[2]) {
            *set.entry((i, inp[1])).or_insert(0) += 1;
        }
    } else if inp[0] > inp[2] && inp[1] > inp[3] {
        for (i, j) in (inp[2]..=inp[0]).zip(inp[3]..=inp[1]) {
            *set.entry((i, j)).or_insert(0) += 1;
        }
    } else if inp[0] < inp[2] && inp[1] < inp[3] {
        for (i, j) in (inp[0]..=inp[2]).zip(inp[1]..=inp[3]) {
            *set.entry((i, j)).or_insert(0) += 1;
        }
    } else if inp[0] < inp[2] && inp[1] > inp[3] {
        for (i, j) in ((inp[0]..=inp[2]).rev()).zip(inp[3]..=inp[1]) {
            *set.entry((i, j)).or_insert(0) += 1;
        }
    } else if inp[0] > inp[2] && inp[1] < inp[3] {
        for (i, j) in (inp[2]..=inp[0]).zip((inp[1]..=inp[3]).rev()) {
            *set.entry((i, j)).or_insert(0) += 1;
        }
    } else {
        panic!()
    }
}