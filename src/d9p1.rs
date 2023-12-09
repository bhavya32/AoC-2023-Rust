use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path};
use std::vec;
use std::cmp::{min, Ordering, Eq};

fn diff_vec(v:&Vec<i64>) -> Vec<i64> {
    let mut r:Vec<i64> = vec![];

    for i in 0..v.len() - 1 {
        r.push(v[i+1] - v[i]);
    } 

    r
}

fn check_zeroes(v:&Vec<i64>)-> bool {
    for x in v {
        if *x != 0 {
            return false;
        }
    }
    true
}

fn get_next(t:Vec<i64>) -> i64 {
    if check_zeroes(&t) {return 0;}
    let mut d = diff_vec(&t);

    return t[t.len() - 1] + get_next(d);
}
fn main() {
    //let mut sum = 0;
    let text = get_text();
    let lines = text.split("\n");
    let mut sum = 0;

    for line in lines {
        //println!("{}/", line);
        let t: Vec<i64> = line.split(" ").filter(|f| *f!= "").map(|f| f.parse::<i64>().unwrap()).collect();
        sum += get_next(t);
    }

    println!("{}", sum);
}


fn get_text() -> String {

    let mut s = String::from("");
    if let Ok(lines) = read_lines("./q.txt") {
        for (_i, line) in lines.enumerate() {
            if let Ok(k) = line {
                s+=&(k + "\n");
            }
        }

    }
    s

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}