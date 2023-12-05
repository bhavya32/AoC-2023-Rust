use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn match_num1(t:&[u8], mut i:usize, num:&str)->bool{
    let slen = t.len();
    let l = num.len();
    if i + l <= slen{
        for nc in num.chars(){
            if nc != (t[i] as char){
                //println!("{}!= {}", nc, t[i] as char);
                return false
            }
            i+=1;
        }
    }
    else {
        return false
    }
    true

}

fn match_num(t:&[u8], i:usize)-> usize {
    
    let nums = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
    for (ind, num) in nums.iter().enumerate(){
        let res = match_num1(t, i, num);
        //println!("{} - {} - {} - {}", String::from_utf8_lossy(t),i,num,res);
        if res {
            return ind
        }
    }
    10
}


fn handle(x: &String)-> u32 {
    let mut y = 0;
    let mut ff = false;
    let mut ld = 0; 
    let t = x.as_bytes();
    let slen = t.len();
    for i in 0..slen{
        let c = t[i] as char;
        if c.is_digit(10){
            if ff == false {
                y = c.to_digit(10).unwrap() * 10;
                ff = true;
            };
            ld = c.to_digit(10).unwrap();
            continue;
        }

        //match for a word
        let m = match_num(t, i);
        if m != 10{
            if ff == false {
                y = (m as u32)* 10;
                ff = true;
            };
            ld = m as u32;
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