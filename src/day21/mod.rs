use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;

use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}


pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut pos_1: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();
    let mut pos_2: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();

    let score = [6, 5, 4, 3, 2, 1, 0, 9, 8, 7];

    let mut score_1 = 0;
    let mut score_2 = 0;
    for i in 0.. {
        pos_1 = (pos_1 + score[(2*i)%10]-1)%10+1;
        pos_2 = (pos_2 + score[((2*i)+1)%10]-1)%10+1;
        score_1 += pos_1;
        if score_1 >= 1000 {
            println!("day 21a result: {}", score_2 * (6*(i+1)-3));
            break;
        }
        score_2 += pos_2;
        if score_2 >= 1000 {
            println!("day 21a result: {}", score_1 * 6*(i+1));
            break;
        }
    }
    Ok(())
}



pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let pos_1: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();
    let pos_2: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();
    
    let mut positions: HashMap<((usize, usize), (usize, usize), bool), usize> = HashMap::new();
    positions.insert(((pos_1, 0), (pos_2, 0), true),1);

    let mut universe_1 = 0;
    let mut universe_2 = 0;
    let mut changed = true;
    while changed {
        changed = false;
        let mut to_iterate_over = positions;
        positions = HashMap::new();
        for ((first, second, who) , val) in to_iterate_over.drain() {
            for i in 1..4 {
                for j in 1..4 {
                    for k in 1..4 {
                        if who {
                            let temp = (first.0+i+j+k-1)%10+1;
                            if first.1 + temp >= 21 {
                                universe_1 += val;
                            } else {
                                *positions.entry(((temp, first.1 + temp), second, !who)).or_insert(0) += val;
                            }
                        } else {
                            let temp = (second.0+i+j+k-1)%10+1;
                            if second.1 + temp >= 21 {
                                universe_2 += val;
                            } else {
                                *positions.entry((first, (temp, second.1 + temp), !who)).or_insert(0) += val;
                            }
                        }
                    }
                }
            }
            changed = true;
        }
    }
    println!("day 21b result: {}", universe_1.max(universe_2));
    
    Ok(())
}

