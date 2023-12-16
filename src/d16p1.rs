#![allow(non_snake_case)]
use std::collections::HashSet;
use std::collections::VecDeque;
//use std::vec;
use std::fs;
use crate::Dir::*;
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    r:usize,
    c:usize
}

enum Dir {
    R,
    L,
    U,
    D
}


fn get_n(m:&Vec<Vec<char>>, s:Point, dir:Dir) -> Option<Point> {
    match dir {
        R => {
            for j in s.c + 1..m[0].len() {
                return Some(Point { r: s.r, c: j });
                //if m[s.r][j] != '.' {return Some(Point { r: s.r, c: j })}
            }
            return None;
        },
        L => {
            for j in (0..s.c).rev() {
                return Some(Point { r: s.r, c: j });
                //if m[s.r][j] != '.' {return Some(Point { r: s.r, c: j })}
            }
            return None;
        },
        D => {
            for i in s.r + 1..m.len() {
                return Some(Point { r: i, c: s.c });
                //if m[i][s.c] != '.' {return Some(Point { r: i, c: s.c })}
            }
            return None;
        }
        U => {
            for i in (0..s.r).rev() {
                return Some(Point { r: i, c: s.c });
                //if m[i][s.c] != '.' {return Some(Point { r: i, c: s.c })}
            }
            return None;
        }
    }
}

fn get_all_n(m:&Vec<Vec<char>>, se:(Point, Point)) -> Vec<Point>{

    let (to, from) = se;
    
    let dir;
    if to == from && to.r == 0 && to.c == 0 {
        dir = R;
        if m[0][0] == '.' {
            let mut poss = Vec::new();
            poss.push(Point {r:0, c:1});
            return poss;
        }
    }else if to.c > from.c {
        dir = R;
    } else if to.c < from.c {
        dir = L;
    } else if to.r > from.r {
        dir = D;
    } else {
        dir = U;
    }
    let mut poss = Vec::new();
    if m[to.r][to.c] == '.' {
        let xrr : i32 = (2*(to.r) as i32) - from.r as i32;
        let xrc:i32 = (2*(to.c) as i32) - from.c as i32;
        
        if xrr >= 0 && xrr < m.len() as i32 && xrc >= 0 && xrc < m[0].len() as i32 {
            poss.push(Point {r:xrr as usize, c:xrc as usize});
        }
        return poss;
    }
    fn add_n(t:Option<Point>, v:&mut Vec<Point>) {
        match t {
            Some(x) => v.push(x),
            _ => {}
        }
    }
    let c = m[to.r][to.c];
    if c == '\\' {
        match dir {
            D => add_n(get_n(m, to, R), &mut poss),
            R => add_n(get_n(m, to, D), &mut poss),
            L => add_n(get_n(m, to, U), &mut poss),
            U => add_n(get_n(m, to, L), &mut poss)
        }
    } else if c == '/' {
        match dir {
            D => add_n(get_n(m, to, L), &mut poss),
            R => add_n(get_n(m, to, U), &mut poss),
            L => add_n(get_n(m, to, D), &mut poss),
            U => add_n(get_n(m, to, R), &mut poss)
        }
    } else if c == '|' {
        match dir {
            D => add_n(get_n(m, to, D), &mut poss),
            R| L => {add_n(get_n(m, to, U), &mut poss); add_n(get_n(m, to, D), &mut poss)},
            U => add_n(get_n(m, to, U), &mut poss)
        }
    } else {
        match dir {
            R => add_n(get_n(m, to, R), &mut poss),
            D| U => {add_n(get_n(m, to, L), &mut poss); add_n(get_n(m, to, R), &mut poss)},
            L => add_n(get_n(m, to, L), &mut poss)
        }
    }
    poss
}

fn bfs(mut m:Vec<Vec<char>>) {
    let mut visited:HashSet<(Point, Point)> = HashSet::new();
    let mut q:VecDeque<(Point, Point)> = VecDeque::new();

    q.push_back((Point{r:0,c:0}, Point{r:0,c:0}));
    visited.insert((Point{r:0,c:0}, Point{r:0,c:0}));
    while q.len() > 0 {
        let c = q.pop_front().unwrap();
        for n in get_all_n(&m, c) {
            if visited.contains(&(n, c.0)) {continue;}
            visited.insert((n, c.0));
            q.push_back((n, c.0));
        }
    }
    let mut total = 0;
    for ( i, _j) in visited.into_iter() {
        if m[i.r][i.c] != '#' {
            m[i.r][i.c] = '#';
            total += 1;
        }
        
        
    }
    println!("{}", total);
    /*for x in m {
        println!("{}", x.into_iter().collect::<String>());
    }*/
}

fn main() {
    let text = fs::read_to_string("./q.txt").expect("Unable to read file");
    let lines = text.split("\r\n");
    let mut matrix:Vec<Vec<char>> = Vec::new();
    for line in lines {
        //println!("{}|", line);
        let t: Vec<char> = line.chars().collect();
        matrix.push(t);
    }
    //println!("{:?}", matrix);
    bfs(matrix);
}
