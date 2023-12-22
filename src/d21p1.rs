#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::{VecDeque, HashSet};
use std::time::Instant;
use std::{fs, collections::HashMap};

use num::Integer;
//use std::time::Instant;

fn pr(matrix: & Vec<Vec<char>>) {
    for x in matrix {
        println!("{}", x.into_iter().collect::<String>());
    }
}


fn bfs(m:Vec<Vec<char>>, (si, sj):(usize, usize)) {
    let maxrow = m.len() as i32 - 1;
    let maxcol = m[0].len() as i32 -1;
    let max_dist = 64;
    let nb = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    let mut queue:VecDeque<(i32,i32, usize)> = VecDeque::new();
    let mut distmap:HashMap<(i32, i32), usize> = HashMap::new();
    queue.push_back((si as i32, sj as i32, 0));
    while let Some((i, j, dist)) = queue.pop_front() {
        if dist == max_dist {break;}
        for (dx, dy) in nb {
            let (ni, nj) = (i + dx, j + dy);
            if ni < 0 || nj < 0 || ni > maxrow || nj > maxcol || m[ni as usize][nj as usize] == '#' {continue;}
            if !distmap.contains_key(&(ni, nj)) {
                distmap.insert((ni, nj), dist + 1);
                queue.push_back((ni, nj, dist + 1));
            }
        }
    }
    println!("{}", distmap.values().filter(|d| d.is_even()).count());
    
    //println!("{}", queue.len() + 1);
}

fn main() {
    let text = fs::read_to_string("./q.txt").expect("Unable to read file");
    let lines = text.split("\r\n");
    let mut matrix:Vec<Vec<char>> = Vec::new();
    let (mut si, mut sj) = (0, 0);
    for (i, line) in lines.enumerate() {
        let mut t: Vec<char> = Vec::new();
        for (j, mut c) in line.chars().enumerate() {
            if c == 'S' {
                c = '.';
                (si, sj) = (i, j);
            }
            t.push(c);
        }
        
        matrix.push(t);
    }

    //bfs(matrix, (si, sj));
    //println!("{:?}", djinplen);
    
    let now = Instant::now();
    //let res = d20(modules, djinplen);
    bfs(matrix, (si, sj));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    //println!("{}", res);
}
