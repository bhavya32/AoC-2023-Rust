#![allow(non_snake_case)]
use std::collections::{VecDeque, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;


fn get_start(m:&Vec<Vec<char>>) -> (usize, usize) {
    let mut si = 0;
    let mut sj = 0;
    for i in 0..m.len(){
        for j in 0..m[i].len() {
            if m[i][j] == 'S' {
                si = i;
                sj = j;
                break
            }
        }
    }
    (si, sj)
}

fn get_neighbours(i:usize, j:usize, maxI:usize, maxJ:usize, m:&Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut t:Vec<(usize, usize)> = vec![];

    if m[i][j] == '|' {}

    if i != 0 && (m[i][j] == '|' || m[i][j] == 'J' || m[i][j] == 'L' || m[i][j] == 'S'){
        if m[i-1][j] == '|' || m[i-1][j] == 'F' || m[i-1][j] == '7' || m[i-1][j] == 'S' {
        t.push((i-1, j));}
    }
    if i !=  maxI-1  && (m[i][j] == '|' || m[i][j] == 'F' || m[i][j] == '7'  || m[i][j] == 'S'){
        if m[i+1][j] == '|' || m[i+1][j] == 'J' || m[i+1][j] == 'L' || m[i+1][j] == 'S' {
        t.push((i+1, j));}
    }
    if j != 0  && (m[i][j] == '-' || m[i][j] == 'J' || m[i][j] == '7' || m[i][j] == 'S'){
        if m[i][j-1] == '-' || m[i][j-1] == 'F' || m[i][j-1] == 'L' || m[i][j-1] == 'S' {
        t.push((i, j-1));
        }
    }
    if j != maxJ-1  && (m[i][j] == '-' || m[i][j] == 'F' || m[i][j] == 'L' || m[i][j] == 'S'){
        if m[i][j+1] == '-' || m[i][j+1] == 'J' || m[i][j+1] == '7' || m[i][j+1] == 'S'{
        t.push((i, j+1));}
    }
    t
}


fn count_tiles(mut v:Vec<Vec<char>>, vor:Vec<Vec<char>>) {
    //first find and mark all within vertical boundaries
    //for line in &mut v {
    
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            
            if v[i][j] == 'O'{
                v[i][j] = vor[i][j]
            } 
            else {
                v[i][j] = '.';
            }

        }
    }
    
    for x in &v {
        println!("{}", x.into_iter().collect::<String>());
    }
    let mut opps:HashMap<char, char> = HashMap::new();
    opps.insert('-', '-');
    opps.insert('L', 'J');
    opps.insert('-', '-');
    opps.insert('-', '-');
    opps.insert('-', '-');
    for i in 0..v.len() {
        let mut start = false;
        let mut start_char= 'A';
        for j in 0..v[i].len() {
            
            if v[i][j] == 'O' || v[i][j] == 'S' {
                if ['L', 'J', '|', '7', 'F'].contains(&vor[i][j]) {
                    if start {
                        if (start_char == '-' && vor[i][j] != '-') || (start_char == '-' && vor[i][j] == '-') ||  {
                            continue;
                        } 
                        start =false;
                    } else {
                        start = true;
                        start_char = vor[i][j];
                    }
                }
            }
            else if start {
                v[i][j] = 'X';
                start_char = vor[i][j];
            }

        }
    }
    let mut r = 0;
    for j in 0..v[0].len() {
        let mut start = false;
        let mut start_char = 'A';
        for i in 0..v.len() {
            if v[i][j] == 'O' || v[i][j] == 'S' {
                if ['L', 'J', '-', '7', 'F'].contains(&vor[i][j]) {
                    if start {
                        if start_char == '-' && vor[i][j] != '-' {
                            continue;
                        } 
                        start =false;
                    } else {
                        start = true;
                        start_char = vor[i][j];
                    }
                }
            } 
            else if start && v[i][j] == 'X'{
                println!("coutning {}-{}", i, j);
                r += 1;
            }
        }
    }
    println!("final - {}", r);
    
}

fn runBfs(mut m:Vec<Vec<char>>)-> u32 {
    //find S
    let mcopy = m.to_vec();
    let (si, sj) = get_start(&m);
    let mut q:VecDeque<(usize, usize, u32)> = VecDeque::new();
    let maxI = m.len();
    let maxJ = m[0].len();

    //run bfs from every neighbour, and check which reaches s
    for (xi, xj) in get_neighbours(si, sj, maxI, maxJ, &m) {
        q.push_back((xi, xj, 0));
        while q.len()>0 {
            let (ti, tj, dist) = q.pop_front().unwrap();
            //if dist > max_dist {max_dist = dist;}
            for (i, j) in get_neighbours(ti, tj, maxI, maxJ, &m) {
                if i == si && j == sj {
                    if ti!= xi && tj!=xj {
                        m[ti][tj] = 'O';
                        count_tiles(m, mcopy);
                    return (dist + 2)/2;}
                    else {
                        continue;
                    }
                }
                if m[i][j] == '.' || m[i][j] == 'O' {continue;}
                q.push_back((i,j,dist + 1));
            }
            m[ti][tj] = 'O';
        } 
    }

    return 0;
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
    let r = runBfs(matrix);
    //dbg!(&matrix);
    println!("{}", r);
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