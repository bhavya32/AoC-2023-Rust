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

fn find_diff(x:&String, y:&String) -> u8 {
    let t = x.as_bytes();
    let u = y.as_bytes();
    let mut found_diff = false;
    for i in 0..t.len() {
        if t[i] == u[i] {continue};
        if found_diff {return 2;}
        found_diff = true;
    }
    if found_diff {
        return 1;
    }
    0
}

fn check_palindrome(v:&Vec<String>, mut l:usize, mut found_diff:bool) -> bool {
    let mut r = l+1;
    while l > 0 && r < v.len() {
        //if v[l] != v[r] {return false;}
        let x = find_diff(&v[l], &v[r]);
        //println!("x = {} at l={}, r = {}", x, l,r);
        if x == 2 {return false;}
        if x == 1 {
            if found_diff {return false;}
            found_diff = true;
        }
        r += 1;
        l -= 1;
    }
    if l == 0 && r<v.len() {
        //if v[l] != v[r] {return false;}
        let x = find_diff(&v[l], &v[r]);
        //println!("x = {} at l={}, r = {}", x, l,r);
        if x == 2 {return false;}
        if x == 1 {
            if found_diff {return false;}
            found_diff = true;
        }
    }
    if found_diff {return true;}
    return false;
}

fn find_ref_point(v:Vec<String>) ->usize{
    for i in 0..v.len() - 1 {
        let t = find_diff(&v[i], &v[i+1]);
        if t == 2 {continue};
        if check_palindrome(&v, i, false){
            return i+1;
        }
        //println!("t = {} at i={}", t, i);
    }
    return 0;
}

fn main() {
    //let mut sum = 0;
    let texto = get_text();
    let matrices = texto.split("\n\n");
    let mut total = 0;
    for text in matrices {
        if text == "" {continue;}
        let lines = text.split("\n");
        let mut matrix:Vec<Vec<char>> = Vec::new();
        let mut rowVec:Vec<String> = Vec::new();
        let mut colVec:Vec<String> = Vec::new();
        for line in lines {
            if line == "" {continue;}
            let t: Vec<char> = line.chars().collect();
            matrix.push(t);
            rowVec.push(String::from(line));
        }

        for col_i in 0..matrix[0].len() {
            let mut s = String::from("");

            for rows in 0..matrix.len() {
                s.push(matrix[rows][col_i]);
            }
            colVec.push(s);
        }

        let mut l = find_ref_point(rowVec) as u64;
        let r = find_ref_point(colVec) as u64;
        //println!("l - {}, r - {}", l, r);
        if l != 0 && r != 0 {
            l = if l > r {r} else {l*100};
            //println!("l and r not 0 - {} {}", l, r);
        }
        else if l == 0 {
            //l = find_ref_point(colVec) as u64;
            l = r;
        }
        else {
            l *= 100;
        }
        total += l;
    }
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