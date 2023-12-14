#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn pr(matrix: & Vec<Vec<char>>) {
    for x in matrix {
        println!("{}", x.into_iter().collect::<String>());
    }
}

fn get_total(m:Vec<Vec<char>>) -> u64{
    let mut t = 0;
    let row_count = m.len();
    let col_count = m[0].len();

    for j in 0..col_count {
        let mut inst = 0;
        for i in 0..row_count {
            if m[i][j] == '.' {continue;}
            if m[i][j] == 'O' {
                t += (row_count - inst) as u64;
                inst += 1;
            } else {
                inst = i + 1;
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
    let total = get_total(matrix);
    println!("{}", total);
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