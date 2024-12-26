pub fn part_one(input_vec: Vec<String>){
    let mut counter = 0;
    for row in input_vec{
        let mut splitted_row = row.split(": ");
        let res = splitted_row.next().unwrap().parse::<i64>().unwrap();
        let mut numbers = splitted_row.next().unwrap().split(" ").map(|val| val.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        // numbers.reverse();
        print!("Res: {:?} val {:?} ", res, numbers);
        
        if can_be_solved2(res, numbers){
            println!("OK");
            counter += res;
        }
        else {
            println!("NOPE");
        }
    }
    println!("Result: {:?}", counter);

}

fn can_be_solved(result: i64, numbers: Vec<i64>) -> bool{
    if numbers.len() == 1{
        return result == numbers[0];
    }

    let (&last_number, rest) = numbers.split_last().unwrap();
    // dbg!(last_number, result, &numbers);
    // dbg!(result%last_number,result >= last_number);
    if result%last_number == 0 && can_be_solved(result/last_number, rest.to_vec()){
        return  true;
    }

    if result >= last_number && can_be_solved(result-last_number, rest.to_vec()){
        return  true;
    }
    // dbg!(lastval);
    return false;
}   

fn concat(a: u64, b: u64) -> u64 {
    a as u64 * 10u64.pow(b.ilog10() + 1) + b as u64
}

fn can_be_solved2(result: i64, numbers: Vec<i64>) -> bool{
    if numbers.len() == 1{
        return result == numbers[0];
    }

    let (&last_number, rest) = numbers.split_last().unwrap();
    // dbg!(last_number, result, &numbers);
    // dbg!(result%last_number,result >= last_number);
    if result%last_number == 0 && can_be_solved2(result/last_number, rest.to_vec()){
        return  true;
    }

    if result >= last_number && can_be_solved2(result-last_number, rest.to_vec()){
        return  true;
    }
    // dbg!(result-last_number, 10*((last_number.checked_ilog10().unwrap_or(1)+1)), rest);
    let digits = (10*((last_number.checked_ilog10().unwrap_or(1)+1)) as i64);
    if (result-last_number)%digits== 0 && can_be_solved2((result-last_number)/digits, rest.to_vec()){
        return  true;
    }
    // dbg!(lastval);
    return false;
}   
