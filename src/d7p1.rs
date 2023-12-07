use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, Display};
use std::vec;
use std::cmp::{min, Ordering, Eq};
use std::fmt;

const  VALUEORD:[char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8','7','6','5', '4', '3','2'];


fn o2s(s: Ordering) -> char {
    if s == Ordering::Equal {
        return '=';
    }
    else if s == Ordering::Less {
        return '<';
    }
    else {
        return '>';
    }
}
#[derive(Debug)]
struct Card {
    c: char
}
impl Eq for Card {
    
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.c == other.c;
    }
}

impl PartialOrd for Card {
    fn ge(&self, other: &Self) -> bool {
        for prior in VALUEORD {
            if prior == self.c {
                return true;
            }
            if prior == other.c {
                return false;
            }
        }
        return false;
    }
    fn gt(&self, other: &Self) -> bool {
        return !self.eq(other) && self.ge(other);
    }
    fn le(&self, other: &Self) -> bool {
        for prior in VALUEORD {
            if prior == other.c {
                return true;
            }
            if prior == self.c {
                return false;
            }
        }
        return false;
    }
    fn lt(&self, other: &Self) -> bool {
        return !self.eq(other) && self.le(other);
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.c == other.c {return Some(Ordering::Equal);}
        if self.ge(other) {
            return Some(std::cmp::Ordering::Less);
        }
        return Some(std::cmp::Ordering::Equal);
    }
}
struct Hand {
    l:Vec<Card>,
    ctype:u8,
    bid:u32
}
impl Hand {
    fn get_type(v:&Vec<Card>) -> u8 {
        //dbg!(&v);
        let mut p3 = 0;
        let mut p2 = 0;
        let mut p1 = 0;
        let mut p = [0, 0, 0, 0];
        for c in v {
            let mut count = 0;
            for cr in v {
                if c == cr {
                    count += 1;
                }
            }
            if count == 5 {return 6;}
            if count == 4 {return 5;}
            p[count] += 1;
        }
        if p[3] == 3 {
            if p[2] == 2 {return 4;}
            return 3;
        }
        if p[2] == 4 {return 2;}
        if p[2] == 2 {return 1;}
        //dbg!(p);
        return 0;
    }

    fn new(str: &str, bid:u32) -> Self {
        let mut l:Vec<Card> = vec![];
        for char in str.chars() {
            l.push(Card {c:char});
        }
        let ctype = Hand::get_type(&l);
        Self {l, ctype, bid}
    }
}

impl Eq for Hand {

}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("");
        for x in &self.l {
            s.push(x.c);
        }
        write!(f, "{}", s)
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.ctype > other.ctype {
            return Ordering::Greater;
        }
        if self.ctype < other.ctype {
            return Ordering::Less;
        }

        for i in 0..self.l.len() {
            if self.l[i] > other.l[i] {
                return Ordering::Greater;
            }
            else if self.l[i] < other.l[i] {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.l.len() {
            if self.l[i] != other.l[i] {
                return false;
            }
        }
        return true;
    }
}


impl PartialOrd for Hand {
    fn gt(&self, other: &Self) -> bool {
        if self.ctype != other.ctype {return self.ctype > other.ctype}
        for i in 0..self.l.len() {
            if self.l[i] > other.l[i] {
                //println!("{} {} {}", self.l[i].c, '>', other.l[i].c);
                return true;
            }
            else if self.l[i] < other.l[i] {
                return false;
            }
        }
        return false;
    }   
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.gt(other) {
            //println!("{} {} {}", self, '>', other);
            return Some(std::cmp::Ordering::Greater);
        }
        else if self.eq(other) {
            //println!("{} {} {}", self, '=', other);
            return Some(std::cmp::Ordering::Equal);
        }
        //println!("{} {} {}", self, '<', other);
        return Some(std::cmp::Ordering::Less);
    }
}

fn main() {
    //let mut sum = 0;
    let text = get_text();
    let mut txt = text.split("\n");
    let mut hands:Vec<Hand> = vec![];
    for line in txt {
        if line == "" {continue;}
        let mut it = line.split(" ");
        let h = it.next().unwrap();
        let b = it.next().unwrap();
        hands.push(Hand::new(&h, b.parse::<u32>().unwrap()));
    }
    //hands[2].p
    //println!("{}", hands[1].partial_cmp(&hands[2]));
    hands.sort();
    
    let mut sum = 0;
    for i in 0..hands.len() {
        let rank = (i+1) as u32;
        sum += rank*hands[i].bid;
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