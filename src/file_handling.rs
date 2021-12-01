use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};
use std::error::Error;



pub fn get_file(inp: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
    Ok(BufReader::new(File::open(format!("data//day{}.txt", inp))?).lines())
}