use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};
use std::error::Error;



pub fn get_file(inp: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
    if let Ok(a) = File::open(format!("data//day{}.txt", inp)) {
        Ok(BufReader::new(a).lines())
    } else {
        Ok(BufReader::new(File::open(format!("data//day{}.txt", inp.chars().filter(|x| x.is_digit(10)).collect::<String>()))?).lines())
    }
}