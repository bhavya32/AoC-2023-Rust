use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;




fn handle(x: &String)-> u32 {
    let byte_ar = x.as_bytes();
    let mut i = 0;
    let mut win_count = 0;
    while(byte_ar[i] as char != ':'){
        i += 1;
    }
    i+=2;

    let t = String::from(&x[i..x.len()]);
    //println!("{}", t);
    let mut iter = t.split(" | ");
    let winning_set:HashSet<&str> = HashSet::from_iter(iter.next().unwrap().split(" "));
    //dbg!(&winning_set);
    for num in iter.next().unwrap().split(" ") {
        if num == "" {continue;}
        if winning_set.contains(num) {
            win_count += 1;
        }
    }
    

    if win_count == 0 {
        return 0;
    }
    return (2 as i32).pow(win_count-1) as u32;
    //get digit

}

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./q.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(k) = line {
                //println!("{}", k);
                sum += handle(&k);
                
                //sum+=handle(&k);
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