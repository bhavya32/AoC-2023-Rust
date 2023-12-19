#![allow(non_snake_case)]

use core::panic;
use std::{fs, collections::HashMap};
use std::time::Instant;
#[derive(Clone)]
struct Range {
    i:u32,
    j:u32
}

impl Range {
    fn count(&self) -> u64 {
        (self.j + 1 - self.i) as u64
    }
}

fn increase(p:Vec<Range>, c:&mut u64) {
    *c += p[0].count() * p[1].count() * p[2].count() * p[3].count()
}

fn blackbox((var, cond, val, next): &(usize, u8, u32, &str), mut v: Vec<Range>,
    w:&HashMap<String, Vec<(usize, u8, u32, &str)>>, c:&mut u64) 
    -> Option<Vec<Range>>
{
    let r = &v[*var];
    if *cond == 0 {
        //check if range is lower than val
        if r.i > *val {return Some(v);}
        if r.j < *val {check_recur(w, v, next, c);return None;}
        let mut nrangev = v.clone();
        let lrange = Range {i:r.i, j: *val - 1};
        let rrange = Range {i:*val, j: r.j};
        nrangev[*var] = lrange;
        v[*var] = rrange;
        check_recur(w, nrangev, next, c);
        return Some(v);
    } else {
        //chec if range is greater than val;
        if r.i > *val {check_recur(w, v, next, c);return None;}
        if r.j < *val {return Some(v);}
        
        let mut nrangev = v.clone();
        let lrange = Range {i:r.i, j:*val};
        let rrange = Range {i:*val + 1, j: r.j};
        nrangev[*var] = lrange;
        v[*var] = rrange;
        check_recur(w, v, next, c);
        return Some(nrangev);
    }

}

fn check_recur(w:&HashMap<String, Vec<(usize, u8, u32, &str)>>,mut p:Vec<Range>, work:&str, c:&mut u64) {
    if work == "R" {return;}
    if work == "A" {
        increase(p, c);
        return;
    }
    let conds = w.get(work).unwrap();
    for cd in conds {
        if cd.0 == 4 {
            if cd.3 == "R" {return;}
            if cd.3 == "A" {
                increase(p, c);
                return;
            }
            check_recur(w, p, cd.3, c);
            return;
        }

        //check if the condition matches the whole range, if not, break the range into 2, 1 tht matches, 1 tht not;
        let x = blackbox(cd, p, w,c);
        match x {
            Some(t) => {p = t;},
            None => {return;}
        }
    }
    
    
}

fn d19(workflow:HashMap<String, Vec<(usize, u8, u32, &str)>>)->u64{
    
    let mut c = 0;
    let x = vec![Range {i:1, j:4000}; 4];
    check_recur(&workflow, x, "in", &mut c);
    c
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

    let now = Instant::now();
    let t = d19(workflow);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{t}");
}
