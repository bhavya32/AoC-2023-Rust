#![allow(non_snake_case)]
use std::vec;
use std::fs;
fn hash(s:&str) -> usize {
    let mut hsh = 0;
    for chr in s.chars() {
        hsh = ((hsh + chr as usize)*17) % 256;
    }
    hsh
}

fn main() {
    let text = fs::read_to_string("./q.txt").expect("Unable to read file");
    let lines = text.split(",");
    let mut boxes:Vec<Vec<(&str, u8)>> = vec![Vec::new(); 256];
    for line in lines {
        let v:Vec<char> = line.chars().collect();
        let mut sloc = 0;
        for i in 0..v.len() {
            if v[i] == '=' || v[i] == '-' {sloc = i; break;}
        }
        let y = &line[0..sloc];
        let hsh = hash(y);
        let lensloc = boxes[hsh].iter().position(|f| f.0 == y);
        if v[sloc] == '-' {
            match lensloc {
                Some(t) => { boxes[hsh].remove(t); },
                None => {}
            }

            continue;
        }
        let lens_p = v[sloc + 1].to_digit(10).unwrap() as u8;
        match lensloc {
            Some(t) => { boxes[hsh][t].1 =lens_p; },
            None => {boxes[hsh].push((y,lens_p));}
        }
    }
    let mut power = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            power += ((i+1)*(j+1)*(boxes[i][j].1 as usize)) as u64;
        }
    }
    println!("power is {}", power);
}
