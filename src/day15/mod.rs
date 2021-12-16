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

    let mut data: Vec<Vec<(i64, Option<i64>)>> = lines.map(|x| x.unwrap().chars().map(|x| (x.to_digit(10).unwrap() as i64, None)).collect::<Vec<(i64, Option<i64>)>>()).collect();
    data[0][0].1 = Some(0);


    let mut candidates: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    candidates.insert((0, 1), (0, 0));
    candidates.insert((1, 0), (0, 0));

    let mut visited = vec![vec![false; data[0].len()]; data.len()];
    visited[0][0] = true;

    while !visited[data.len()-1][data[0].len()-1] {

        let mut remove = None;
        let mut min = std::i64::MAX;
        for (&(y, x), &(from_y, from_x)) in candidates.iter() {
            if data[y][x].0 + data[from_y][from_x].1.unwrap() < min {
                min = data[y][x].0 + data[from_y][from_x].1.unwrap();
                remove = Some((y, x));
            }
        }
        let a = candidates.remove_entry(&remove.unwrap()).unwrap();
        data[a.0.0][a.0.1].1 = Some(data[a.0.0][a.0.1].0 + data[a.1.0][a.1.1].1.unwrap());
        visited[a.0.0][a.0.1] = true;
        if let Some(c) = a.0.0.checked_sub(1) {
            if !visited[c][a.0.1] && !candidates.contains_key(&(c, a.0.1)) {
                candidates.insert((c, a.0.1), (a.0.0, a.0.1));
            }
        }
        if let Some(c) = a.0.1.checked_sub(1) {
            if !visited[a.0.0][c] && !candidates.contains_key(&(a.0.0, c)) {
                candidates.insert((a.0.0, c), (a.0.0, a.0.1));
            }
        }
        if let Some(c) = visited.get(a.0.0 + 1) {
            if !c[a.0.1] && !candidates.contains_key(&(a.0.0 + 1, a.0.1)) {
                candidates.insert((a.0.0 + 1, a.0.1), (a.0.0, a.0.1));
            }

        }
        if let Some(&c) = visited[a.0.0].get(a.0.1 + 1) {
            if !c && !candidates.contains_key(&(a.0.0, a.0.1 + 1)) {
                candidates.insert((a.0.0, a.0.1 + 1), (a.0.0, a.0.1));
            }
        }
    }

    println!("day15a result: {}", data[data.len()-1][data[0].len()-1].1.unwrap());
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



    let mut candidates: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    candidates.insert((0, 1), (0, 0));
    candidates.insert((1, 0), (0, 0));

    let mut visited = vec![vec![false; data[0].len()]; data.len()];
    visited[0][0] = true;

    while !visited[data.len()-1][data[0].len()-1] {
    // for _ in 0..4 {
        // for i in candidates.iter() {
        //     print!("{:?}", i.0);
        // }
        // println!("");
        let mut remove = None;
        let mut min = std::i64::MAX;
        for (&(y, x), &(from_y, from_x)) in candidates.iter() {
            // println!("Checking {:?} from {:?}", (y, x), (from_y, from_x));
            if data[y][x].0 + data[from_y][from_x].1.unwrap() < min {
                // println!("Replacing. Cost: {}",data[y][x].0 + data[from_y][from_x].1.unwrap());
                min = data[y][x].0 + data[from_y][from_x].1.unwrap();
                remove = Some((y, x));
            }
        }
        // println!();
        let a = candidates.remove_entry(&remove.unwrap()).unwrap();
        data[a.0.0][a.0.1].1 = Some(data[a.0.0][a.0.1].0 + data[a.1.0][a.1.1].1.unwrap());
        visited[a.0.0][a.0.1] = true;
        if let Some(c) = a.0.0.checked_sub(1) {
            if !visited[c][a.0.1] && !candidates.contains_key(&(c, a.0.1)) {
                candidates.insert((c, a.0.1), (a.0.0, a.0.1));
            }
        }
        if let Some(c) = a.0.1.checked_sub(1) {
            if !visited[a.0.0][c] && !candidates.contains_key(&(a.0.0, c)) {
                candidates.insert((a.0.0, c), (a.0.0, a.0.1));
            }
        }
        if let Some(c) = visited.get(a.0.0 + 1) {
            if !c[a.0.1] && !candidates.contains_key(&(a.0.0 + 1, a.0.1)) {
                candidates.insert((a.0.0 + 1, a.0.1), (a.0.0, a.0.1));
            }

        }
        if let Some(&c) = visited[a.0.0].get(a.0.1 + 1) {
            if !c && !candidates.contains_key(&(a.0.0, a.0.1 + 1)) {
                candidates.insert((a.0.0, a.0.1 + 1), (a.0.0, a.0.1));
            }
            // else {
            //     let &(y, x) = candidates.get(&(a.0.0, a.0.1 + 1)).unwrap();
            //     if data[y][x] > data[a.0.0][a.0.1] {
            //         candidates.insert((a.0.0, a.0.1 + 1), (a.0.0, a.0.1));
            //     }
            // }
        }
    }

    println!("day15b result: {}", data[data.len()-1][data[0].len()-1].1.unwrap());

    for (y, i) in data.iter().enumerate() {
        if y % pre_data.len() == 0 {
            println!();
        }
        for (x, j) in i.iter().enumerate() {
            if x % pre_data[0].len() == 0 {
                print!("  ");
            }
            print!("{} ", j.0);
        }
        println!()
    }
    println!();
    for i in data.iter() {
        for j in i.iter() {
            match j.1 {
                Some(a) => {
                    if a / 100 > 0 {
                        print!("{} ", a)
                    } else if a / 10 > 0 {
                        print!("{}  ", a)
                    } else {
                        print!("{}   ", a)
                    }
                }
                None => print!("___ "),
            }
        }
        println!()
    }


    Ok(())
}
