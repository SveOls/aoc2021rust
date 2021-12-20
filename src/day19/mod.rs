use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::{HashMap, HashSet};

use priority_queue::PriorityQueue;

use super::file_handling;

pub fn run(inp: &str) -> Result<(), Box<dyn std::error::Error>> {

    run_a(file_handling::get_file(inp)?)?;

    run_b(file_handling::get_file(inp)?)?;

    Ok(())

}

pub fn run_a(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {


    
    let finished = finish_beacons(lines);
    

    let mut beacons = HashSet::new();

    for (_, val) in finished.iter() {
        val.insert_beacons(&mut beacons);
    }
    println!("day 19a result: {}", beacons.len());
    

    Ok(())



}


pub fn run_b(lines: Lines<BufReader<File>>) -> Result<(), Box<dyn std::error::Error>> {

    
    let finished = finish_beacons(lines);

    let mut max = 0;
    for (_, val) in finished.iter() {
        for (_, val2) in finished.iter() {
            max = max.max(val.distance(val2));
        }
    }
    println!("day 19b result: {}", max);
    

    Ok(())
}

#[derive(Clone, Debug)]
struct Scanner {
    id: usize,
    visited: HashSet<usize>,
    data: Vec<[i64; 3]>,
    orientation: usize,
    position: Option<[i64; 3]>,
}

impl Scanner {
    fn new(id: usize, data: Vec<[i64; 3]>) -> Scanner {
        Scanner {
            id,
            visited: HashSet::new(),
            data,
            orientation: 0,
            position: None
        }
    }
    fn get(&self, pos: usize) -> [i64; 3] {
        [
            (1 - 2*((self.orientation/4)%2) as i64)                                 * self.data[pos][(self.orientation/8)%3], 
            (1 - 2*(((((self.orientation+1)%4)/2)+(self.orientation/4))%2) as i64)  * self.data[pos][((self.orientation/8) + ((self.orientation%4)/2+1) )%3], 
            (1 - 2*(self.orientation%2) as i64)                                     * self.data[pos][ ((self.orientation/8) + (((self.orientation+2)%4)/2+1) )%3]
        ]
    }
    fn distance(&self, rhs: &Self) -> i64 {
        let pos1 = self.position.unwrap();
        let pos2 = rhs .position.unwrap();
        (pos1[0] - pos2[0]).abs() + (pos1[1] - pos2[1]).abs() + (pos1[2] - pos2[2]).abs()
    }
    fn flip(&mut self, new_pos: &[i64], new_pos_2: &[i64]) {
        for i in 0..self.data.len() {
            self.data[i] = self.get(i);
        }
        self.orientation = 0;
        let mut a = [0, 0, 0];
        a.iter_mut().zip(new_pos).zip(new_pos_2).for_each(|((x, y), z)| *x = y + z );
        self.position = Some(a); 
    }
    fn increment(&mut self) {
        self.orientation += 1;
        self.orientation %= 24;
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn get_data(&self) -> &[[i64; 3]] {
        &self.data
    }
    fn zero_position(&mut self) {
        self.position = Some([0; 3])
    }
    fn insert_beacons(&self, into: &mut HashSet<[i64; 3]>) {
        let pos = self.position.unwrap();
        for i in 0..self.len() {
            let loc = self.get(i);
            into.insert([loc[0] + pos[0], loc[1] + pos[1], loc[2] + pos[2]]);
        }
    }
    fn matcher(&mut self, rhs: &mut Self) -> bool {

        if !self.visited.contains(&rhs.id) {
            'outer: for i in 11..self.len() {
                let goal = self.get(i);
                let lhs_data = self.get_data();
                for _ in 0..24 {
                    for j in 11..rhs.len() {
    
                        let mut trn = rhs.get(j);
                    
                        trn.iter_mut().zip(goal).for_each(|(x1, x2)| *x1 = x2 - *x1);
                        

                        let to_check_r = (0..rhs.len())
                            .map(|x| rhs.get(x))

                            .map(|x| [
                                x[0] + trn[0],
                                x[1] + trn[1],
                                x[2] + trn[2]
                                ])
    
                            .filter(|&x| x.iter().map(|x| x.abs())
                                .max().unwrap() <= 1000)
                            .map(|x| if lhs_data.contains(&x) { Some(true) } else { None })
                            .scan((), |_, item| item)
                            .fold((true, 0), |(bol, num), x| (x && bol, num + 1));

                        // i think this speeds things up
                        if to_check_r.0 {

                            let to_check_l = (0..self.len())
                                .map(|x| self.get(x))
                                .filter(|&x| x.iter()
                                    .enumerate()
                                    .map(|(i, x)| (*x - trn[i]).abs())
                                    .max().unwrap() <= 1000)
                                .count();
                            

                            if to_check_r.0 && to_check_l == to_check_r.1 && to_check_l >= 12 {
                                rhs.flip(&trn, &self.position.unwrap());
                                self.visited.insert(rhs.id);
                                
                                break 'outer;
                            }
                        }
                        
                    }
                    rhs.increment();
                }
            }
            !self.visited.insert(rhs.id)
        } else {
            false
        }
    }
}


fn finish_beacons(mut lines: Lines<BufReader<File>>) -> HashMap<usize, Scanner> {
    let mut data = HashMap::new();
    let mut i = 0;
    while let Some(_) = lines.next() {
        let mut temp: Vec<[i64; 3]> = Vec::new();
        let mut temp2: Vec<i64>;
        while let Some(b) = lines.next() {
            let line = b.unwrap();
            if !line.is_empty() {
                temp2 = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
                temp.push([temp2[0], temp2[1], temp2[2]]);
            } else {
                break;
            }
        }
        data.insert(i, Scanner::new(i, temp));
        i += 1;
    }

    let mut queue: PriorityQueue<usize, i64> = PriorityQueue::new();
    queue.push(0, data.get(&0).unwrap().data.len() as i64);

    let mut finished = HashMap::new();
    let mut temp = data.remove(&0).unwrap();
    temp.zero_position();
    finished.insert(0, temp);

    
    let mut temp: Option<(usize, usize, i64)>;
    while data.len() > 0 {
        temp = None;
        // 'outer: for (key_out, finito) in finished.iter_mut() {
        // println!("{:?}", queue);
        let t_queue = queue;
        queue = PriorityQueue::new();
        let mut ttt = t_queue.into_sorted_iter();
        let mut fin = false;
        for (key_out, val) in ttt.by_ref() {
            // println!("with: {}", key_out);
            queue.push(key_out, val);
            if !fin {
                let finito = finished.get_mut(&key_out).unwrap();
                // println!("(confirming: {})", finito.id);
                for (key, progresso) in data.iter_mut() {
                    // println!("trying: {}", key);
                    if finito.matcher(progresso) {
                        temp = Some((key_out, *key, val));
                        fin = true;
                        break;
                    }
                }
            }
        }
        // ttt.inspect(|x| println!("{:?}", x)).for_each(|(i, val)| { queue.push(i, val); } );
        if let Some(a) = temp {
            let victorious = data.remove(&a.1).unwrap();
            queue.push(a.1, victorious.data.len() as i64);
            queue.change_priority(&a.0, a.2 - 12);
            finished.insert(a.1, victorious);
        } else {
            panic!("{}", finished.len());
        }
    }
    finished
}