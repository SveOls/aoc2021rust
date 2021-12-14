use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let draws: Vec<u16>  = lines.next().unwrap().unwrap().split(',').map(|x| x.parse().unwrap()).collect();
    let mut boards = Vec::new();
    let mut tempboard;
    let mut templine;
    let mut templineiter;

    'outer: while let Some(_) = lines.next() {
        tempboard = Board::new();
        for i in 0..5 {
            templine = match lines.next() {
                Some(a) => a.unwrap(),
                None => break 'outer,
            };
            templineiter = templine.split(' ').filter_map(|x| x.parse().ok());
            for j in 0..5 {
                tempboard.add(j, i, templineiter.next().unwrap());
            }
        }
        boards.push(tempboard);
    }
    'outer2: for i in draws.iter() {
        for board in boards.iter_mut() {
            if board.draw(*i) {
                println!("day 4a result:  {}", board.score() * i);
                break 'outer2;
            }
        }
    }


    Ok(())

}

pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let draws: Vec<u16>  = lines.next().unwrap().unwrap().split(',').map(|x| x.parse().unwrap()).collect();
    let mut boards = Vec::new();
    let mut tempboard;
    let mut templine;
    let mut templineiter;

    'outer: while let Some(_) = lines.next() {
        tempboard = Board::new();
        for i in 0..5 {
            templine = match lines.next() {
                Some(a) => a.unwrap(),
                None => break 'outer,
            };
            templineiter = templine.split(' ').filter_map(|x| x.parse().ok());
            for j in 0..5 {
                tempboard.add(j, i, templineiter.next().unwrap());
            }
        }
        boards.push(tempboard);
    }
    let mut last_score = 0;
    for i in draws.iter() {
        for board in boards.iter_mut() {
            if board.draw(*i) && !board.poke() {
                last_score = board.score() * i;
                board.kill();
            }
        }
        if boards.iter().filter(|x| !x.poke()).count() == 0 {
            println!("day 4b result:  {}", last_score);
            break;
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Board {
    numbers: [(u16, bool); 25],
    dead: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            numbers: [(0, false); 25],
            dead: false
        }
    }
    pub fn add(&mut self, x: usize, y: usize, val: u16) {
        self.numbers[5 * y + x] = (val, false)
    }
    pub fn draw(&mut self, val: u16) -> bool {
        let mut win = false;
        'outer: for y in 0..5 {
            for x in 0..5 {
                if self.numbers[5*y + x].0 == val {
                    self.numbers[5*y + x].1 = true;
                    if !win {
                        win = true;
                        for j in 0..5 {
                            win = win && self.numbers[5*j + x].1;
                        }
                    }
                    if !win {
                        win = true;
                        for i in 0..5 {
                            win = win && self.numbers[5*y + i].1;
                        }
                    }
                    break 'outer;
                }
            }
        }
        win
    }
    pub fn score(&self) -> u16 {
        self.numbers.iter().filter(|x| !x.1).fold(0, |tot, x| tot + x.0)
    }
    pub fn kill(&mut self) {
        self.dead = true
    }
    pub fn poke(&self) -> bool {
        self.dead
    }
}