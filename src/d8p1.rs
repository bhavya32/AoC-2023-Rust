use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, Display};
use std::vec;
use std::cmp::{min, Ordering, Eq};
use std::fmt;

struct iter {
    seq:Vec<usize>,
    i:usize
}

impl iter {
    fn new(s: &str)-> Self {
        let mut seq:Vec<usize> = Vec::new();
        for char in s.chars() {
            if char == 'L' {
                seq.push(0);
            } else {
                seq.push(1);
            }
        }
        return iter {seq, i:0};
    }
    fn get_next(&mut self) -> usize {
        if self.i >= self.seq.len() {
            self.i -= self.seq.len();
        }
        let t =  self.seq[self.i];
        self.i += 1;
        return t;
    }
}
fn parse_line(s:&str)-> (&str, Vec<&str>) {
    let v:Vec<&str> = vec![&s[7..10], &s[12..15]];
    let k = &s[0..3];

    (k, v)
}
fn create_map(s:&str)-> HashMap<&str, Vec<&str>>{
    let lines = s.split("\n");
    let mut m:HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines {
        if line == "" {continue;}
        let (key, val) = parse_line(line);
        m.insert(key, val);
        //println!("{}/", line);
    }
    //dbg!(&m);
    return m;
}
fn main() {
    //let mut sum = 0;
    let text = get_text();
    let mut txt = text.split("\n\n");
    let mut seq = iter::new(txt.next().unwrap());

    let map = create_map(txt.next().unwrap());
    let mut steps = 0;
    let mut curr_pos = "AAA";
    loop {
        steps += 1;
        let npos = map.get(curr_pos).unwrap()[seq.get_next()];
        if npos == "ZZZ" {break;}
        curr_pos = npos;
        //println!("{}", npos);
    }
    println!("{}", steps);
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