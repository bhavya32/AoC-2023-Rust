#![allow(non_snake_case)]

use std::collections::{VecDeque, HashSet};
use std::{fs, collections::HashMap};
//use std::time::Instant;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum PulseType {
    High,
    Low
}

impl PulseType {
    fn num(&self) -> usize {
        match self {
            PulseType::High => 1,
            PulseType::Low => 0
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Mtype {
    Ff,
    Conj,
    Brod
}
#[derive(Clone, Debug)]
struct Module<'a> {
    state:usize,
    next:Vec<&'a str>,
    tp:Mtype
}

struct Cnjmap<'a> {
    map:HashMap<&'a str, HashMap<&'a str, usize>>
}
impl Cnjmap<'_> {
    fn new() -> Self {
        Self {
            map:HashMap::new()
        }
    }


    fn check(&self, s:&str) -> bool {
        let t = self.map.get(s).unwrap();
        for x in t.values() {
            if *x == 0 {return false;}
        }
        return true;
    }
}

fn get_map<'a>(hmap:&HashMap<&'a str, Module<'a>>) -> Cnjmap<'a> {
    let mut temphset = HashSet::new();
    let mut x = Cnjmap::new();
    for (name, md) in hmap {
        if md.tp == Mtype::Conj {
            x.map.insert(name, HashMap::new());
            temphset.insert(name);
        }
    }
    for (name, md) in hmap {
        if md.tp == Mtype::Conj {continue;}
        for nxt in &md.next {
            //if *nxt == "dg" {djinplen += 1;}
            if !temphset.contains(nxt) {continue;}
            let tx = x.map.get_mut(nxt).unwrap();
            tx.insert(name, 0);
        }
    }
    x
}

fn d20(mut hmap:HashMap<&str, Module>, djinplen:usize)-> (u64, u64) {
    let mut queue:VecDeque<(PulseType, &str, &str)> = VecDeque::new();
    let mut hpulse = 0;
    let mut lpulse = 0;
    let mut cnjmp = get_map(&hmap);
    let mut cyclemap:HashMap<&str, usize> = HashMap::new();
    for i in 1..1001 {
        queue.push_back((PulseType::Low, "broadcaster", "origin"));

        while let Some((p, m, from)) = queue.pop_front() {
            //println!("sending {:?} to {}", p, m);
            
            match p {
                PulseType::High => {hpulse += 1;},
                PulseType::Low =>  {
                    if m == "rx" {println!("{i}");return(0,0);}
                    lpulse += 1;
                }
            };
            let mut hm;
            match hmap.get_mut(m){
                Some(xt)  => {hm = xt;},
                None => continue
            };
            match hm.tp {
                Mtype::Brod => {
                    //send a low pulse to all the members
                    for i in 0..hm.next.len() {
                        queue.push_back((PulseType::Low, hm.next[i], m));
                    }
                },
                Mtype::Ff => {
                    //check if the state is on or off 
                    if hm.state == 0 && p == PulseType::Low {
                        hm.state = 1;
                        for i in 0..hm.next.len() {
                            queue.push_back((PulseType::High, hm.next[i], m));
                        }
                    } else if hm.state == 1 && p == PulseType::Low  {
                        hm.state = 0;
                        for i in 0..hm.next.len() {
                            queue.push_back((PulseType::Low, hm.next[i], m));
                        }
                    }
                },
                Mtype::Conj => {
                    //cnjmp.set(m, from, p.num() );
                    cnjmp.map.get_mut(m).unwrap().insert(from, p.num());
                    //println!("{} - {:?}", m,cnjmp.map.get_mut(m).unwrap());
                    let np = if !cnjmp.check(m) {
                        PulseType::High
                    } else {
                        PulseType::Low
                    };
                    if m == "dg" && p.num() == 1{
                        if cyclemap.len() == djinplen {
                            println!("{:?}", cyclemap);
                        }
                        if !cyclemap.contains_key(from) {
                            cyclemap.insert(from, i);
                        }
                        println!("got a high pulse from {} at {}", from, i);
                    }
                    for i in 0..hm.next.len() {
                        queue.push_back((np, hm.next[i], m));
                    }
                }
            };
        }
    }
    (hpulse, lpulse)
}
fn main() {
    let text = fs::read_to_string("./q.txt").expect("Unable to read file");
    let lines = text.split("\r\n");
    let mut modules:HashMap<&str, Module> = HashMap::new();
    let mut djinplen = 0;
    for line in lines {
        let mut x = line.split(" -> ");
        let mut k = x.next().unwrap();
        
        let tp = match k.chars().next().unwrap() {
            '&' => Mtype::Conj,
            '%' => Mtype::Ff,
            _ => Mtype::Brod
        };
        if k.starts_with('&') || k.starts_with('%') {
            k = &k[1..k.len()];
        }
        let mut v:Vec<&str> = Vec::new();
        let iv = x.next().unwrap().split(", ");

        for t in iv {
            if t == "dg" {
                djinplen += 1;
            }
            v.push(t);
        }
        modules.insert(k, Module { state: 0, next: v, tp });
    }
    //println!("{:?}", djinplen);
    let (hpulse, lpulse) = d20(modules, djinplen);
    println!("{:?}, {}", (hpulse, lpulse), hpulse*lpulse);
    /*let now = Instant::now();
    let t = d19(workflow);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{t}");*/
}
