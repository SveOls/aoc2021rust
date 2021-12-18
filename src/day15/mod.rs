use std::fs::File;
use std::io::{BufReader, Lines};
use priority_queue::PriorityQueue;


use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}


pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data: Vec<Vec<(i64, Option<i64>)>> = lines.map(|x| x.unwrap().chars().map(|x| (x.to_digit(10).unwrap() as i64, None)).collect::<Vec<(i64, Option<i64>)>>()).collect();
    data[0][0].1 = Some(0);


    let mut candidates: PriorityQueue<(usize, usize), i64> = PriorityQueue::new();
    candidates.push((0, 1), -data[0][1].0);
    candidates.push((1, 0), -data[1][0].0);

    let mut visited = vec![vec![false; data[0].len()]; data.len()];
    visited[0][0] = true;

    while !visited[data.len()-1][data[0].len()-1] {
        let (a, priority) = candidates.pop().unwrap();
        data[a.0][a.1].1 = Some(priority);
        visited[a.0][a.1] = true;
        if let Some(c) = a.0.checked_sub(1) {
            if !visited[c][a.1] {
                candidates.push_increase((c, a.1), -(-data[a.0][a.1].1.unwrap() + data[c][a.1].0));
            }
        }
        if let Some(c) = a.1.checked_sub(1) {
            if !visited[a.0][c] {
                candidates.push_increase((a.0, c), -(-data[a.0][a.1].1.unwrap() + data[a.0][c].0));
            }
        }
        if let Some(c) = visited.get(a.0 + 1) {
            if !c[a.1] {
                candidates.push_increase((a.0 + 1, a.1), -(-data[a.0][a.1].1.unwrap() + data[a.0 + 1][a.1].0));
            }

        }
        if let Some(&c) = visited[a.0].get(a.1 + 1) {
            if !c {
                candidates.push_increase((a.0, a.1 + 1), -(-data[a.0][a.1].1.unwrap() + data[a.0][a.1 + 1].0));
            }
        }
    }

    println!("day 15a result: {}", -data[data.len()-1][data[0].len()-1].1.unwrap());
    Ok(())
}





pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let pre_data: Vec<Vec<(i64, Option<i64>)>> = lines.map(|x| x.unwrap().chars().map(|x| (x.to_digit(10).unwrap() as i64, None)).collect::<Vec<(i64, Option<i64>)>>()).collect();
    
    let mut data = vec![vec![(0, None); pre_data.len() * 5]; pre_data.len() * 5];
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            data[y][x].0 = 
                (pre_data[y%(pre_data.len())][x%(pre_data[0].len())].0 
                + ((y/(pre_data.len()) + x/(pre_data[0].len())) as i64)-1)%9+1;
        }
    }


    data[0][0].1 = Some(0);

    let mut candidates: PriorityQueue<(usize, usize), i64> = PriorityQueue::new();
    candidates.push((0, 1), -data[0][1].0);
    candidates.push((1, 0), -data[1][0].0);

    let mut visited = vec![vec![false; data[0].len()]; data.len()];
    visited[0][0] = true;

    while !visited[data.len()-1][data[0].len()-1] {


        let (a, priority) = candidates.pop().unwrap();

        data[a.0][a.1].1 = Some(priority);
        visited[a.0][a.1] = true;
        if let Some(c) = a.0.checked_sub(1) {
            if !visited[c][a.1] {
                candidates.push_increase((c, a.1), -(-data[a.0][a.1].1.unwrap() + data[c][a.1].0));
            }
        }
        if let Some(c) = a.1.checked_sub(1) {
            if !visited[a.0][c] {
                candidates.push_increase((a.0, c), -(-data[a.0][a.1].1.unwrap() + data[a.0][c].0));
            }
        }
        if let Some(c) = visited.get(a.0 + 1) {
            if !c[a.1] {
                candidates.push_increase((a.0 + 1, a.1), -(-data[a.0][a.1].1.unwrap() + data[a.0 + 1][a.1].0));
            }

        }
        if let Some(&c) = visited[a.0].get(a.1 + 1) {
            if !c {
                candidates.push_increase((a.0, a.1 + 1), -(-data[a.0][a.1].1.unwrap() + data[a.0][a.1 + 1].0));
            }
        }
    }

    println!("day 15b result: {}", -data[data.len()-1][data[0].len()-1].1.unwrap());
    Ok(())
}