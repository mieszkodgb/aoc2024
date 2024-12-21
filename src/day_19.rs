use std::collections::HashMap;

use colored::Colorize;

fn find_combi(serie: &String, combinations: &Vec<String>, memo: &mut HashMap<String, bool>) -> bool{
    dbg!(memo.len());
    if serie.is_empty(){
        return true;
    }
    if memo.contains_key(serie){
        return memo[serie]
    }
    for val in combinations.iter(){
        // dbg!(&new_serie);
        if serie.starts_with(val){
            let new_serie =  serie[val.len()..].to_owned().clone();
            println!("Serie new {} val {}", new_serie, val);
            if find_combi(&new_serie, combinations, memo){
                memo.insert(new_serie, true);
                return true
            }
        }
    }

    memo.insert(serie.to_owned(), false);
    return false
}


fn find_all_combi(serie: &String, combinations: &Vec<String>, memo: &mut HashMap<String, i64>) -> i64{
    let mut counter = 0;
    // dbg!(memo.len());
    if serie.is_empty(){
        counter += 1;
        return counter;
    }
    if memo.contains_key(serie){
        return memo[serie]+counter
    }
    for val in combinations.iter(){
        // dbg!(&new_serie);
        if serie.starts_with(val){
            let new_serie =  serie[val.len()..].to_owned().clone();
            // println!("Serie new {} val {}", new_serie, val);
            let small_counter = find_all_combi(&new_serie, combinations, memo);

            // memo.insert(new_serie, counter);
            counter += small_counter
        }

    }

    memo.insert(serie.to_owned(), counter);
    return counter
}

pub fn part_one(input_vec: Vec<String>){
    let mut possible_combination: Vec<String> = input_vec[0].split(", ").map(|stripes| stripes.to_owned()).collect();
    possible_combination.sort_by(|a, b| b.len().cmp(&a.len()));
    let patterns: Vec<String> = input_vec[2..].to_vec();
    let mut counter = 0;
    for pattern in patterns{
        let mut memo: HashMap<String, bool> = HashMap::new();
        if find_combi(&pattern, &possible_combination, &mut memo){
            dbg!(&counter);
            counter += 1;
        }
        
    }
    println!("Counter: {}", counter);
}

pub fn part_two(input_vec: Vec<String>){
    let mut possible_combination: Vec<String> = input_vec[0].split(", ").map(|stripes| stripes.to_owned()).collect();
    possible_combination.sort_by(|a, b| b.len().cmp(&a.len()));
    let patterns: Vec<String> = input_vec[2..].to_vec();
    let mut counter = 0;
    for pattern in patterns{
        let mut memo: HashMap<String, i64> = HashMap::new();
        let new_count = find_all_combi(&pattern, &possible_combination, &mut memo);
        // dbg!(new_count);
        counter += new_count;
        
    }
    println!("Counter: {}", counter);
}