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

    println!("day 16a result: {}", stuf.letsgo_a());
    Ok(())

}


pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut stuf = Bits::new(lines.flat_map(|x| x.unwrap().chars().map(|x| x.to_digit(16).unwrap() as usize + 16).collect::<Vec<usize>>()).collect()); 

    println!("day 16b result: {}", stuf.letsgo());
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
            ret += self.get_num() * 2usize.pow((by - i - 1) as u32);
            self.pos += 1;
        }
        ret
    }
    fn get_four(&mut self) -> (usize, bool) {
        let mut ret = (0, false);
        ret.1 = self.advance(1) != 0;
        ret.0 = self.advance(4);
        ret
    }
    fn letsgo(&mut self) -> usize {
        let mut ret = vec![self.advance(3), self.advance(3)];
        if ret[1] == 4 {
            ret.push(0);
            while {
                let t = self.get_four(); 
                ret[2] = ret[2]*16usize + t.0;
                t.1
            } {}
        } else {
            ret.push(self.advance(1));
            ret.push(self.advance(15 - 4*ret[2]));
            match ret[2] {
                0 => {
                    let curr_pos = self.pos + ret[3];
                    while self.pos < curr_pos {
                        ret.push(self.letsgo());
                    }
                }
                1 => (0..ret[3]).for_each(|_| ret.push(self.letsgo())),
                _ => unreachable!()
            }
        }
        match ret[1] {
            0 =>  ret.iter().skip(4).sum(),
            1 =>  ret.iter().skip(4).product(),
            2 => *ret.iter().skip(4).min().unwrap(),
            3 => *ret.iter().skip(4).max().unwrap(),
            4 => ret[2],
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
                let curr_pos = self.pos + ret[3];
                while self.pos < curr_pos {
                    r += self.letsgo_a();
                }
            } else {
                r += (0..ret[3]).map(|_| self.letsgo_a()).sum::<usize>();
            }
        } else {
            while self.get_four().1 {}
        }
        r
    }
}




