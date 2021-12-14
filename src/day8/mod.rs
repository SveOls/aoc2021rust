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

    let mut result = 0;
    
    while let Some(a) = lines.next() {
        let line: Vec<Vec<Vec<usize>>> = a.unwrap().split('|').map(|x| x.split_whitespace().map(|x| x.chars().map(to_number).collect()).collect()).collect();




        for i in line[1].iter() {
            match i.len() {
                2 | 3 | 4 | 7 => {
                    result += 1;
                }
                _ => {}
            }
        }
    }

    println!("day 8a result:  {}", result);
    Ok(())
}
pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut stuff: HashMap<Vec<usize>, usize> = HashMap::new();
    stuff.insert(vec![0, 1, 2,    4, 5, 6], 0);    
    stuff.insert(vec![      2,       5   ], 1);    
    stuff.insert(vec![0,    2, 3, 4,    6], 2);    
    stuff.insert(vec![0,    2, 3,    5, 6], 3);    
    stuff.insert(vec![   1, 2, 3,    5   ], 4);
    stuff.insert(vec![0, 1,    3,    5, 6], 5);    
    stuff.insert(vec![0, 1,    3, 4, 5, 6], 6);    
    stuff.insert(vec![0,    2,       5,  ], 7);
    stuff.insert(vec![0, 1, 2, 3, 4, 5, 6], 8);
    stuff.insert(vec![0, 1, 2, 3,    5, 6], 9);
    

    let mut result = 0;
    
    while let Some(a) = lines.next() {
        let mut possibilities = vec![vec![true; 7]; 7];
        let mut line: Vec<Vec<Vec<usize>>> = a.unwrap().split('|').map(|x| x.split_whitespace().map(|x| x.chars().map(to_number).collect()).collect()).collect();
        line[0].sort_by(|x, y| x.len().cmp(&y.len()));
        line[1].iter_mut().for_each(|x| x.sort());
        let mut data = Vec::new();
        let mut resu = Vec::new();
        std::mem::swap(&mut data, &mut line[0]);
        std::mem::swap(&mut resu, &mut line[1]);
        possibilities[2] = vec![false; 7];
        possibilities[2][data[0][0]] = true;
        possibilities[2][data[0][1]] = true;
        possibilities[5] = possibilities[2].clone();
        possibilities[0] = vec![false; 7];
        possibilities[0][data[1][0]] = true;
        possibilities[0][data[1][1]] = true;
        possibilities[0][data[1][2]] = true;
        for i in 0..7 {
            possibilities[0][i] = possibilities[0][i] && !possibilities[2][i]; 
        }
        possibilities[1] = vec![false; 7];
        possibilities[1][data[2][0]] = true;
        possibilities[1][data[2][1]] = true;
        possibilities[1][data[2][2]] = true;
        possibilities[1][data[2][3]] = true;
        for i in 0..7 {
            possibilities[1][i] = possibilities[1][i] && !possibilities[2][i]; 
        }
        possibilities[3] = possibilities[1].clone();
        let fives = vec![data[3].clone(), data[4].clone(), data[5].clone()];
        let mut mutual_fives = Vec::new();
        for i in 0..5 {
            if fives[1].contains(&fives[0][i]) && fives[2].contains(&fives[0][i]) && !possibilities[0][fives[0][i]] {
                mutual_fives.push(fives[0][i]);
            }
        }
        possibilities[3] = vec![false; 7];
        possibilities[3][mutual_fives[0]] = true;
        possibilities[3][mutual_fives[1]] = true;

        possibilities[6] = possibilities[3].clone();

        let sixes = vec![data[6].clone(), data[7].clone(), data[8].clone()];
        let mut mutual_sixes = Vec::new();
        for i in 0..6 {
            if sixes[1].contains(&sixes[0][i]) && sixes[2].contains(&sixes[0][i]) && !possibilities[0][sixes[0][i]] {
                if possibilities[2][sixes[0][i]] {
                    possibilities[2][sixes[0][i]] = false;
                    possibilities[5] = vec![false; 7];
                    possibilities[5][sixes[0][i]] = true;
                } else {
                    mutual_sixes.push(sixes[0][i])
                }
            }
        }

        for i in 0..2 {
            if mutual_fives.contains(&mutual_sixes[i]) {
                possibilities[6]    = vec![false; 7];
                possibilities[6][mutual_sixes[i]] = true;
                possibilities[5][mutual_sixes[i]] = false;
            } else {
                possibilities[1] = vec![false; 7];
                possibilities[1][mutual_sixes[i]] = true;
            }
        }
        for i in 0..2 {
            if !possibilities[6][mutual_fives[i]] {
                possibilities[3] = vec![false; 7];
                possibilities[3][mutual_fives[i]] = true;
            } 
        }

        
        possibilities[4] = vec![false; 7];
        for i in 0..7 {
            if !possibilities.iter().fold(false, |tot, x| tot ^ x[i]) {
                possibilities[4][i] = true;
            }
        }

        let mut a: Vec<(usize, usize)> = possibilities.into_iter().filter_map(|x| x.iter().position(|&x| x)).enumerate().collect();
        a.sort_by(|(_, x), (_, y)| x.cmp(y));
        let translate: Vec<usize> = a.iter().map(|x| x.0).collect();

        resu.iter_mut().for_each(|x| {
            x.iter_mut().for_each(|y| *y = translate[*y]);
            x.sort_by(|x, y| x.cmp(y))
        });



        result += resu.iter().map(|x| stuff.get(x).unwrap()).fold(0, |tot, x| 10*tot + x);
        // break;

    }

    println!("day 8b result:  {}", result);
    Ok(())
}


fn to_number(inp: char) -> usize {
    match inp {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!("{}", inp)
    }
}
