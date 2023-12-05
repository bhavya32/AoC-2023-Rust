use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn handle(x: &String)-> u32 {
    let mut y = 0;
    let mut ff = false;
    let mut ld = 0; 
    for c in x.chars() {
        if c.is_digit(10){
            if ff == false {
                y = c.to_digit(10).unwrap() * 10;
                ff = true;
            };
            ld = c.to_digit(10).unwrap();
        }
    }

    y += ld;
    //println!("{}", y);
    y
}

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./q.txt") {
        for line in lines {
            if let Ok(k) = line {
                //println!("{}", k);
                sum+=handle(&k);
            }
        }
    }
    println!("{}", sum);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}