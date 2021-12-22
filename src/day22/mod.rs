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
    
    let xrange: i128 = 50;
    let mut data: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 2*(xrange+1) as usize +3]; 2*(xrange+1) as usize +1]; 2*(xrange+1) as usize +1];

    let mut info = Vec::new();
    while let Some(a) = lines.next() {
        let mut temp: (Vec<i128>, bool) = (Vec::new(), false);
        let line = a?;
        let mut t = line.split_whitespace();
        temp.1 = match t.next() {
            Some("on") => true,
            Some("off") => false,
            _ => panic!()
        };
        temp.0 = t.next().unwrap()
            .split(|x: char| !x.is_digit(10) && x != '-')
            .filter_map(|x| x.parse().ok())
            .map(|x: i128| x.max(-xrange-1))
            .map(|x: i128| x.min(xrange+1))
            .collect();
        info.push(temp);
    }
    
    for (vals, status) in info.drain(..) {
        for x in vals[0]..vals[1]+1 {
            for y in vals[2]..vals[3]+1 {
                for z in vals[4]..vals[5]+1 {
                    data[(x + xrange + 1) as usize][(y + xrange + 1) as usize][(z + xrange + 1) as usize] = status;
                }
            }   
        }
    }
    let mut count = 0;
    for i in 1..data.len()-1 {
        for j in 1..data[0].len()-1 {
            for k in 1..data[0][0].len()-1 {
                count += data[i][j][k] as usize;
            }
        }
    }
    println!("{}", count);
    Ok(())
}


pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    

    let mut info = Vec::new();
    while let Some(a) = lines.next() {
        let mut temp: (Vec<i128>, bool) = (Vec::new(), false);
        let line = a?;
        let mut t = line.split_whitespace();
        temp.1 = match t.next() {
            Some("on") => true,
            Some("off") => false,
            _ => panic!()
        };
        temp.0 = t.next().unwrap()
            .split(|x: char| !x.is_digit(10) && x != '-')
            .filter_map(|x| x.parse().ok())
            .collect();
        info.push(temp);
    }

    let mut state = CubeColl::new();
    for (data, bol) in info.drain(..).rev() {
        state.add_cube(Cube::new(data, bol));
    }
    println!("{}", state.count());
    
    Ok(())
}


#[derive(Debug)]
struct CubeColl {
    i: Vec<Cube>
}

impl CubeColl {
    fn new() -> CubeColl {
        CubeColl { i: Vec::new() }
    }
    fn add_cube(&mut self, inp: Cube) {
        if !inp.status {
            self.i.push(inp);
        }  else {
            let mut change = true;
            let mut to_insert = HashSet::new();
            to_insert.insert(inp);
            while change {
                change = false; 
                let mut temp = (Vec::new(), None);
                'outer: for i in to_insert.iter() {
                    for old_c in self.i.iter() {
                        if let Some(a) = i.overlap(old_c) {
                            temp = (a, Some(i.clone()));
                            change = true;
                            break 'outer;
                        }
                    }
                }
                if let Some(a) = temp.1 {
                    to_insert.remove(&a);
                }
                for i in temp.0 {
                    to_insert.insert(i);
                }
            }   
            for i in to_insert.drain() {
                self.i.push(i);
            }
        }
    }

    fn count(&self) -> i128 {
        self.i.iter().filter(|x| x.status).map(|x| x.sum()).sum()
    }
}


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Cube {
    ranges: Vec<i128>,
    status: bool
}

