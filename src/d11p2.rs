#![allow(non_snake_case)]
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

fn is_row_empty(m: &mut Vec<Vec<char>>, i:usize) -> bool{
    for j in 0..m[i].len() {
        if m[i][j] != '.' {
            return false;
        }
    }
    return true;
}

fn is_col_empty(m: &mut Vec<Vec<char>>, j:usize) -> bool{
    for i in 0..m.len() {
        if m[i][j] != '.' {
            return false;
        }
    }

    return true;
}

fn expand(m: &mut Vec<Vec<char>>) {
    let mut rows:Vec<usize> = vec![];
    let mut cols:Vec<usize> = vec![];
    let mut ic = 0;
    while ic < m[0].len() {
        if is_col_empty(m, ic) {

            cols.push(ic);
        }
        ic += 1;
    }
    
    
    let mut i = 0;
    while i < m.len() {
        if is_row_empty(m, i) {
            rows.push(i);
        }
        i+=1;
    }

    findSum(m, rows, cols);

    
}

fn get_row_dist(mut x1:usize, mut x2:usize,rows: &Vec<usize>) -> usize {
    if x1 > x2 {
        (x2, x1) = (x1, x2);
    }
    let mut r = x2-x1;

    for row in rows {
        if *row > x1 && *row < x2 {
            r += 999999;
        }
    }
    r
}

fn get_col_dist(mut y1:usize, mut y2:usize, cols:&Vec<usize>) -> usize{
    if y1 > y2 {
        (y2, y1) = (y1, y2);
    }
    let mut r = y2-y1;

    for col in cols {
        if *col > y1 && *col < y2 {
            r += 999999;
        }
    }
    r
}

fn findSum(m: &Vec<Vec<char>>, rows:Vec<usize>, cols:Vec<usize>) {
    let mut loc: Vec<(usize, usize)> = vec![];
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '#' {
                loc.push((i,j));
            }
        }
    }

    let mut sum = 0;
    for i in 0..loc.len() {
        let (x1, y1) = loc[i];
        for j in i+1..loc.len() {
            let (x2, y2) = loc[j];
            sum += get_row_dist(x1, x2, &rows);
            sum += get_col_dist(y1, y2, &cols);
            //sum += x2.abs_diff(x1) + y2.abs_diff(y1);
        }
    }

    println!("sum - {}", sum);
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
    expand(&mut matrix);
    
    //pr(&matrix);
    //findSum(&matrix);
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