use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashSet;

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut content: HashSet<(usize, usize)> = HashSet::new();

    let mut data: Vec<Vec<usize>> = Vec::new();
    let mut folds: Vec<(bool, usize)> = Vec::new();
    while let Some(a) = lines.next() {
        let line = a.unwrap(); 
        if line == "" {
            break;
        }
        data.push(line.split(',').map(|x| x.parse::<usize>().unwrap()).collect());
    }
    let mut temp;
    while let Some(a) = lines.next() {
        temp = (false, 0);
        let line = a.unwrap();
        let mut datar = line.split_whitespace();
        datar.next();
        datar.next();
        let mut datar2 = datar.next().unwrap().split('=');
        if let Some(b) = datar2.next() {
            if b == "y" {
                temp.0 = false;
            } else if b == "x" {
                temp.0 = true;
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        if let Some(c) = datar2.next() {
            temp.1 = c.parse().unwrap()
        } else {
            unreachable!()
        }
        folds.push(temp);
    }
    let mut temp;
    for i in data.iter() {
        temp = i.clone();
        for &(dir, num) in folds.iter() {
            if dir {
                if temp[0] > num {
                    temp[0] = 2*num - temp[0];
                } else if temp[0] == num {
                    panic!()
                }
            } else {
                if temp[1] > num {
                    temp[1] = 2*num - temp[1];
                } else if temp[1] == num {
                    panic!()
                }
            }
            break;
        }
        
        content.insert((temp[0], temp[1]));
    }


    println!("result: {}", content.iter().count());

    Ok(())
}
pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut content: HashSet<(usize, usize)> = HashSet::new();

    let mut data: Vec<Vec<usize>> = Vec::new();
    let mut folds: Vec<(bool, usize)> = Vec::new();
    while let Some(a) = lines.next() {
        let line = a.unwrap(); 
        if line == "" {
            break;
        }
        data.push(line.split(',').map(|x| x.parse::<usize>().unwrap()).collect());
    }
    let mut temp;
    while let Some(a) = lines.next() {
        temp = (false, 0);
        let line = a.unwrap();
        let mut datar = line.split_whitespace();
        datar.next();
        datar.next();
        let mut datar2 = datar.next().unwrap().split('=');
        if let Some(b) = datar2.next() {
            if b == "y" {
                temp.0 = false;
            } else if b == "x" {
                temp.0 = true;
            }
        }
        if let Some(c) = datar2.next() {
            temp.1 = c.parse().unwrap()
        }
        folds.push(temp);
    }
    let mut temp;
    for i in data.iter() {
        temp = i.clone();
        for &(dir, num) in folds.iter() {
            if dir {
                if temp[0] > num {
                    temp[0] = 2*num - temp[0];
                }
            } else {
                if temp[1] > num {
                    temp[1] = 2*num - temp[1];
                }
            }
        }
        
        content.insert((temp[0], temp[1]));
    }
    let maxx: usize = content.iter().map(|x| x.0).fold(0, |tot, x| if x > tot { x } else { tot } );
    let maxy: usize = content.iter().map(|x| x.1).fold(0, |tot, x| if x > tot { x } else { tot } );
    for _ in 0..maxx+2 {
        print!("..");
    }
    println!();
    for j in 0..maxy+2 {
        for i in 0..maxx+2 {
            print!(".");
            if content.contains(&(i, j)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()

    }

    Ok(())
}

