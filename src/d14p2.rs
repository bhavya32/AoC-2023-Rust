#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;


fn north(m:&mut Vec<Vec<char>>) {
    let row_count = m.len();
    let col_count = m[0].len();

    for j in 0..col_count {
        let mut inst = 0;
        for i in 0..row_count {
            if m[i][j] == '.' {continue;}
            if m[i][j] == 'O' {
                m[i][j] = '.';
                m[inst][j] = 'O';
                inst += 1;
            } else {
                inst = i + 1;
            }
        }
    }
}
fn west(m:&mut Vec<Vec<char>>){
    let row_count = m.len();
    let col_count = m[0].len();

    for i in 0..row_count {
        let mut inst = 0;
        for j in 0..col_count {
            if m[i][j] == '.' {continue;}
            if m[i][j] == 'O' {
                //println!("Shifting from {} to {}", i, inst);
                m[i][j] = '.';
                m[i][inst] = 'O';
                inst += 1;
            } else {
                inst = j + 1;
            }
        }
    }
}

fn south(m:&mut Vec<Vec<char>>) {
    let row_count = m.len();
    let col_count = m[0].len();
    for j in 0..col_count {
        let mut inst:i32 = row_count as i32- 1;
        for i in (0..row_count).rev() {
            if m[i][j] == '.' {continue;}
            if m[i][j] == 'O' {
                m[i][j] = '.';
                m[inst as usize][j] = 'O';
                inst -= 1;
            } else {
                inst = i as i32 - 1;
            }
        }
    }
}

fn east(m:&mut Vec<Vec<char>>)->u64{
    let row_count = m.len();
    let col_count = m[0].len();
    let mut t = 0;
    for i in 0..row_count {
        let mut inst:i32 = col_count as i32 - 1;
        for j in (0..col_count).rev() {
            if m[i][j] == '.' {continue;}
            if m[i][j] == 'O' {
                m[i][j] = '.';
                m[i][inst as usize] = 'O';
                t += (row_count - i) as u64;
                inst -= 1;
            } else {
                inst = j as i32 - 1;
            }
        }
    }
    t
}

fn main() {
    //let mut sum = 0;
    let text = get_text();
    let lines = text.split("\n");
    
    let mut matrix:Vec<Vec<char>> = Vec::new();
    for line in lines {
        if line == "" {continue;}
        let t: Vec<char> = line.chars().collect();
        matrix.push(t);
    }
    let mut s = 0;
    let mut hmap:HashMap<Vec<Vec<char>>, i32> = HashMap::new();
    let mut i = 0;
    let max = 1000000001;
    let mut skipped = false;
    while i <  max{
        north(&mut matrix);
        west(&mut matrix);
        south(&mut matrix);
        s = east(&mut matrix);
        
        if !skipped &&  hmap.contains_key(&matrix) {
            let last_idx = hmap.get(&matrix).unwrap();
            let diff = i+1 - last_idx;
            i = max - ((max - (i+1)) % diff);
            skipped = true;
        } else {
            hmap.insert(matrix.to_vec(), i+1);
        }
        i += 1;
    }
    println!("{}", s);
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