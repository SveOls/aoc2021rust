use std::fs::File;
use std::io::{BufReader, Lines};

use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}


pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let iterations = 10;

    let enhancement = lines.next().unwrap().unwrap().chars().map(|x| x == '#').collect::<Vec<bool>>();
    lines.next();
    let mut image = lines
        .map(|x| x.unwrap())
        .map(|x| x.chars()
            .fold(vec![false; 2*(iterations + 1)], |mut tot, x| { tot.push(x == '#'); tot }))
        .collect::<Vec<Vec<bool>>>();
    image.append(&mut vec![vec![false; image[0].len()]; 2*(2*(iterations+1))]);
    image.iter_mut().for_each(|x| (*x).append(&mut vec![false; 2*(iterations + 1)]));
    image.rotate_right(2*(iterations + 1));

    let mut indices = vec![vec![0; image[0].len()]; image.len()];
    let h = image.len();
    let w = image[0].len();
    let mut outside_world = false;


    for _ in 0..iterations {
        for (i, y ) in image.iter().enumerate() {
            for (j, &x) in y.iter().enumerate() {
                if x {
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
        for i in 0..h {
            for j in 0..w {
                image[i][j] = enhancement[indices[i][j]];
                indices[i][j] = 0;
            }
        }
        

        // print_img(&image);
    }

    
    let mut count = 0;
    for i in 2*iterations..h-2*iterations {
        for j in 2*iterations..h-2*iterations {
            if image[i][j] {
                count += 1;
            }
        }
    }
    println!("{}", count);

    Ok(())
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



pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let iterations = 50;

    let enhancement = lines.next().unwrap().unwrap().chars().map(|x| x == '#').collect::<Vec<bool>>();
    lines.next();
    let mut image = lines
        .map(|x| x.unwrap())
        .map(|x| x.chars()
            .fold(vec![false; 2*(iterations + 1)], |mut tot, x| { tot.push(x == '#'); tot }))
        .collect::<Vec<Vec<bool>>>();
    image.append(&mut vec![vec![false; image[0].len()]; 2*(2*(iterations+1))]);
    image.iter_mut().for_each(|x| (*x).append(&mut vec![false; 2*(iterations + 1)]));
    image.rotate_right(2*(iterations + 1));

    let mut indices = vec![vec![0; image[0].len()]; image.len()];
    let h = image.len();
    let w = image[0].len();
    let mut outside_world = false;


    for _ in 0..iterations {
        for (i, y ) in image.iter().enumerate() {
            for (j, &x) in y.iter().enumerate() {
                // if x && i > iteration && j > iteration && (w-j) > iteration && (h-i) > iteration {
                if x {
                    for i_1 in i.max(1)-1..(i+2).min(h) {
                        for j_1 in j.max(1)-1..(j+2).min(w) {
                            // println!("{} {} {} {}", i, j, i_1, j_1);
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
        for i in 0..h {
            for j in 0..w {
                image[i][j] = enhancement[indices[i][j]];
                indices[i][j] = 0;
            }
        }
    }

    
    let mut count = 0;
    for i in iterations+2..h-iterations-2 {
        for j in iterations+2..h-iterations-2 {
            if image[i][j] {
                count += 1;
            }
        }
    }
    println!("{}", count);

    Ok(())
}


