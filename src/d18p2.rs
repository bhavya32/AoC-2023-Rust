#![allow(non_snake_case)]

use core::panic;
use std::fs;

fn d18(steps: Vec<(u8, u64)>) {
    //let mut bound : Vec<(i64, i64)> = Vec::new();
    let mut curr_loc = (0,0);
    let mut area:i128 = 0;
    let mut nloc;
    let mut peri:u128 = 0;
    for mut step in steps {
        let( dx, dy ):(i128, i128)= match step.0 {
            3 => (-1, 0),
            0 => (0, 1),
            2 => (0, -1),
            1=> (1, 0),
            _=> panic!()
        };
        
        while step.1 > 0 {
            nloc = (curr_loc.0 + dx, curr_loc.1 + dy);
            area += curr_loc.0 * nloc.1;
            area -= nloc.0 * curr_loc.1;
            curr_loc = nloc;
            //bound.push(curr_loc);
            step.1 -= 1;
            peri += 1;
        }
    }
    area = area.abs()/2;
    println!("total points = {}", area + (peri/2) as i128 + 1);
}

fn main() {
    let text = fs::read_to_string("./q.txt").expect("Unable to read file");
    let lines = text.split("\r\n");
    let mut steps:Vec<(u8, u64)> = Vec::new();
    for line in lines {
        let mut t = line.split(" ");
        t.next();
        t.next();
        
        let color = t.next().unwrap();
        let d = match &color[7..8] {
            "1" => {1},
            "3" => {3},
            "2" => {2},
            "0" => {0},  
            _ => {println!("{}", &color[7..8]);panic!()}
        };
        let hex = u64::from_str_radix(&color[2..7], 16).unwrap();
        steps.push((d, hex));
    }
    d18(steps);
}
