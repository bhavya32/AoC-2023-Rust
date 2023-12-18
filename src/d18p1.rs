#![allow(non_snake_case)]

use std::fs;
use crate::Dir::*;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    r:i32,
    c:i32,
    cost:u32,
    streak:u32,
    l:(i32, i32)
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    R,
    L,
    U,
    D
}


fn d18(steps: Vec<(Dir, u32, &str)>) {
    let mut bound : Vec<(i32, i32)> = Vec::new();
    let mut curr_loc = (0,0);
    

    for mut step in steps {
        let( dx, dy ):(i32, i32)= match step.0 {
            U => (-1, 0),
            R => (0, 1),
            L => (0, -1),
            D=> (1, 0)
        };
        
        while step.1 > 0 {
            //let nloc = (curr_loc.0 as i32 + dx, curr_loc.1 as i32 + dy);
            curr_loc = (curr_loc.0 + dx, curr_loc.1 + dy);
            //viz[curr_loc.0][curr_loc.1]= ('#', step.2, step.0);
            bound.push(curr_loc);
            step.1 -= 1;
        }
    }
    
    /* run shoelace to get the area */
    let mut area = 0;
    let n = bound.len();
    for i in 0..n {
        let j = (i + 1) % n;
        area += (bound[i].0 * bound[j].1) as i32;
        area -= (bound[j].0 * bound[i].1) as i32;
    }
    area = area.abs()/2;

    //get perimeter
    let peri = bound.len() as i32;
    println!("total points = {}", area + (peri/2) + 1);
}

fn main() {
    let text = fs::read_to_string("./q.txt").expect("Unable to read file");
    let lines = text.split("\r\n");
    let mut steps:Vec<(Dir, u32, &str)> = Vec::new();
    for line in lines {
        //println!("{}|", line);
        //let t: Vec<u32> = line.chars().map(|f| f.to_digit(10).unwrap()).collect();
        let mut t = line.split(" ");
        let d = match t.next().unwrap() {
            "D" => {D},
            "U" => {U},
            "L" => {L},
            "R" => {R},  
            _ => {panic!()}
        };
        let c = t.next().unwrap().parse::<u32>().unwrap();
        let color = t.next().unwrap();
        steps.push((d, c, color));
    }
    //let viz = vec![vec![('.', ""); x[0].len()]; x.len()];
    //viz
    //djisktra(matrix);
    d18(steps);
}
