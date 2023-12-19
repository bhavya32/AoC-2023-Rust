#![allow(non_snake_case)]

use core::panic;
use std::{fs, collections::HashMap};

fn check(w:&HashMap<String, Vec<(usize, u8, u32, &str)>>, p:&Vec<u32>) -> bool {
    let mut curr = "in";

    loop {
        if curr == "R" {return false;}
        if curr == "A" {return true;}
        let conds = w.get(curr).unwrap();
        for cd in conds {
            if cd.0 == 4 {
                if cd.3 == "R" {return false;}
                if cd.3 == "A" {return true;}
                curr = cd.3;
                break;
            }

            if cd.1 == 0 && p[cd.0] < cd.2 {
                curr = cd.3;
                break;
            }
            if cd.1 == 1 && p[cd.0] > cd.2 {
                curr = cd.3;
                break;
            }
        }
    }
}

fn d19(workflow:HashMap<String, Vec<(usize, u8, u32, &str)>>, parts:Vec<Vec<u32>>) {
    
    let mut c:u64 = 0;
    for p in parts {
        if check(&workflow, &p) {
            c += (p[0] + p[1] + p[2] +p[3]) as u64;
        }
    }
    println!("{c}");

}

fn get_fn(c:&str) -> (usize, u8, u32, &str) {
    if !c.contains(':') {
        return (4, 0, 0, c);
    }
    let a = match &c[0..1] {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("{c}")
    };
    if a == 4 || a == 5 {return (a, 0, 0, "");}
    let b = match &c[1..2]{
        "<" => 0,
        ">" => 1,
        _ => panic!()
    };
    let idx = c.find(':').unwrap();
    let x = c[2..idx].parse::<u32>().unwrap();
    (a,b,x, &c[idx + 1..c.len()])
}
fn main() {
    let text = fs::read_to_string("./q.txt").expect("Unable to read file");
    let mut lines = text.split("\r\n\r\n");
    let mut parts: Vec<Vec<u32>> = Vec::new();
    let mut workflow:HashMap<String, Vec<(usize, u8, u32, &str)>> = HashMap::new();


    let f = lines.next().unwrap();
    for line in f.split("\r\n") {
        let idx: usize = line.find('{').unwrap();
        let k = String::from(&line[0..idx]);
        let mut v: Vec<(usize, u8, u32, &str)> = Vec::new();
        for cond in line[idx + 1..line.len() - 1].split(",") {
            v.push(get_fn(cond));
        }

        workflow.insert(k, v);
    }


    let s = lines.next().unwrap();

    for line in s.split("\r\n") {
        let mut p = Vec::with_capacity(4);
        let x = line[1..line.len() - 1].split(",");
        for z in x {
            //p.push(value)
            p.push(z[2..z.len()].parse::<u32>().unwrap());
        }
        parts.push(p);
    }

    //println!("{:?}", parts);
    //println!("{:?}", workflow);
    d19(workflow, parts);
}
