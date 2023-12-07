use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, Display};
use std::vec;
use std::cmp::{min, Ordering, Eq};
use std::fmt;

const  VALUEORD:[char; 13] = ['A', 'K', 'Q', 'T', '9', '8','7','6','5', '4', '3','2', 'J'];

struct Card {
    c: char
}
impl Eq for Card {}
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
    fn get_type_n(v:&Vec<Card>) -> u8 {
        let mut hmap: HashMap<char, u8> = HashMap::new();
        //dbg!(v);
        for c in v {
            *hmap.entry(c.c).or_insert(0) += 1;
        }
        //dbg!(&hmap);
        let mut nj = hmap.remove(&'J').unwrap_or(0);
        if nj == 5 || nj == 4 {return 6;}
        let mut p = [0, 0, 0, 0];
        
        let mut mv:Vec<u8> = hmap.iter().map(|f| *f.1).filter(|f| *f!= 0).collect();
        mv.sort_by(|a, b| b.cmp(a));

        let ccount = mv[0];
        let ncount = nj + ccount;
        if ccount == 5 {return 6;}
        if ncount == 5 {return 6;}
        if ncount == 4 {return 5;}
        p[ncount as usize] += 1;
        //dbg!(&mv);
        for count in mv.iter().skip(1) {
            p[*count as usize] += 1; 
        }
        if p[3] == 1 {
            if p[2] == 1 {return 4;}
            return 3;
        }
        if p[2] == 2 {return 2;}
        if p[2] == 1 {return 1;}
        //dbg!(p);
        0
    }
    
    fn new(str: &str, bid:u32) -> Self {
        let mut l:Vec<Card> = vec![];
        for char in str.chars() {
            l.push(Card {c:char});
        }
        let ctype = Hand::get_type_n(&l);
        /*let ctype_old = Hand::get_type(&l);
        if ctype != ctype_old {
            println!("{} {} != {}", str, ctype, ctype_old);
        }*/
        Self {l, ctype, bid}
    }
}

impl Eq for Hand {}
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
            return Some(std::cmp::Ordering::Greater);
        }
        else if self.eq(other) {
            return Some(std::cmp::Ordering::Equal);
        }
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