use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, Display};

use std::cmp::min;
fn handle_n(mut v:Vec<c>) {
    let maxl = v.len();
    for idx in 0..maxl {
    //for (idx, i) in v.iter_mut().enumerate(){
        let i = &v[idx];
        
        let this_win_count: u32 = handle(&i.str);
        for t in idx+1..min(maxl, idx+1+this_win_count as usize) {
            v[t].count += v[idx].count;
        }
    }

    let mut total = 0;
    for card in &v {
        total += card.count;
    }
    //println!("{:#?}", v);
    println!("{}", total);
}

fn handle(x: &String)-> u32 {
    let byte_ar = x.as_bytes();
    let mut i = 0;
    let mut win_count = 0;
    while(byte_ar[i] as char != ':'){
        i += 1;
    }
    i+=2;

    let t = String::from(&x[i..x.len()]);
    //println!("{}", t);
    let mut iter = t.split(" | ");
    let winning_set:HashSet<&str> = HashSet::from_iter(iter.next().unwrap().split(" "));
    //dbg!(&winning_set);
    for num in iter.next().unwrap().split(" ") {
        if num == "" {continue;}
        if winning_set.contains(num) {
            win_count += 1;
        }
    }
    
    return win_count;

}

#[derive(Debug)]
struct c {
    str: String,
    count:u32
}

impl std::fmt::Display for c {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result{
        writeln!(f,"{}", self.count);
        Ok(())
    }
}

fn main() {
    //let mut sum = 0;
    let mut v:Vec<c> = vec![];
    if let Ok(lines) = read_lines("./q.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(k) = line {
                //println!("{}", k);
                //sum += handle(&k);
                v.push(c {str:k, count:1});
                
                //sum+=handle(&k);
            }
        }

    }
    handle_n(v);
    //println!("{}", sum);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}