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
    
    
    let mut opps:HashMap<char, [char; 2]> = HashMap::new();
    let mut res = 0;
    opps.insert('-', ['W', 'E']);
    opps.insert('L', ['N', 'E']);
    opps.insert('|', ['N', 'S']);
    opps.insert('J', ['N', 'W']);
    opps.insert('7', ['W', 'S']);
    opps.insert('F', ['E', 'S']);
    for i in 0..v.len() {
        let mut start = false;
        for j in 0..v[i].len() {
            if v[i][j] == '.' {
                if start {
                    res += 1;
                }
                continue;
            }
            if opps.get(&v[i][j]).unwrap().contains(&'N') {
                if start {
                    start =false;
                } else {
                    start = true;
                }
            }
            

        }
    }

    println!("inner tiles -  {}", res);
    
}

fn get_start_char(mut xi:usize, mut xj:usize,mut ti:usize,mut tj:usize, si:usize, sj:usize) -> char {

    if xi == ti {
        return '-';
    }
    if xj == tj {
        return '|';
    }

    if xi > ti {
        (xi, xj, ti, tj) = (ti, tj, xi, xj);
    }

    if si != 0 && (si - 1, sj) == (xi, xj) {
        //start is north
        //end can be west or east

        if xj > tj {
            return 'J';
        }else {
            return 'L';
        }
    } else {
        //end is south
        //start can be west or east
        if xj > tj {
            return 'F';
        }
        return '7';
    }
}
fn runBfs(mut m:Vec<Vec<char>>)-> u32 {
    //find S
    let mut mcopy = m.to_vec();
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
                        mcopy[si][sj] = get_start_char(xi, xj, ti,tj,si,sj);
                        m[si][sj] = 'O';
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