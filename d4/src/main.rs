use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut groups_fully_contains = 0;
    let mut overlaps = 0;

    for line in stdin.lock().lines() {        
        let l = line.unwrap();
        let items: Vec<HashSet<i32>> = l.split(',').into_iter()
            .map(|r| {
                let n: Vec<i32> = r.split('-').map(|n| n.parse().expect("int")).collect();
                return (n[0]..=n[1]).collect();
            })
            .collect();
        let intersec: HashSet<_> = items[0].intersection(&items[1]).collect();

        if intersec.len() < 1 {
            continue;
        }

        overlaps += 1;
        
        let contains_range = intersec.len() == items[0].len() || intersec.len() == items[1].len();

        if contains_range {
            groups_fully_contains += 1;
        }
    }

    println!("sol 1: {}", groups_fully_contains);
    println!("sol 2: {}", overlaps);
}
