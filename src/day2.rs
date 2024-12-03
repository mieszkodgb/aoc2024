


pub fn check_level(level: Vec<i32>)-> bool{
    let level_win: Vec<i32> = level.windows(2).map(|s| s[1] - s[0]).collect();
    let mut rev_level = level.clone();
    rev_level.reverse();
    let level_win_prod = level_win.iter().product::<i32>();
    // println!("{}, {}", level_win_prod, level_win[0].signum());
    let all_pos_or_neg: bool = level_win.iter().all(|&v| v<0) || level_win.iter().all(|&v| v>0);
    let within_range = (*level_win.iter().max().unwrap() <= 3) && (*level_win.iter().min().unwrap() >= 1)
                            ||(*level_win.iter().max().unwrap() <= -1) && (*level_win.iter().min().unwrap() >= -3);
    let is_safe = all_pos_or_neg && within_range;
    println!("{:?} gives {:?} is safe {} because {} {}", level, level_win, is_safe, all_pos_or_neg, within_range);
    return is_safe
}
pub fn part_one(input_vec: Vec<String>){
    let input_iter = input_vec.iter();
    let mut safeness: i32 = 0;
    for report in input_iter{
        let levels = report.split(" ").map(|v| v.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let is_safe = check_level(levels.clone());
        if is_safe{
            safeness+=1;
        }
        else{
            let mut sub_levels: Vec<i32> = levels.clone();
            for (i, val) in levels.iter().enumerate(){
                sub_levels.remove(i);
                let is_safe = check_level(sub_levels.clone());
                if is_safe{
                    safeness+=1;
                    break;
                }
                sub_levels = levels.clone();
            }
        }
        
        
    }
    println!("{}", safeness);
}