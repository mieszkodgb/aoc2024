

pub fn part_one(input_vec: Vec<String>){
    let input_iter = input_vec.iter();
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();
    for val in input_iter{
        let splitted_val = val.split("   ").map(|v| v.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        left_column.push(splitted_val[0].clone());
        right_column.push(splitted_val[1].clone())
    }
    left_column.sort();
    right_column.sort();
    // println!("left {}: {:?}",left_column.len(),left_column);
    // println!("right {}: {:?}",right_column.len(),right_column);
    let mut sum_column: Vec<i32> = Vec::with_capacity(left_column.len());
    for (i, (aval, bval)) in left_column.iter().zip(&right_column).enumerate() {
        sum_column.push((aval - bval).abs());
    }
    
    
    println!("sum: {:?}",sum_column.iter().sum::<i32>());
}

pub fn part_two(input_vec: Vec<String>){
    let input_iter = input_vec.iter();
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();
    for val in input_iter{
        let splitted_val = val.split("   ").map(|v| v.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        left_column.push(splitted_val[0].clone());
        right_column.push(splitted_val[1].clone())
    }
    left_column.sort();
    right_column.sort();
    println!("left {}: {:?}",left_column.len(),left_column);
    println!("right {}: {:?}",right_column.len(),right_column);
    let mut similarity:i32 = 0;
    for i in left_column.iter(){
        let mut right_occurences:i32 = 0;
        for k in right_column.iter(){
            if i==k{
                right_occurences+=1;
            }
        }
        similarity += right_occurences*i;
        
    }

    
    
    println!("sim is : {:?}",similarity);
}