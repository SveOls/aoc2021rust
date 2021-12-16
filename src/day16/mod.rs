use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}


pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut stuf = Bits::new(lines.flat_map(|x| x.unwrap().chars().map(|x| x.to_digit(16).unwrap() as usize + 16).collect::<Vec<usize>>()).collect()); 

    println!("day16a result: {}", stuf.letsgo_a());
    Ok(())

}


pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut stuf = Bits::new(lines.flat_map(|x| x.unwrap().chars().map(|x| x.to_digit(16).unwrap() as usize + 16).collect::<Vec<usize>>()).collect()); 

    println!("day16b result: {}", stuf.letsgo());
    Ok(())

}


struct Bits {
    data: Vec<usize>,
    pos: usize,
}

impl Bits {
    fn new(data: Vec<usize>) -> Bits {
        Bits {
            data,
            pos: 0 
        }
    }  
    fn get_num(&self) -> usize {
        if self.data[self.pos/4] & (1 << (3 - self.pos%4)) == 0 {
            0
        } else {
            1
        }
    }
    fn advance(&mut self, by: usize) -> usize {
        let mut ret = 0;
        for i in 0..by {
            let temp = self.get_num();
            ret += temp * (2*temp).pow((by - i - 1) as u32);
            self.pos += 1;
        }
        ret
    }
    fn letsgo(&mut self) -> usize {
        let mut ret = vec![self.advance(3), self.advance(3)];
        if ret[1] != 4 {
            ret.push(self.advance(1));
            ret.push(self.advance(15 - 4*ret[2]));
            if ret[2] == 0 {
                let curr_pos = self.pos;
                while self.pos - curr_pos < ret[3] {
                    ret.push(self.letsgo());
                }
            } else {
                for _ in 0..ret[3] {
                    ret.push(self.letsgo());
                }
            }
        } else {
            while self.advance(1) == 1 {
                ret.push(self.advance(4));
            }            
            ret.push(self.advance(4));
            let mut total = 0;
            for i in 2..ret.len() {
                total += ret[i]*16usize.pow((ret.len()-i-1) as u32);
            }
            ret.push(total);
        }
        match ret[1] {
            0 =>  ret.iter().skip(4).sum(),
            1 =>  ret.iter().skip(4).product(),
            2 => *ret.iter().skip(4).min().unwrap(),
            3 => *ret.iter().skip(4).max().unwrap(),
            4 => *ret.iter().last().unwrap(),
            5 => (ret[4] >  ret[5]) as usize,
            6 => (ret[4] <  ret[5]) as usize,
            7 => (ret[4] == ret[5]) as usize,
            _ => unreachable!()
        }
    }
    fn letsgo_a(&mut self) -> usize {
        // ret[0] == version, ret[1] == packet id, ret[2] == length (0 = 15, 1 = 11)
        let mut ret = vec![self.advance(3), self.advance(3)];
        let mut r = ret[0];
        if ret[1] != 4 {
            ret.push(self.advance(1));
            ret.push(self.advance(15 - 4*ret[2]));
            if ret[2] == 0 {
                let curr_pos = self.pos;
                while self.pos - curr_pos < ret[3] {
                    r += self.letsgo_a();
                }
            } else {
                for _ in 0..ret[3] {
                    r += self.letsgo_a();
                }
            }

        } else {
            while self.advance(1) == 1 {
                ret.push(self.advance(4));
            }
            ret.push(self.advance(4));
        }
        r
    }
}
