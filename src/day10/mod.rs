use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;


pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let stacks: Vec<Vec<Brack>> = lines.map(|x| x.unwrap().chars().map(Brack::to_brack).collect()).collect();

    let mut score_a = 0;
    let mut score_b = Vec::new();
    for stack in &stacks {
        let mut broken = false;
        let mut allowed = Vec::new();
        for brack in stack {
            if brack.opens() {
                allowed.push(brack.inverse());
            } else {
                let new = allowed.pop().unwrap();
                if !new.matcher(brack) {
                    score_a += brack.score();
                    broken = true;
                    break;
                }
            }
        }
        if !broken {
            score_b.push(allowed.iter().rev().fold(0, |tot, x| 5 * tot + x.score_b()));
        }
    }
    score_b.sort();
    println!("{}", score_a);
    println!("{}", score_b[score_b.len()/2]);

    Ok(())
}
pub fn run_b(_lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    // let stacks: Vec<Vec<Brack>> = lines.map(|x| x.unwrap().chars().map(Brack::to_brack).collect()).collect();

    // let mut score = 0;
    // for stack in &stacks {
    //     let mut broken = false;
    //     let mut allowed = Vec::new();
    //     for brack in stack {
    //         if brack.opens() {
    //             allowed.push(brack.inverse());
    //         } else {
    //             let new = allowed.pop().unwrap();
    //             if !new.matcher(brack) {
    //                 score += brack.score();
    //                 broken = true;
    //                 break;
    //             }
    //         }
    //     }
    //     if !broken {

    //     }
    // }
    // println!("{}", score);

    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Brack {
    Swish(bool),
    Whosh(bool),
    Blams(bool),
    Truck(bool),
}

impl Brack {
    fn to_brack(inp: char) -> Brack {
        match inp {
            '<' => Brack::Swish(true),
            '>' => Brack::Swish(false),
            '(' => Brack::Whosh(true),
            ')' => Brack::Whosh(false),
            '[' => Brack::Truck(true),
            ']' => Brack::Truck(false),
            '{' => Brack::Blams(true),
            '}' => Brack::Blams(false),
            _   => unreachable!()
        }
    }
    fn inverse(&self) -> Brack {
        match self {
            Brack::Swish(a) => Brack::Swish(!a),
            Brack::Whosh(a) => Brack::Whosh(!a),
            Brack::Blams(a) => Brack::Blams(!a),
            Brack::Truck(a) => Brack::Truck(!a),
        }
    }
    fn opens(&self) -> bool {
        match self {
            Brack::Swish(a) => *a,
            Brack::Whosh(a) => *a,
            Brack::Blams(a) => *a,
            Brack::Truck(a) => *a,
        }
    }
    fn matcher(&self, rhs: &Brack) -> bool {
        match self {
            Brack::Swish(_) => rhs == &Brack::Swish(true) || rhs == &Brack::Swish(false),
            Brack::Whosh(_) => rhs == &Brack::Whosh(true) || rhs == &Brack::Whosh(false),
            Brack::Blams(_) => rhs == &Brack::Blams(true) || rhs == &Brack::Blams(false),
            Brack::Truck(_) => rhs == &Brack::Truck(true) || rhs == &Brack::Truck(false),
        }
    }
    fn score(&self) -> usize {
        match self {
            Brack::Swish(false) => 25137,
            Brack::Whosh(false) => 3,
            Brack::Blams(false) => 1197,
            Brack::Truck(false) => 57,
            _ => panic!()
        }
    }
    fn score_b(&self) -> usize {
        match self {
            Brack::Swish(false) => 4,
            Brack::Whosh(false) => 1,
            Brack::Blams(false) => 3,
            Brack::Truck(false) => 2,
            _ => panic!()
        }
    }
}


impl std::fmt::Debug for Brack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Brack::Swish(true ) => write!(f, "<"),
            Brack::Swish(false) => write!(f, ">"),
            Brack::Whosh(true ) => write!(f, "("),
            Brack::Whosh(false) => write!(f, ")"),
            Brack::Truck(true ) => write!(f, "["),
            Brack::Truck(false) => write!(f, "]"),
            Brack::Blams(true ) => write!(f, "{{"),
            Brack::Blams(false) => write!(f, "}}")
        }
    }
}