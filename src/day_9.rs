use ratatui::layout;
use regex::Regex;

pub fn part_one(input: &String){
    let input_vec:Vec<usize> = input.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect();
    let mut vec_size: usize = input_vec.iter().sum();
    vec_size += input_vec.iter().filter(|&&val| val==0).count();
    println!("{vec_size}");
    let mut layout: Vec<String> = vec![".".to_string(); vec_size];// Vec::with_capacity(vec_size);
    let mut current_st: usize = 0;
    // println!("{:?}", input_vec);
    for (id, &val) in input_vec.iter().enumerate(){
        // println!("{}-{} for val {}",current_st,current_st+val, val);

        if id%2 == 0 && val != 0{
            layout[current_st..current_st+val].fill((id/2).to_string());
        }
        else if val !=0{
            layout[current_st..current_st+val].fill(".".to_string());
        }

        current_st += val;
    }
    // println!("{:?}", layout);
    for id in 0..layout.len().clone(){

        if(layout[id]) == "."{
            let pos = layout.iter().rposition(|val| val.parse::<u32>().is_ok()).unwrap();
            if pos > id{
                layout[id] = layout[pos].clone();
                layout[pos] = ".".to_string();
            }

        }
    }
    let checksum: u64 = layout.iter().enumerate().filter(|(_, val)| val.parse::<u32>().is_ok()).map(|(idx, val)| val.parse::<u64>().unwrap()*(idx as u64)).sum();
    // println!("{:?}", layout);
    println!("{}", checksum);
}

pub fn part_two(input: &String){
    let input_vec:Vec<usize> = input.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect();
    let mut vec_size: usize = input_vec.iter().sum();
    vec_size += input_vec.iter().filter(|&&val| val==0).count();
    println!("{vec_size}");
    let mut layout: Vec<Vec<String>> = vec![vec![".".to_string()]; input_vec.len()];
    // let mut layout: Vec<String> = vec![".".to_string(); vec_size];// Vec::with_capacity(vec_size);
    let mut current_st: usize = 0;
    // println!("{:?}", input_vec);
    for (id, &val) in input_vec.iter().enumerate(){
        // println!("{}-{} for val {}",current_st,current_st+val, val);

        if id%2 == 0 && val != 0{
            layout[id] = vec![(id/2).to_string(); val];
            // layout[current_st..current_st+val].fill((id/2).to_string());
        }
        else if val !=0{
            // layout[current_st..current_st+val].fill(".".to_string());
            layout[id] = vec![".".to_string(); val];
        }

        current_st += val;
    }
    

    for (mut idx, vec_val) in layout.clone().iter().rev().enumerate(){
        // println!("{:?}", layout);
        idx = layout.len() - 1 - idx;
        if vec_val.last().unwrap() != "."{
            let pos_opt = layout.iter().position(|val| val.last().unwrap() == "." && val.len()>= vec_val.len());
            if pos_opt.is_none(){
                continue;
            }
            let pos = pos_opt.unwrap();
            let idx2 = layout.iter().rposition(|val| val == vec_val).unwrap();
            if pos < idx2{
                // println!("Move {:?}", vec_val);
                // println!("At {:?}", layout.iter().position(|val| val == vec_val).unwrap());
                
                // println!("Instead of {:?}", idx);
                let missing = layout[pos].len()-vec_val.len();
                layout[pos] = vec_val.clone();
                layout[idx2].fill(".".to_string());
                if missing >0{
                    layout.insert(pos+1, vec![".".to_string();missing]);
                }
                
            }
        }
        
    }

    let checksum: u64 = layout.iter().flatten().enumerate().filter(|(_, val)| val.parse::<u32>().is_ok()).map(|(idx, val)| val.parse::<u64>().unwrap()*(idx as u64)).sum();
    // println!("{:?}", layout);
    println!("Final {}", checksum);
}


