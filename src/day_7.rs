
fn find_operation(result: i64, values: Vec<i64>) -> i64{
    println!("Try {result} {values:?}");
    let mut int_result= result;
    let mut operations: Vec<Operations> = Vec::with_capacity(values.len()-1);
    for (i, val) in values.iter().enumerate(){
        // println!("Int result {}", int_result);
        // if int_result < 0{
        //     println!("Stop {}", int_result);
        //     break;
        // }
        if int_result%val == 0{
            operations.push(Operations::MULT);
            print!("Is mult {}/{} = {} ", int_result, val, int_result/val);
            if i+1 < values.len(){
                println!("Removing {}", val);
                let other = find_operation(int_result-val, values[i+1..].to_vec());
                println!("Other found is {other}");
                if other == int_result-val{
                    
                    int_result = int_result-val;
                    println!("Match new int result is {}", int_result);
                    continue;
                }
            }
            // if i+1 < values.len(){
            //     let other = find_operation(int_result-val, values[i+1..].to_vec());
            //     println!("Alt: {:?}", other);
            //     if other != 0 && other == int_result-val{
            //         int_result -= val
            //     }
            //     else{
            //         int_result /= val;
            //     }
            // }
            // else {
            //     int_result /= val;
            // }
            // // int_result /= val;
            // println!("Continue {} where result is {}", int_result, result);
            int_result /= val;
        }
        else {
            operations.push(Operations::SUM);
            // print!("Is sum {}-{} = ", int_result, val);
            int_result -= val;
            // println!("{}", int_result);
            // println!("Is sum {}", int_result);
        }
    }

    if int_result == 0 || int_result == 1 {
        println!("Found {:?} with {:?}", result, operations);
        return result;
    }
    println!("NOPE {:?} with {:?}", result, values);
    return 0;
}

#[derive(Debug)]
enum Operations{
    SUM,
    MULT
}
pub fn part_one(input_vec: Vec<String>){
    let mut total_result:i64 = 0;
    for row in input_vec.iter(){
        println!("{:?}", row);
        let row_vec: Vec<&str> = row.split(": ").to_owned().collect();
        let result = row_vec[0].parse::<i64>().unwrap();
        let mut values: Vec<i64>  = row_vec[1].split(" ")
            .to_owned()
            .map(|val| val.parse::<i64>().unwrap())
            .collect();
        values.reverse();
        total_result += find_operation(result, values);
        // let mut operations: Vec<Operations> = Vec::with_capacity(values.len()-1);
        // let mut exp_result: i64 = 0;
        // let mut int_result= result;
        // print!("Try for {} {:?} ", result, values);
        // for (i, val) in values.iter().enumerate(){
        //     // println!("Int result {}", int_result);
        //     if int_result < 0{
        //         // println!("Stop {}", int_result);
        //         break;
        //     }
        //     if int_result%val == 0 && i != values.len()-1{
        //         operations.push(Operations::MULT);
        //         print!("Is mult {}/{} = ", int_result, val);
        //         int_result = int_result/val;
        //         println!("{}", int_result);
        //     }
        //     else {
        //         operations.push(Operations::SUM);
        //         print!("Is sum {}-{} = ", int_result, val);
        //         int_result = int_result-val;
        //         println!("{}", int_result);
        //         // println!("Is sum {}", int_result);
        //     }
        // }

        // if int_result == 0 || int_result == 1 {
        //     println!("OK");
        //     total_result += result as u64;
        // }
        // else {
        //     println!("Fail");
        // }
        

    }
    println!("{}", total_result)
    // println!("{:?}", input_vec.len());
    // let max_vals: usize = input_vec.iter().map(|row| row.to_owned().split(" ").collect::<Vec<&str>>().len()).sum();
    // // for row in input_vec.iter(){
    // //     max_vals = max(row.split(" ").len(), max_vals);
    // // }
    // println!("{:?}", max_vals);
}

fn is_reachable(target: usize, nums: &[usize]) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let (&last, rest) = nums.split_last().unwrap();
    if target % last == 0 && is_reachable(target / last, rest) {
        return true;
    }
    if target > last && is_reachable(target - last, rest) {
        return true;
    }
    false
}
    

pub fn part_one2(input_vec: Vec<String>){
    let mut total_result:i64 = 0;
    for row in input_vec.iter(){
        let row_vec: Vec<&str> = row.split(": ").to_owned().collect();
        let result = row_vec[0].parse::<i64>().unwrap();
        let mut values: Vec<usize>  = row_vec[1].split(" ")
            .to_owned()
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        values.reverse();
        if is_reachable(result as usize,&values,){
            total_result+=result;
        }
    }
    println!("{total_result}");
}