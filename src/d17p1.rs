#![allow(non_snake_case)]
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    r:i32,
    c:i32,
    cost:u32,
    streak:u32,
    l:(i32, i32)
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.l.cmp(&other.l))
            .then_with(|| other.streak.cmp(&self.streak))
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//find neighbours of the point
fn nbs(p:Point) -> Vec<Point>{
    let mut r:Vec<Point> = Vec::with_capacity(3);
    let row_diff = p.r - p.l.0;
    let col_diff = p.c - p.l.1;

    //left and right points
    if row_diff != 0 {
        r.push(Point {r:p.r, c:p.c + 1, cost:0, streak:0, l:(p.r, p.c)});
        r.push(Point {r:p.r, c:p.c - 1, cost:0, streak:0, l:(p.r, p.c)});
    } else {
        r.push(Point {r:p.r + 1, c:p.c, cost:0, streak:0, l:(p.r, p.c)});
        r.push(Point {r:p.r - 1, c:p.c, cost:0, streak:0, l:(p.r, p.c)});
    }

    //option to move straight if its not the 3rd block 
    if p.streak < 2 {
        r.push(Point {r:p.r + row_diff, c:p.c + col_diff, cost:0, streak:p.streak + 1, l:(p.r, p.c)});
    }

    r
}

fn djisktra(x:Vec<Vec<u32>>) {
    // defining points to reach
    let goalr = x.len() as i32 - 1;
    let goalc = x[0].len() as i32 - 1;
    
    let inf = 9*(x.len()*x[0].len()) as u32;   //max poss dist
    let mut dist = vec![vec![inf; x[0].len()]; x.len()]; //setting distance to each point as infinity
    let mut heap:BinaryHeap<Point> = BinaryHeap::new(); //max-heap but with reveres cmp, so min-heap

    //putting both starting neighbours in the heap
    dist[0][0] = 0; 
    dist[0][1] = x[0][1];
    dist[1][0] = x[1][0]; 
    heap.push(Point { r: 0, c: 1, cost: x[0][1], streak: 0,l:(0, 0)});
    heap.push(Point { r: 1, c: 0, cost: x[1][0], streak: 0, l:(0, 0)});

    //creating a full parent map just to get the full path for debugging
    let mut distn:HashMap<((i32, i32), (i32, i32), u32), u32> = HashMap::new();
    while let Some(p) = heap.pop() {

        //if p.cost > dist at that point, it should mean that the vertex is already visited from a shorter path
        //if p.cost > dist[p.r as usize][p.c as usize] {continue;}
        //burnt.insert(((p.r, p.c), p.l, p.streak));
        //parent.insert((p.r as usize, p.c as usize), (p.l.0 as usize, p.l.1 as usize));
        if p.r == goalr && p.c == goalc {
            println!("ans - {}", p.cost); 
            //print( viz,  parent,goalr, goalc);
            //pd(dist);
            return;
        }
        //visited.insert((r,c));
        /*if p.r == 1 && p.c == 0 {
            println!("{:?}", nbs(p));
        }*/
        /*if p.r == 1 && p.c == 2 {
            println!("parent of 1,2 are {:?}", p.l);
            println!("nbs of 1,2 are {:?}", nbs(p));
        }*/

        //get neighbours
        for mut n in nbs(p) {
            //check if point is valid and set its cost or weight till that path
            if n.r < 0 || n.c < 0 || n.r >= x.len() as i32 || n.c >= x[0].len() as i32 {continue;}
            n.cost = p.cost + x[n.r as usize][n.c as usize];
            

            let x = distn.get(&((n.r, n.c), n.l, n.streak));
            if x.is_none() || n.cost < *x.unwrap() {
                dist[n.r as usize][n.c as usize] = n.cost;
                distn.insert(((n.r, n.c), n.l, n.streak), n.cost);
                heap.push(n);
            }
            /*
            if !burnt.contains(&((n.r, n.c), n.l, n.streak)) {
                heap.push(n);
            } */
        }
        
    }
}

fn main() {
    let text = fs::read_to_string("./q.txt").expect("Unable to read file");
    let lines = text.split("\r\n");
    let mut matrix:Vec<Vec<u32>> = Vec::new();
    for line in lines {
        //println!("{}|", line);
        let t: Vec<u32> = line.chars().map(|f| f.to_digit(10).unwrap()).collect();
        matrix.push(t);
    }
    djisktra(matrix);
}
