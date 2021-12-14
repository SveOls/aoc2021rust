use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashSet;
use std::collections::HashMap;


use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    // let mut original: Vec<(char, Option<usize>)> = lines.next().unwrap().unwrap().chars().enumerate().map(|x| (x.1, Some(x.0 + 1))).collect();
    // let temp = original.len();
    // original[temp-1].1 = None;
    // let mut set = HashSet::new();
    // for i in original.iter() {
    //     set.insert(i.0);
    // }
    // lines.next();

    // let temp: Vec<Vec<char>> = lines.map(|x| x.unwrap().split(" -> ").flat_map(|x| x.chars().collect::<Vec<char>>()).inspect(|&x| {set.insert(x);}).collect()).collect();
    // let mut data: HashMap<(char, char), char> = HashMap::new();
    // for i in temp.iter() {
    //     data.insert((i[0], i[1]), i[2]);
    // }


    // for _ in 0..10 {
    //     for i in 0..original.len() {
    //         if let Some(i_next) = original[i].1 {
    //             if let Some(&ins) = data.get(&(original[i].0, original[i_next].0)) {
    //                 original.push((ins, Some(i_next)));
    //                 original[i].1 = Some(original.len() - 1);
    //             }
    //         }
    //     }
    // }

    // let mut max = 0;
    // let mut min = std::usize::MAX;
    // for i in set.iter() {
    //     if original.iter().filter(|(x, _)| x == i).count() > max {
    //         max = original.iter().filter(|(x, _)| x == i).count()
    //     }
    //     if original.iter().filter(|(x, _)| x == i).count() < min {
    //         min = original.iter().filter(|(x, _)| x == i).count()
    //     }
    // }
    // println!("{} - {} = {}", max, min, max - min);


    // Ok(())

    let original: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    lines.next();

    let temp: Vec<Vec<char>> = lines.map(|x| x.unwrap().split(" -> ").flat_map(|x| x.chars().collect::<Vec<char>>()).collect()).collect();
    let mut data: HashMap<(char, char), char> = HashMap::new();
    for i in temp.iter() {
        data.insert((i[0], i[1]), i[2]);
    }



    let data = start_recfunc(original, &data, 10);

    let max = data.iter().map(|(_, val)| val).max().unwrap();
    let min = data.iter().map(|(_, val)| val).min().unwrap();

    
    println!("{} - {} = {}", max, min, max - min);

    Ok(())
}
pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let original: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    lines.next();

    let temp: Vec<Vec<char>> = lines.map(|x| x.unwrap().split(" -> ").flat_map(|x| x.chars().collect::<Vec<char>>()).collect()).collect();
    let mut data: HashMap<(char, char), char> = HashMap::new();
    for i in temp.iter() {
        data.insert((i[0], i[1]), i[2]);
    }



    let data = start_recfunc(original, &data, 40);

    let max = data.iter().map(|(_, val)| val).max().unwrap();
    let min = data.iter().map(|(_, val)| val).min().unwrap();

    
    println!("{} - {} = {}", max, min, max - min);

    Ok(())
}

fn start_recfunc(inp: Vec<char>, compare: &HashMap<(char, char), char>, maxdepth: usize) -> HashMap<char, usize> {
    let mut ret = HashMap::new();
    for i in inp.iter() {
        *ret.entry(*i).or_insert(0) += 1;
    }
    let mut saved = HashMap::new();
    for i in 0..inp.len()-1 {
        let temp = recfunc((inp[i], inp[i+1]), &mut saved, compare, 0, maxdepth);
        for (key, val) in temp {
            ret.entry(key).and_modify(|v| *v += val).or_insert(val);
        }
    }
    ret
}


fn recfunc(inp: (char, char), saved: &mut HashMap<(char, char, usize), HashMap<char, usize>>, compare: &HashMap<(char, char), char>, depth: usize, maxdepth: usize) -> HashMap<char, usize> {
    let mut ret = HashMap::new();
    if depth < maxdepth {
        if let Some(&a) = compare.get(&inp) {
            *ret.entry(a).or_insert(0) += 1;
            
            let temp1;
            if let Some(a) = saved.get(&(inp.0, a, depth)) {
                temp1 = a.clone();
            } else {
                temp1 = recfunc((inp.0, a), saved, compare, depth + 1, maxdepth);
                saved.insert((inp.0, a, depth), temp1.clone());
            }
            for (key, val) in temp1 {
                ret.entry(key).and_modify(|v| *v += val).or_insert(val);
            }
            let temp2;
            if let Some(a) = saved.get(&(a, inp.1, depth)) {
                temp2 = a.clone();
            } else {
                temp2 = recfunc((a, inp.1), saved, compare, depth + 1, maxdepth);
                saved.insert((a, inp.1, depth), temp2.clone());
            }
            for (key, val) in temp2 {
                ret.entry(key).and_modify(|v| *v += val).or_insert(val);
            }
        }
    }
    ret
}