#![allow(non_snake_case)]
#![allow(dead_code)]
use std::collections::VecDeque;
use std::{fs, collections::HashMap};
use num::Integer;

fn bfs(m:Vec<Vec<char>>, (si, sj):(usize, usize)) {
    let maxrow = m.len() as i32 - 1;
    let maxcol = m[0].len() as i32 -1;
    let nb = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    let mut queue:VecDeque<(i32,i32, usize)> = VecDeque::new();
    let mut distmap:HashMap<(i32, i32), usize> = HashMap::new();

    queue.push_back((si as i32, sj as i32, 0));
    distmap.insert((si as i32, sj as i32), 0);

    while let Some((i, j, dist)) = queue.pop_front() {
        for (dx, dy) in nb {
            let (ni, nj) = (i + dx, j + dy);
            if ni < 0 || nj < 0 || ni > maxrow || nj > maxcol || m[ni as usize][nj as usize] == '#' {continue;}
            
            if !distmap.contains_key(&(ni, nj)) {
                distmap.insert((ni, nj), dist + 1);
                queue.push_back((ni, nj, dist + 1));
            }
        }
    }

    println!("{}", distmap.values().filter(|d| **d <= 64 && d.is_even()).count());
    let even_corners = distmap.values().filter(|v| **v % 2 == 0 && **v > 65).count();
    let odd_corners = distmap.values().filter(|v| **v % 2 == 1 && **v > 65).count();
    let even_full = distmap.values().filter(|v| **v % 2 == 0).count();
    let odd_full = distmap.values().filter(|v| **v % 2 == 1).count();

    // This is 202300 but im writing it out here to show the process
    let n = 202300;

    let p2 = ((n+1)*(n*1)) * odd_full + (n*n) * even_full - (n+1) * odd_corners + n * even_corners;
    println!("{}", p2);
    //ans is 612941134797232
    println!("should be - 612941134797232");
}

fn main() {
    let text = fs::read_to_string("./src/d21input.txt").expect("Unable to read file");
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

    bfs(matrix, (si, sj));
}
