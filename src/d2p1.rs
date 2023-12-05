use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn validate(num:u32, color:&String)->bool{
    //println!("{} - {}", color, num);
    if color == "blue" && num > 14{
        return false;
    }
    if color == "red" && num > 12{
        return false;
    }
    if color == "green" && num > 13{
        return false;
    }
    true
}

fn handle(x: &String)-> bool {
    let byte_ar = x.as_bytes();
    let mut i = 0;
    while(byte_ar[i] as char != ':'){
        i += 1;
    }
    i+=2;

    while i < byte_ar.len(){
        let mut digit = 0;
        let mut color = String::from("");
        while(byte_ar[i] as char != ' '){
            digit = digit*10 + (byte_ar[i] as char).to_digit(10).unwrap();
            i+=1;
        }
        i+=1;

        while i < byte_ar.len() && (byte_ar[i] as char != ',' && byte_ar[i] as char != ';'){
            color.push(byte_ar[i] as char);
            i+=1;
        }

        if (i < byte_ar.len()){
            i+=2;
        }
        if !validate(digit, &color){
            return false;
        }
    }
    true
    //get digit

}

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./q.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(k) = line {
                //println!("{}", k);
                if handle(&k){
                    //println!("{}",i);
                    sum += i + 1;
                }
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