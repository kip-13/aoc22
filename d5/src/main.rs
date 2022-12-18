use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut stacks: Vec<Vec<char>> =vec![];
    let mut stacks_one_move: Vec<Vec<char>> =vec![];
    let mut sol1 = "".into();
    let mut sol2 = "".into();

    for line in stdin.lock().lines() {        
        let l = line.unwrap();

        if l.is_empty() {
            for stack in stacks.iter_mut() {
                stack.reverse();
            }

            stacks_one_move = stacks.clone();

            continue;
        }

        if l[0..1] != *"m" {
            let v: Vec<_> = l.chars().collect();

            if stacks.len() < v.chunks(4).len() {
                stacks.resize_with(v.chunks(4).len(), || vec![]);
            }

            for (i, item) in v.chunks(4).enumerate() {
                if !item[1].is_alphabetic() {
                    continue;
                }
                stacks[i].push(item[1]);
            }

            continue;
        }
        
        let positions: Vec<i32> = l.split(' ').filter_map(|c| c.parse::<i32>().ok()).collect();
        let mut one_move = vec![];

        for _ in 0..positions[0] {
            if let Some(item) = stacks[positions[1] as usize - 1].pop() {
                stacks[positions[2] as usize - 1].push(item);
            }

            if let Some(item) = stacks_one_move[positions[1] as usize - 1].pop() {
                one_move.push(item);
            }
        }

        one_move.reverse();
        stacks_one_move[positions[2] as usize - 1].append(&mut one_move);
    }

    for (i, item) in stacks.into_iter().enumerate() {
        sol1 = format!("{}{}", sol1, item[item.len()-1]);
        sol2 = format!("{}{}", sol2, stacks_one_move[i][item.len()-1]);
    }
    
    println!("sol1: {}", sol1);
    println!("sol2: {}", sol2);
}
