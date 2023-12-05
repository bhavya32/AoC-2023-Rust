use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;
use std::cmp::min;

struct Mapping {
    start:i64,
    end:i64,
    diff:i64
}
fn map_n(seeds:&mut Vec<seed>, mapper:&str)-> Vec<seed>{
    let mut maps = mapper.split("\n");
    maps.next();
    let mut v:Vec<Mapping> = vec![];
    let mut tempseeds:Vec<seed> = vec![];
    for map in maps {
        if map == "" {continue;}
        let mut t:Vec<i64> = vec![];
        map.split(" ").for_each(|f| t.push(f.parse::<i64>().unwrap()) );
        let x = Mapping { start: t[1] as i64, end: (t[1] + t[2] - 1) as i64, diff: (t[0] - t[1]) as i64 };

        let mut idx = 0;
        while (idx < seeds.len()){
            //println!("{}", idx);
            let olidx = idx;
            idx += 1;
            if seeds[olidx].end < seeds[olidx].start {continue;}
            if seeds[olidx].end < x.start {continue;}
            if seeds[olidx].start > x.end {continue;}
            //-------
            //     ------
            if seeds[olidx].start<=x.start{
                let map_end:i64 = min(x.end, seeds[olidx].end) as i64;
                let mut mapseed = seed {start:(x.start as i64 + x.diff) as i64, end:(map_end + x.diff) as i64};
                seeds[olidx].end = x.start - 1;
                tempseeds.push(mapseed);
                if seeds[olidx].end > x.end {
                    seeds.push(seed {start:x.end + 1, end: x.end})
                }
            }
            //     ------
            // ------
            else if seeds[olidx].start > x.start{
                let map_end:i64 = min(x.end, seeds[olidx].end) as i64;
                let mut mapseed = seed {start:(seeds[olidx].start as i64 + x.diff) as i64, end:(map_end + x.diff) as i64};
                seeds[olidx].start = x.end + 1;
                tempseeds.push(mapseed);
            }
        }
    }
    for i in seeds {
        if i.end < i.start {continue;}
        tempseeds.push(seed{..*i});
    }
    tempseeds

}

fn map(seeds:&mut Vec<i64>, mapper:&str){
    //println!("{}", mapper);
    let mut maps = mapper.split("\n");
    maps.next();
    let mut v:Vec<Mapping> = vec![];
    for map in maps {
        if map == "" {continue;}
        let mut t:Vec<i64> = vec![];
        //println!("{}", map);
        map.split(" ").for_each(|f| t.push(f.parse::<i64>().unwrap()) );
        v.push(Mapping { start: t[1] as i64, end: (t[1] + t[2] - 1) as i64, diff: (t[0] - t[1]) as i64 })
    }

    for i in 0..seeds.len() {
        let seed = seeds[i];
        for map in &v{
            if seed>=map.start && seed <= map.end {
                seeds[i] = (seeds[i] as i64 + map.diff) as i64;
                break;
            }
        }
    }
}

#[derive(Debug)]
struct seed {
    start:i64,
    end:i64,
}
fn main() {
    //let mut sum = 0;
    let txt = get_text();
    //println!("{}",txt);
    let arr:Vec<&str> = txt.split("\n\n").collect();
    let mut seeds:Vec<i64> = vec![];
    let mut nseeds:Vec<seed> = vec![];
    let t: Vec<&str> = arr[0].split(" ").collect();
    let mut idx = 1;
    while idx < t.len() {
        //let x = t[idx].parse::<i64>().unwrap();
        let start = t[idx].parse::<i64>().unwrap();
        let end = t[idx+1].parse::<i64>().unwrap()  + start - 1;
        nseeds.push(seed {start, end});
        idx +=2;
    }
    for i in 1..arr.len() {
        nseeds = map_n(&mut nseeds, arr[i]);
    }
    println!("{}", nseeds.iter().map(|f| f.start).min().unwrap());
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