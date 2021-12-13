use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::{BTreeSet, HashMap};

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {


    let mut data = vec![(Vec::new(), false); 20];

    let mut mapper = HashMap::new();
    mapper.insert("start".to_owned(), 0);
    mapper.insert("end".to_owned(), 1);
    let mut count = 2;

    while let Some(a) = lines.next() {
        let line = a.unwrap();
        let stuff: Vec<&str> = line.split('-').collect();
        let first = *mapper.entry(stuff[0].to_owned()).or_insert(count);
        if mapper.len() > count {
            count += 1;
        }
        let second = *mapper.entry(stuff[1].to_owned()).or_insert(count);
        if mapper.len() > count {
            count += 1;
        }
        data[first ].0.push(second);
        data[second].0.push(first);
        data[first ].1 = stuff[0].chars().fold(false, |_, x| !x.is_uppercase());
        data[second].1 = stuff[1].chars().fold(false, |_, x| !x.is_uppercase());

    }

    println!("Result: {}", recfunc(&data, BTreeSet::new(), 0, false, &mut HashMap::new()));

    Ok(())
}
pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data = vec![(Vec::new(), false); 20];

    let mut mapper = HashMap::new();
    mapper.insert("start".to_owned(), 0);
    mapper.insert("end".to_owned(), 1);
    let mut count = 2;

    while let Some(a) = lines.next() {
        let line = a.unwrap();
        let stuff: Vec<&str> = line.split('-').collect();
        let first = *mapper.entry(stuff[0].to_owned()).or_insert(count);
        if mapper.len() > count {
            count += 1;
        }
        let second = *mapper.entry(stuff[1].to_owned()).or_insert(count);
        if mapper.len() > count {
            count += 1;
        }
        data[first ].0.push(second);
        data[second].0.push(first);
        data[first ].1 = stuff[0].chars().fold(false, |_, x| !x.is_uppercase());
        data[second].1 = stuff[1].chars().fold(false, |_, x| !x.is_uppercase());

    }

    println!("Result: {}", recfunc(&data, BTreeSet::new(), 0, true, &mut HashMap::new()));

    Ok(())
}


fn recfunc(data: &Vec<(Vec<usize>, bool)>, mut visited: BTreeSet<usize>, position: usize, oneup: bool, memo: &mut HashMap<(usize, BTreeSet<usize>, bool), usize>) -> usize {
    if let Some(a) = memo.get(&(position, visited.clone(), oneup)) {
        *a
    } else {
        let mut ret = 0;
        if data[position].1 {
            visited.insert(position);
        }
        for paths in data[position].0.iter() {
            match *paths {
                1   => ret += 1,
                2.. => {
                    if !visited.contains(paths) {
                        ret += recfunc(&data, visited.clone(), *paths,  oneup, memo)
                    } else if oneup {
                        ret += recfunc(&data, visited.clone(), *paths, !oneup, memo)
                    }
                }
                _ => {}
            }
        }
        memo.insert((position, visited, oneup), ret);
        ret
    }
}