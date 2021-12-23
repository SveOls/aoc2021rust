use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;

use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;
    

    
    use std::time::Instant;
    let tests = [1000, 100000, 1000000];
    for &i in &tests {
        println!();
        let before_any = Instant::now();
        run_a_o1(file_handling::get_file(inp)?, true, i)?;
        let after_skip = before_any.elapsed();
        run_a_o1(file_handling::get_file(inp)?, false, i)?;
        let after_non_skip = before_any.elapsed();
        println!("doing {} iterations with skips took {} micros", i, after_skip.as_micros());
        println!("doing {} iterations without skips took {} micros", i, (after_non_skip - after_skip).as_micros());
        println!();
    }


    Ok(())

}


pub fn run_a_o1(mut lines: Lines<BufReader<File>>, skip: bool, turns: usize) -> Result<(), Box<dyn std::error::Error>> {

    let mut pos_1: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();
    let mut pos_2: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();

    let score = [6, 5, 4, 3, 2, 1, 0, 9, 8, 7];

    let goal = turns;

    let mut score_1 = 0;
    let mut score_2 = 0;

    let mut score_pos = 0;
    // hashmap of (pos_1, pos_2, last roll, who's up next) to (score, score, round number)
    let mut states: HashMap<(usize, usize, usize, bool), (usize, usize, usize)> = HashMap::new();
    let mut skipped = !skip;
    let mut round = 0;
    //
    for _ in 0.. {
        round += 1;
        // one for before any rounds start, and after player 2 has made their turn
        // if you're unfamilaiar with rust, this asks whether "states" contains 
        // this key already. If so, it returns the value of the hashmap as "a" by reference.
        if !skipped {
            if let Some((old_score_1, old_score_2, old_round)) = states.get(&(pos_1, pos_2, score_pos%10, true)) {
                // we can now safely iterate the score with whatever multiple of (score_1 - old_score_1) and (score_2 - old_score_2)
                // we want. So let's do that. To prove we were here: 
                println!("Skipping forward in time");
                // and fast forwarding:

                // the -1 is because if you're at 950/1000, and you can go exactly 50 forward, you'd overshoot by landing on 1000. 
                let turns_forward_1 = (goal - score_1 - 1)/(score_1 - old_score_1);
                let turns_forward_2 = (goal - score_2 - 1)/(score_2 - old_score_2);

                // both scores must be increased by the same number of rounds. to avoid overshooting, im sticking to
                // the lowest possible skip.
                let max_turns_forward = turns_forward_1.min(turns_forward_2);

                score_1 += max_turns_forward * (score_1 - old_score_1);
                score_2 += max_turns_forward * (score_2 - old_score_2);
                round   += max_turns_forward * (round   - old_round  );

                // I've already skipped forward, skipping again won't help. Probably unncessary, but who knows what 
                // bugs might show up.
                skipped = true;
    
    
            } else {
            // if it doesn't have the value, i insert it to save it for later.
                states.insert((pos_1, pos_2, score_pos%10, true), (score_1, score_2, round));
            }
            //
        }


        pos_1 = (pos_1 + score[score_pos%10]-1)%10+1;
        score_pos += 1;
        score_1 += pos_1;

        
        // you could insert another one here. I won't since it will never do anything in this case.
        // states.insert((pos_1, pos_2, score_pos, false), (score_1, score_2));
        //

        pos_2 = (pos_2 + score[score_pos%10]-1)%10+1;
        score_pos += 1;


        if score_1 >= goal {
            println!("Test result: {}", score_2 * (6*(round)-3));
            break;
        }
        score_2 += pos_2;
        if score_2 >= goal {
            println!("Test result: {}", score_1 * 6*(round));
            break;
        }
    }
    Ok(())
}



pub fn run_a(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut pos_1: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();
    let mut pos_2: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();

    let score = [6, 5, 4, 3, 2, 1, 0, 9, 8, 7];

    let mut score_1 = 0;
    let mut score_2 = 0;
    for i in 0.. {
        pos_1 = (pos_1 + score[(2*i)%10]-1)%10+1;
        pos_2 = (pos_2 + score[((2*i)+1)%10]-1)%10+1;
        score_1 += pos_1;
        if score_1 >= 1000 {
            println!("day 21a result: {}", score_2 * (6*(i+1)-3));
            break;
        }
        score_2 += pos_2;
        if score_2 >= 1000 {
            println!("day 21a result: {}", score_1 * 6*(i+1));
            break;
        }
    }
    Ok(())
}



pub fn run_b(mut lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    let pos_1: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();
    let pos_2: usize = lines.next().unwrap().unwrap().split(|x| x == ':').filter_map(|x|x.split_whitespace().collect::<String>().parse().ok() ).next().unwrap();
    
    let mut positions: HashMap<((usize, usize), (usize, usize), bool), usize> = HashMap::new();
    positions.insert(((pos_1, 0), (pos_2, 0), true),1);

    let mut universe_1 = 0;
    let mut universe_2 = 0;
    let mut changed = true;
    while changed {
        changed = false;
        let mut to_iterate_over = positions;
        positions = HashMap::new();
        for ((first, second, who) , val) in to_iterate_over.drain() {
            for i in 1..4 {
                for j in 1..4 {
                    for k in 1..4 {
                        if who {
                            let temp = (first.0+i+j+k-1)%10+1;
                            if first.1 + temp >= 21 {
                                universe_1 += val;
                            } else {
                                *positions.entry(((temp, first.1 + temp), second, !who)).or_insert(0) += val;
                            }
                        } else {
                            let temp = (second.0+i+j+k-1)%10+1;
                            if second.1 + temp >= 21 {
                                universe_2 += val;
                            } else {
                                *positions.entry((first, (temp, second.1 + temp), !who)).or_insert(0) += val;
                            }
                        }
                    }
                }
            }
            changed = true;
        }
    }
    println!("day 21b result: {}", universe_1.max(universe_2));
    
    Ok(())
}

