use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;
use std::cmp::min;
fn find_win_ways(time:u64, dist:u64) -> u64{
    let mut best_dist:i64 = -1;
    let mut win_idx = Option::None;
    for holdTime in 0..time {
        let new_dist:i64 = ((time - holdTime) * holdTime) as i64;
        if new_dist > dist as i64 && win_idx == Option::None{
            win_idx = Option::Some(holdTime);
        }
        if new_dist > best_dist {
            best_dist =new_dist;
        }
        else {
            //println!("win idx - {}", win_idx.unwrap());
            return time -  (win_idx.unwrap() - 1)*2  -1;
            //return (holdTime - win_idx.unwrap())*2;
        }
    }
    time
}
fn main() {
    //let mut sum = 0;
    let text = get_text();
    let mut txt = text.split("\n");
    let mut time:Vec<u64>= vec![];
    let mut distance:Vec<u64>= vec![];
    for num in txt.next().unwrap().split(" ").filter(|p| *p != "Time:" && *p!= "")
        .map(|p| p.parse::<u64>().unwrap()) {
            time.push(num);
    }
    for num in txt.next().unwrap().split(" ").filter(|p| *p != "Distance:" && *p!= "")
        .map(|p| p.parse::<u64>().unwrap()) {
            distance.push(num);
    }
    let mut r:u64 = 1;
    for i in 0..time.len() {
        //println!("{}", find_win_ways(time[i], distance[i]));    
        r*=find_win_ways(time[i], distance[i]);
    }
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