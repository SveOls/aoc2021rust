use std::fs::File;
use std::io::{BufReader, Lines};
use std::ops::{Index, IndexMut};
use std::collections::HashSet;

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data: Map = Map::new(&lines.map(|x| x.unwrap().chars().map(|x| x.to_digit(10).unwrap() as u8).collect()).collect());

    let mut total = 0;
    for _ in 0..100 {
        data.increment();
        while data.changed() {
            // println!("{:?}", data);

            data.blast();
            // break;
            // break;
        }
        total += data.cleanup().0;
    }
    // println!("{:?}", data);

    println!("day 11a result: {}", total);

    Ok(())
}
pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data: Map = Map::new(&lines.map(|x| x.unwrap().chars().map(|x| x.to_digit(10).unwrap() as u8).collect()).collect());

    let mut step = 0;
    while !data.cleanup().1 {
        step += 1;
        data.increment();
        while data.changed() {
            // println!("{:?}", data);

            data.blast();
            // break;
            // break;
        }
    }
    println!("day 11b result: {}", step);



    Ok(())
}


struct Map {
    content: Vec<u8>,
    w: usize,
    h: usize,
    changed: bool,
    blased: HashSet<(usize, usize)>
}

impl Map {
    fn new(inp: &Vec<Vec<u8>>) -> Map {
        Map {
            h: inp.len(),
            w: inp[0].len(),
            content: inp.iter().flat_map(|x| x.iter().map(|x| *x)).collect(),
            changed: true,
            blased: HashSet::new(),
        }
    }
    fn changed(&mut self) -> bool {
        self.changed = !self.changed;
        !self.changed
    }
    fn increment(&mut self) {
        // println!("increment");
        for y in 0..self.h {
            for x in 0..self.w {
                self[y][x] += 1;
            }
        }

    }
    fn to_xy(&self, inp: usize) -> (usize, usize) {
        (inp%self.w, inp/self.w)
    }
    fn blast(&mut self) {
        // println!("blast");
        let blast_locs: Vec<(usize, usize)> = self.content.iter().enumerate().filter(|(_, val)| **val == 10).map(|(i, _)| self.to_xy(i)).collect();
        for &(x, y) in blast_locs.iter() {
            if self[y][x] == 10 && !self.blased.contains(&(x, y)) {
                self.blased.insert((x, y));
                self.blastoise(x, y)
            }
        }
    }
    fn blastoise(&mut self, x: usize, y: usize) {
        // println!("blast at {} {}", y, x);
        for j in 1.max(y)..(self.h+1).min(y+3) {
            for i in 1.max(x)..(self.w+1).min(x+3) {
                if self[j-1][i-1] != 10 {
                    self.changed = true;
                    self[j-1][i-1] += 1;
                }
                // println!("{} {}", j-1, i-1);
                // if == 10, do not increment
            }
        }
    }
    fn cleanup(&mut self) -> (usize, bool) {
        // println!("cleanup");
        self.content.iter_mut().for_each(|x| *x %= 10);
        let ret = self.blased.drain().count();
        (ret, self.content.len() == ret)
    }
}

impl Index<usize> for Map {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.content[self.w*index..self.w*index+self.w]
    }
}

impl IndexMut<usize> for Map {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[self.w*index..self.w*index+self.w]
    }
}


impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.h {
            for j in 0..self.w {
                if self[i][j] == 0 {
                    write!(f, "  ")?;   
                } else {
                    write!(f, "{} ", self[i][j])?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
