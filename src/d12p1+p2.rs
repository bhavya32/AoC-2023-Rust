#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::{VecDeque, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;


fn get_perms(q:&mut Vec<char>, u:&mut Vec<usize>, li:usize, lj:usize, hmap:&mut HashMap<(usize, usize), u64>) -> u64 {
    //let x = q.iter().position(|&r| r=='?' || r=='#');
    
    if hmap.contains_key(&(li, lj)) {
        return *hmap.get(&(li, lj)).unwrap();
    }
    let mut x = None;
    let mut y = None;
    for i in li..q.len() {
        if q[i] == '?' || q[i] == '#' {
            x = Some(i);
            break;
        }
    }
    for i in lj..u.len() {
        if u[i] != 0 {
            y = Some(i);
            break;
        }
    }
    //return if no
    if y.is_none() && !q[li..].contains(&'#') {
        return 1;
    }
    if x.is_none() && y.is_none(){
        return 1;
    }
    if y.is_none() || x.is_none(){
        //println!("returning 0 here");
        return 0;
    }
    let i = x.unwrap();
    let j = y.unwrap();


    let mut il = i;
    let mut containsHash = false;
    while il < q.len() && (q[il] == '?' || q[il] == '#') {
        if q[il] == '#' {containsHash = true;}
        il += 1;
    }

    
    //now make the qmark a . or a #
    //check if the block starting with ? is of the size in u[j]

    // if the block size is equal, make all ?s #s. or all ?s dots. 
    //if it has a #, it has to be all #s
    if il - i == u[j] {
        //println!("found a block of equal size starting at {} to {}", i, il);
        let permx = get_perms(q, u, il, lj+1, hmap);
        //pr(&q[il..], &u[lj+1..], permx);
        hmap.insert((il, lj+1), permx);
        if containsHash {return permx};
        let permx2 = get_perms(q, u, il, lj, hmap);
        //pr(&q[il..], &u[lj..], permx2 + permx);
        hmap.insert((il, lj), permx2);
        return permx + permx2;
    }


    //if the block size is smaller, make all qs dots continue
    //and if the block contains a #, we return this combo isn't possible, so 0
    if il - i < u[j] {
        if containsHash {return 0;}
        for a in i..il {
            if q[a] == '#' {return 0;}
        }
        let permx23 = get_perms(q, u, il, lj, hmap);
        //pr(&q[il..], &u[lj..], permx23);
        hmap.insert((il, lj), permx23);
        return permx23;
    }
    // if the block size is greater, we make 2 choices, count the block till u[j] as a block make it either . or #
    // if first el is #
    //println!("here il -");
    let mut permx = 0;
    if q[i] != '#' {
        permx = get_perms(q, u, i+1, lj, hmap);
        //pr(&q[i+1..], &u[lj..], permx);
        hmap.insert((i + 1, lj), permx);
    }
    if ((i + u[j]) as usize) < q.len() {
        if q[(i + u[j]) as usize] == '#' {return permx;}
    } 
    let permy = get_perms(q, u, i+u[j]+1, lj+1, hmap);
    //pr(&q[i+u[j] + 1..], &u[lj+1..], permy);
    hmap.insert((i+u[j] +1, lj + 1), permy);
    return permy + permx;
}


fn multv (v:Vec<char>) -> Vec<char> {
    let mut mv:Vec<char> = Vec::new();

    for _ in 0..4 {
        mv.append(&mut (v.to_vec()));
        mv.push('?');
    }
    mv.append(&mut (v.to_vec()));
    mv
}

fn multu(v:Vec<usize>) -> Vec<usize> {
    [&v[..], &v[..], &v[..], &v[..], &v[..]].concat()
}
fn main() {
    //let mut sum = 0;
    let text = get_text();
    let lines = text.split("\n");
    let mut sum = 0;
    for line in lines {
        if line == "" {continue;}
        let mut t1 = line.split(" ");
        let x1 = t1.next().unwrap();
        let x2 = t1.next().unwrap();
        let mut t: Vec<char> = x1.chars().collect();
        let mut u:Vec<usize> = x2.split(",").map(|f| f.parse::<usize>().unwrap()).collect();
        {
            //comment these 2 for part 1
            t = multv(t);
            u  = multu(u);
        }
        let mut hmap = HashMap::new();
        sum += get_perms(&mut t,&mut u, 0, 0, &mut hmap);
    }
    println!("{}", sum);
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