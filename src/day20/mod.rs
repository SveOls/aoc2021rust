use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}


pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box::<dyn std::error::Error>> {
    println!("day 20a result: {}", runner(lines, 2)?);
    Ok(())
}

pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box::<dyn std::error::Error>> {
    println!("day 20b result: {}", runner(lines, 50)?);
    Ok(())
}


pub fn runner(mut lines: Lines<BufReader<File>>, iterations: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let extra_space = iterations+2;

    let enhancement = lines.next().unwrap().unwrap().chars().map(|x| x == '#').collect::<Vec<bool>>();
    lines.next();
    let mut image = lines
        .map(|x| x.unwrap())
        .map(|x| x.chars()
            .fold(vec![false; extra_space], |mut tot, x| { tot.push(x == '#'); tot }))
        .collect::<Vec<Vec<bool>>>();
    image.append(&mut vec![vec![false; image[0].len()]; 2*extra_space]);
    image.iter_mut().for_each(|x| (*x).append(&mut vec![false; extra_space]));
    image.rotate_right(extra_space);

    let mut indices = vec![vec![0; image[0].len()]; image.len()];
    let h = image.len();
    let w = image[0].len();
    let mut outside_world = false;

    for itera in 0..iterations {
        indices.iter_mut().for_each(|x| x.iter_mut().for_each(|x| *x = 0));
        let offset = extra_space-2-itera;
        for i in offset..h-offset {
            for j in offset..w-offset {
                if image[i][j] {
                    for i_1 in i.max(1)-1..(i+2).min(h) {
                        for j_1 in j.max(1)-1..(j+2).min(w) {
                            indices[i_1][j_1] += 2usize.pow((3*((i_1+1)-i) + ((j_1+1)-j)) as u32);
                        }
                    }
                }
            }
        }
        outside_world = if outside_world {
            enhancement[511]
        } else {
            enhancement[0]
        };
        for i in offset-1..h-offset+1 {
            if i == offset || i == h-offset-1 || i == offset - 1 || i == h-offset {
                for j in offset-1..w-offset+1 {
                    image[i][j] = outside_world;
                }   
            } else {
                image[i][offset-1] = outside_world;
                image[i][offset] = outside_world;
                image[i][w-offset-1] = outside_world;
                image[i][w-offset] = outside_world;
                for j in offset+1..w-offset-1 {
                    image[i][j] = enhancement[indices[i][j]];
                }
            }
        }
        // _print_img(&image);
    }

    let mut count = 0;
    for i in 2..h-2 {
        for j in 2..w-2 {
            if image[i][j] {
                count += 1;
            }
        }
    }

    Ok(count)
}



fn _print_img(image: &Vec<Vec<bool>>) {
    for i in image {
        for &j in i {
            if j {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}