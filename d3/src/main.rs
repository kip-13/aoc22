use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut priorites_total = 0;
    let mut group_priorities_total = 0;
    let mut ln = 0;
    let mut prev_group = HashSet::new();

    for line in stdin.lock().lines() {
        ln += 1;
        
        let l = line.unwrap();
        let items: Vec<String> = l.chars().map(|w| w.into()).collect();
        let items_pack: Vec<HashSet<String>> = items.chunks(items.len()/2).map(|it| Vec::from(it).into_iter().collect()).collect();
        let intersec: HashSet<_> = items_pack[0].intersection(&items_pack[1]).collect();

        for item_inter in intersec {
            let item = item_inter.chars().next().expect("empty");
            let value = item as u32 - if item.is_uppercase() { 38 } else { 96 };
            priorites_total += value;
        }

        if ln == 1 {
            prev_group = items.into_iter().collect();
            continue;
        }

        let group = items.into_iter().collect();
        prev_group = prev_group.intersection(&group).map(|w| w.into()).collect();

        if ln % 3 == 0 {
            for item_inter in prev_group.clone() {
                let item = item_inter.chars().next().expect("empty");
                let value = item as u32 - if item.is_uppercase() { 38 } else { 96 };
                group_priorities_total += value;
            }

            ln = 0;
        }
    }

    println!("sol 1: {}", priorites_total);
    println!("sol 2: {}", group_priorities_total);
}
