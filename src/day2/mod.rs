use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

struct Submarine {
    x: i64,
    y: i64,
    aim: i64,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            x: 0,
            y: 0,
            aim: 0
        }
    }
    pub fn forward(&mut self, x: i64) {
        self.x += x
    }
    pub fn aim_forward(&mut self, x: i64) {
        self.x += x;
        self.y += self.aim * x;
    }
    pub fn down(&mut self, x: i64) {
        self.y += x
    }
    pub fn up(&mut self, x: i64) {
        self.y -= x
    }
    pub fn aim_down(&mut self, x: i64) {
        self.aim += x
    }
    pub fn aim_up(&mut self, x: i64) {
        self.aim -= x
    }
    pub fn get_depth(&self) -> i64 {
        self.y
    }
    pub fn get_position(&self) -> i64 {
        self.x
    }
}

pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut sub = Submarine::new();
    let mut temp;
    
    while let Some(a) = lines.next() {
        if let Ok(line) = a {
            temp = line.split(' ');
            match temp.next().unwrap() {
                "forward"   => sub.forward(temp.next().unwrap().parse().unwrap()),
                "down"      => sub.down(   temp.next().unwrap().parse().unwrap()),
                "up"        => sub.up(     temp.next().unwrap().parse().unwrap()),
                _ => panic!()
            }
        } else {
            panic!()
        }
    }

    println!("{}", sub.get_depth() * sub.get_position());
    Ok(())

}

pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut sub = Submarine::new();
    let mut temp;
    
    while let Some(a) = lines.next() {
        if let Ok(line) = a {
            temp = line.split(' ');
            match temp.next().unwrap() {
                "forward"   => sub.aim_forward(temp.next().unwrap().parse().unwrap()),
                "down"      => sub.aim_down(   temp.next().unwrap().parse().unwrap()),
                "up"        => sub.aim_up(     temp.next().unwrap().parse().unwrap()),
                _ => panic!()
            }
        } else {
            panic!()
        }
    }

    println!("{}", sub.get_depth() * sub.get_position());
    Ok(())
}