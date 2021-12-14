use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

#[allow(warnings)]
pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    unimplemented!();
    Ok(())



}


#[allow(warnings)]
pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    
    unimplemented!();
    Ok(())

}

