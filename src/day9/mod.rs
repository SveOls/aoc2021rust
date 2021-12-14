use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::{HashMap, HashSet};

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let data: Vec<Vec<u8>> = lines.map(|x| x.unwrap().chars().map(|x| x.to_digit(10).unwrap() as u8).collect()).collect();

    let mut larger;
    let mut sum = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            larger = true;
            if let Some(i_above) = i.checked_sub(1) {
                if data[i][j] >= data[i_above][j] {
                    larger = false;
                } 
            }
            if let Some(j_behind) = j.checked_sub(1) {
                if data[i][j] >= data[i][j_behind] {
                    larger = false;
                } 
            }
            if i + 1 != data.len() {
                if data[i][j] >= data[i+1][j] {
                    larger = false;
                }
            }
            if j + 1 != data[i].len() {
                if data[i][j] >= data[i][j+1] {
                    larger = false;
                }
            }
            if larger {
                sum += 1 + data[i][j] as usize;
            }
        }
    }
    println!("day 9a result:  {}", sum);
    Ok(())
}

pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data: Vec<Vec<(u8, Option<usize>)>> = lines.map(|x| (x.unwrap().chars().map(|x| (x.to_digit(10).unwrap() as u8, None))).collect()).collect();

    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();

    let mut count = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j].0 == 9 {
                data[i][j].1 = Some(0);
            } else {
                if let Some(i_above) = i.checked_sub(1) {
                    if data[i_above][j].0 != 9 {
                        data[i][j].1 = data[i_above][j].1;
                        if data[i][j].1.is_none() {
                            panic!()
                        }
                    }
                }
                if let Some(j_behind) = j.checked_sub(1) {
                    if data[i][j_behind].0 != 9 {
                        if let Some(a) = data[i][j].1 {
                            let b = data[i][j_behind].1.unwrap();
                            (*map.entry(a).or_insert(HashSet::new())).insert(b);
                            (*map.entry(b).or_insert(HashSet::new())).insert(a);
                        } else {
                            data[i][j].1 = data[i][j_behind].1;
                        }
                        if data[i][j].1.is_none() {
                            panic!()
                        }
                    }
                }
                if data[i][j].1.is_none() {
                    count += 1;
                    data[i][j].1 = Some(count);
                }
            }
        }
    }
    if !map.contains_key(&1) {
        map.insert(1, HashSet::new());
        (*map.entry(1).or_insert(HashSet::new())).insert(1);
    }


    let mut counts = vec![0; count+1];
    data.iter().for_each(|x| x.iter().for_each(|(_, x)| counts[x.unwrap()] += 1 ));

    let mut set_keys;
    let mut old;
    let mut unchanged = false;
    while !unchanged {
        unchanged = true;
        old = map.clone();
        for (_, val) in map.iter_mut() {
            set_keys = val.clone();
            for i in set_keys.iter() {
                val.extend(old.get(i).unwrap());
            }
        }
        if old != map {
            unchanged = false;
        }
    }
    old = map.clone();
    for (key, val) in old.iter() {
        if map.contains_key(key) {
            for set_key in val.iter() {
                if set_key != key {
                    map.remove(&set_key);
                }
            }
        }
    }
    let mut results = vec![0; 3];
    let mut temp;

    for (_, i) in map.iter() {
        temp = 0;
        for &j in i.iter() {
            temp += counts[j];
        }
        if let Some(a) = results.iter_mut().min() {
            if temp > *a {
                *a = temp;
            }
        }
    }


    println!("day 9b result:  {}", results.iter().product::<usize>());
    Ok(())
}