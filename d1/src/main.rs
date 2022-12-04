use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut higher_calories = vec![0;3];
    let mut carry_over_calories = 0;

    for line in stdin.lock().lines() {
        let l = line.unwrap();

        if l.is_empty() {
            if carry_over_calories > higher_calories[0] {
                higher_calories[0] = carry_over_calories;
                higher_calories.sort();
            }

            carry_over_calories = 0;

            continue;
        }

        let calories: i32 = l.parse().unwrap();

        carry_over_calories = carry_over_calories + calories;
    }
    
    println!("sol 1: {}", higher_calories[2]);
    println!("sol 2: {}", higher_calories.iter().sum::<i32>());

}