impl Cube {
    fn new(ranges: Vec<i128>, status: bool) -> Cube {
        Cube {
            ranges, status
        }
    }
    fn overlap(&self, rhs: &Self) -> Option<Vec<Cube>> {
        if self.is_overlap(rhs) {
            let mut ret = Vec::new();
            
            let under_x = self.get_x();
            let over_x = rhs.get_x();
            let under_y = self.get_y();
            let over_y = rhs.get_y();
            let under_z = self.get_z();
            let over_z = rhs.get_z();

            if under_x[0] < over_x[0] {
                ret.push(Cube::new(vec![
                    under_x[0], over_x[0]-1, 
                    under_y[0], under_y[1], 
                    under_z[0], under_z[1]
                ], self.status));
            }
            if over_x[1] < under_x[1] {
                ret.push(Cube::new(vec![
                    over_x[1]+1, under_x[1], 
                    under_y[0], under_y[1], 
                    under_z[0], under_z[1]
                ], self.status));
            }
            if under_y[0] < over_y[0] {
                ret.push(Cube::new(vec![
                    over_x[0].max(under_x[0]), over_x[1].min(under_x[1]), 
                    under_y[0], over_y[0]-1, 
                    under_z[0], under_z[1]
                ], self.status));
            }
            if over_y[1] < under_y[1] {
                ret.push(Cube::new(vec![
                    over_x[0].max(under_x[0]), over_x[1].min(under_x[1]), 
                    over_y[1]+1, under_y[1], 
                    under_z[0], under_z[1]
                ], self.status));
            }
            if under_z[0] < over_z[0] {
                ret.push(Cube::new(vec![
                    over_x[0].max(under_x[0]), over_x[1].min(under_x[1]), 
                    over_y[0].max(under_y[0]), over_y[1].min(under_y[1]), 
                    under_z[0], over_z[0]-1
                ], self.status));
            }
            if over_z[1] < under_z[1] {
                ret.push(Cube::new(vec![
                    over_x[0].max(under_x[0]), over_x[1].min(under_x[1]), 
                    over_y[0].max(under_y[0]), over_y[1].min(under_y[1]), 
                    over_z[1]+1, under_z[1]
                ], self.status));
            }
            Some(ret)    
        } else {
            None
        }
    }
    fn get_x(&self) -> [i128; 2] {
        [self.ranges[0], self.ranges[1]]
    }
    fn get_y(&self) -> [i128; 2] {
        [self.ranges[2], self.ranges[3]]
    }
    fn get_z(&self) -> [i128; 2] {
        [self.ranges[4], self.ranges[5]]
    }
    fn sum(&self) -> i128 {
        if self.status {
            (self.ranges[1]-self.ranges[0]+1)*(self.ranges[3]-self.ranges[2]+1)*(self.ranges[5]-self.ranges[4]+1)
        } else {
            0
        }
    }
    fn is_overlap(&self, rhs: &Self) -> bool {
        (((self.ranges[0] >= rhs.ranges[0] && self.ranges[0] <= rhs.ranges[1])   ||
          (self.ranges[1] >= rhs.ranges[0] && self.ranges[1] <= rhs.ranges[1]))  ||
         ((rhs.ranges[0] >= self.ranges[0] && rhs.ranges[0] <= self.ranges[1])   ||
          (rhs.ranges[1] >= self.ranges[0] && rhs.ranges[1] <= self.ranges[1]))) &&
         
        (((self.ranges[2] >= rhs.ranges[2] && self.ranges[2] <= rhs.ranges[3])   ||
          (self.ranges[3] >= rhs.ranges[2] && self.ranges[3] <= rhs.ranges[3]))  ||
         ((rhs.ranges[2] >= self.ranges[2] && rhs.ranges[2] <= self.ranges[3])   ||
          (rhs.ranges[3] >= self.ranges[2] && rhs.ranges[3] <= self.ranges[3]))) &&
        
        (((self.ranges[4] >= rhs.ranges[4] && self.ranges[4] <= rhs.ranges[5])   ||
          (self.ranges[5] >= rhs.ranges[4] && self.ranges[5] <= rhs.ranges[5]))  ||
         ((rhs.ranges[4] >= self.ranges[4] && rhs.ranges[4] <= self.ranges[5])   ||
          (rhs.ranges[5] >= self.ranges[4] && rhs.ranges[5] <= self.ranges[5]))) 
    }
}