use std::collections::HashMap;

pub fn part_one(input_vec: &Vec<String>){
    let mut current_vec: Vec<u64> = input_vec.iter().map(|val| val.parse::<u64>().unwrap()).collect();
    println!("{:?}", input_vec);
    for k in 1..26{
        current_vec = play_next(&current_vec);
        println!("Blink {k} has {:?} stones", current_vec.len())
    }

}

fn play_next(current_vec: &Vec<u64>)-> Vec<u64>{
    let mut next_vec = Vec::new();
    for &val in current_vec.iter(){
        if val == 0{
            next_vec.push(1);
        }
        else if val.to_string().len()%2 == 0{
            let val_str = val.to_string();
            next_vec.push(val_str[..val_str.len()/2].parse::<u64>().unwrap());
            next_vec.push(val_str[val_str.len()/2..].parse::<u64>().unwrap());
        }
        else{
            next_vec.push(val*2024);
        }
    }
    return next_vec;
}

fn apply_rule(val: u64) -> Vec<u64>{
    if val == 0{
        return vec![1];
    }
    else if val.to_string().len()%2 == 0{
        let val_str = val.to_string();
        return vec![val_str[..val_str.len()/2].parse::<u64>().unwrap(),
            val_str[val_str.len()/2..].parse::<u64>().unwrap()];
    }
    else{
        return vec![val*2024];
    }

}

pub fn part_two(input_vec: &Vec<String>){
    let mut current_vec: Vec<u64> = input_vec.iter().map(|val| val.parse::<u64>().unwrap()).collect();
    println!("{:?}", input_vec);
    let mut cache: HashMap<Blink, usize> = HashMap::new();
    let result: usize = current_vec.iter().map(|&val| get_count(val, 75, &mut cache)).sum();
    println!("{:?}",result);

}


#[derive(PartialEq, Eq, Hash)]
struct Blink{
    iter: usize,
    init_val: u64
}
fn get_count(val: u64, iteration: usize, cache: &mut HashMap<Blink, usize>) -> usize{
    if iteration < 1 {
        return 1;
    }
    else if let Some(&result) = cache.get(&Blink{iter:iteration, init_val:val}) {
        result
    }
    else {
        let next_vec = apply_rule(val);
        let result: usize = next_vec.iter().map(|&nval| get_count(nval, iteration-1, cache)).sum();
        cache.insert(Blink{iter:iteration, init_val:val}, result);
        result
    }
}