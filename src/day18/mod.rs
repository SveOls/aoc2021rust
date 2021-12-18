use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    println!("day 18a result: {}", lines.map(|x| Comm::new(x.unwrap())).fold(Comm::None, |tot, x| tot + x).magnitude());


    Ok(())



}


pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    
    let data: Vec<Comm> = lines.map(|x| Comm::new(x.unwrap())).collect();

    let mut max_mag = 0;
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i != j {
                max_mag = max_mag.max((data[i].clone() + data[j].clone()).magnitude());
            }
        }
    }
    println!("day 18b result: {}", max_mag);


    Ok(())

}


#[derive(Clone)]
enum Comm {
    A(Box<Comm>, Box<Comm>),
    B(i64),
    None
}


impl Comm {
    fn new(inp: String) -> Comm {
        if let Some(a) = inp.parse::<i64>().ok() {
            Comm::B(a)
        } else {
            let mut position = 0;
            // let mut open = inp.chars().filter(|&x| x == '[').count();
            let mut open = 0;
            // println!("");
            for (i, ch) in inp.chars().enumerate() {
                // println!("{}", open);
                match ch {
                    '[' => open += 1,
                    ']' => open -= 1,
                    ',' if open == 1 =>  position = i,
                    _ => {}
                }
            }
            Comm::A(Box::new(Comm::new(inp[1..position].to_owned())), Box::new(Comm::new(inp[position+1..inp.chars().count()-1].to_owned())))
        }
    }
    fn split(&mut self) -> bool {
        match self {
            Comm::A(a, b) => {
                let c = a.split();
                let d = if !c {
                    b.split()
                } else { false };
                c || d
            }
            Comm::B(a) => {
                if *a >= 10 {
                    *self = match std::mem::replace(self, Comm::None) {
                        Comm::B(a) => Comm::A(Box::new(Comm::B(a/2)), Box::new(Comm::B(a/2 + a%2))),
                        v => v
                    };
                    true
                } else {
                    false
                }
            }
            _ => panic!()
        }
    }
    fn adder(&mut self, from: bool, with: i64) {
        // false = from left, true = from right
        match self {
            Comm::A(a, b) => {
                if from {
                    b.adder(from, with);
                } else {
                    a.adder(from, with);
                }
            }
            Comm::B(a) => *a += with,
            Comm::None => panic!()
        }
    }
    fn pangs(&mut self, depth: usize) -> Option<(i64, i64)> {
        match self {
            Comm::A(a, b) => {
                if depth == 4 {
                    let left = a.pangs(depth + 1).unwrap().0;
                    let right = b.pangs(depth + 1).unwrap().0;
                    *self = match std::mem::replace(self, Comm::None) {
                        Comm::A(_, _) => Comm::B(0),
                        v => v
                    };
                    Some((left, right))
                } else {
                    let mut ret = None;
                    if let Some(a) = a.pangs(depth + 1) {
                        b.adder(false, a.1);
                        ret = Some((a.0, 0))
                    } else if let Some(b) = b.pangs(depth + 1) {
                        a.adder(true, b.0);
                        ret = Some((0, b.1));
                    }
                    ret
                }
            }
            Comm::B(a) => {
                if depth > 4 {
                    Some((*a, *a))
                } else {
                    None
                }
            }
            _ => panic!()
        }
    }
    fn magnitude(&self) -> i64 {
        match self {
            Comm::A(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
            Comm::B(a) => *a,
            Comm::None => panic!()    
        }
    }
}

use std::ops::Add;
impl Add for Comm {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut ret = match self {
            Comm::None => other,
            a => Comm::A(Box::new(a), Box::new(other))
        };
        // tried turning this into a normal while loop. Didn't work. So have this monstrosity.
        while {
            while let Some(_) = ret.pangs(0) { }
            ret.split()
        } {}
        ret
    }
}

use std::fmt;
impl fmt::Debug for Comm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comm::A(a, b) => {
                write!(f, "[")?;
                write!(f, "{:?}", a)?;
                write!(f, ", ")?;
                write!(f, "{:?}", b)?;
                write!(f, "]")
            }
            Comm::B(a) => {
                write!(f, "{:?}", a)
            }
            Comm::None => Ok(())
        }
    }
}