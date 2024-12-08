
#[derive(Debug, Copy, Clone)]
struct Coord{
    x: usize,
    y: usize
}

fn check_right(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord) -> i32{
    if pos.y >= input.len()-reference.len()+1{
        return 0;
    }
    for (i, val) in reference.iter().enumerate(){
        if val != &input[pos.x][pos.y+i]{
            return 0;
        }
    }
    return 1
}
fn check_left(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord) -> i32{
    if pos.y <= reference.len()-2{
        return 0;
    }
    for (i, val) in reference.iter().enumerate(){
        if val != &input[pos.x][pos.y-i]{
            return 0;
        }
    }
    return 1
}
fn check_bottom(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord) -> i32{
    if pos.x >= input.len()-reference.len()+1{
        return 0;
    }
    for (i, val) in reference.iter().enumerate(){
        if val != &input[pos.x+i][pos.y]{
            return 0;
        }
    }
    return 1
}
fn check_top(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord) -> i32{
    if pos.x <= reference.len()-2{
        return 0;
    }
    for (i, val) in reference.iter().enumerate(){
        if val != &input[pos.x-i][pos.y]{
            return 0;
        }
    }
    return 1
}

fn check_rt(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord) -> i32{
    if pos.y >= input.len()-reference.len()+1 || pos.x <= reference.len()-2{
        return 0;
    }
    for (i, val) in reference.iter().enumerate(){
        if val != &input[pos.x-i][pos.y+i]{
            return 0;
        }
    }
    return 1
}
fn check_lt(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord) -> i32{
    if pos.y <= reference.len()-2 || pos.x <= reference.len()-2{
        return 0;
    }
    for (i, val) in reference.iter().enumerate(){
        if val != &input[pos.x-i][pos.y-i]{
            return 0;
        }
    }
    return 1
}
fn check_rb(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord) -> i32{
    if pos.x >= input.len()-reference.len()+1 || pos.y >= input.len()-reference.len()+1{
        // println!("{} - {} - {}", pos.y, input.len(), reference.len());
        return 0;
    }
    for (i, val) in reference.iter().enumerate(){
        if val != &input[pos.x+i][pos.y+i]{
            return 0;
        }
    }
    return 1
}
fn check_lb(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord) -> i32{
    if pos.x >= input.len()-reference.len()+1 || pos.y <= reference.len()-2 {
        return 0;
    }
    for (i, val) in reference.iter().enumerate(){
        if val != &input[pos.x+i][pos.y-i]{
            return 0;
        }
    }
    return 1
}

pub fn part_one(input_vec: Vec<String>){
    let input_matrix = input_vec.iter().map(|row| row.chars().collect::<Vec<char>>()).collect();
    let reference = String::from("XMAS").chars().collect();
    
    let mut tot_counter = 0;
    for( i, row) in input_vec.iter().enumerate(){
        let row_char: Vec<char> = row.clone().chars().collect();
        for (j, char) in row_char.iter().enumerate(){
            if char==&("X".chars().collect::<Vec<char>>()[0]){
                let init_pos = Coord{x: i, y:j};
                tot_counter += check_left(&input_matrix, &reference, init_pos.clone());
                tot_counter += check_right(&input_matrix, &reference, init_pos.clone());
                tot_counter += check_top(&input_matrix, &reference, init_pos.clone());
                tot_counter += check_bottom(&input_matrix, &reference, init_pos.clone());
                tot_counter += check_lb(&input_matrix, &reference, init_pos.clone());
                tot_counter += check_rb(&input_matrix, &reference, init_pos.clone());
                tot_counter += check_lt(&input_matrix, &reference, init_pos.clone());
                tot_counter += check_rt(&input_matrix, &reference, init_pos.clone());
            }
            
        }
    }
    
    println!("{}", tot_counter);
}

fn check_cross(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord) -> i32{
    // println!("Found candidate at {};{}", pos.x, pos.y);
    if (pos.x >= input.len()-reference.len()+2 || pos.y >= input.len()-reference.len()+2 ) ||
        (pos.x <= reference.len()-3 || pos.y <= reference.len()-3) {
        // println!("Candidate out at {};{}", pos.x, pos.y);
        return 0;
    }
    // let sub = &input[pos.x-1..pos.x+1][pos.y-1..pos.y+1];
    // println!("Around is {:?}", sub);
    // println!("{} A {} \n {} A {}", input[pos.x-1][pos.y-1],input[pos.x+1][pos.y+1],input[pos.x+1][pos.y-1],input[pos.x-1][pos.y+1]);
    if (input[pos.x-1][pos.y-1] == 'M') && (input[pos.x+1][pos.y+1] == 'S') && 
        (input[pos.x-1][pos.y+1] == 'S') && (input[pos.x+1][pos.y-1] == 'M') {
    return 1;
    }
    else if (input[pos.x-1][pos.y-1] == 'S') && (input[pos.x+1][pos.y+1] == 'M') && 
        (input[pos.x-1][pos.y+1] == 'M') && (input[pos.x+1][pos.y-1] == 'S') {
    return 1;
    }
    else if (input[pos.x-1][pos.y-1] == 'M') && (input[pos.x+1][pos.y+1] == 'S') && 
        (input[pos.x-1][pos.y+1] == 'M') && (input[pos.x+1][pos.y-1] == 'S') {
    return 1;
    }
    else if (input[pos.x-1][pos.y-1] == 'S') && (input[pos.x+1][pos.y+1] == 'M') && 
        (input[pos.x-1][pos.y+1] == 'S') && (input[pos.x+1][pos.y-1] == 'M') {
    return 1;
    }
    return 0;
}

pub fn part_two(input_vec: Vec<String>){
    let input_matrix = input_vec.iter().map(|row| row.chars().collect::<Vec<char>>()).collect();
    let reference = String::from("MAS").chars().collect();
    
    let mut tot_counter = 0;
    for( i, row) in input_vec.iter().enumerate(){
        let row_char: Vec<char> = row.clone().chars().collect();
        for (j, char) in row_char.iter().enumerate(){
            if char==&'A'{
                let init_pos = Coord{x: i, y:j};
                tot_counter += check_cross(&input_matrix, &reference, init_pos.clone());
            }
            
        }
    }
    
    println!("{}", tot_counter);
}

fn check_next(input: &Vec<Vec<char>>, reference: &Vec<char>, pos: Coord, idx: usize) -> i32{
    // println!("{}, {}", pos.x, pos.y);
    let over_x = pos.x >= input.len()-1;
    let over_y = pos.y >= input.len()-1;
    let under_x = pos.x <= 1;
    let under_y = pos.y <= 1;
    println!("ref:{}-{}, val:{}", reference[idx], idx, input[pos.x][pos.y]);
    if idx == 3{
        println!("Goal");
        return 1;
    }
    if !over_y && input[pos.x][pos.y+1] == reference[idx] {
        let pos = Coord{x: pos.x, y: pos.y+1};
        return check_next(input, reference, pos, idx+1);
    }
    // else if input[pos.x+1][pos.y] == reference[idx] && !over_x {
    //     let pos = Coord{x: pos.x+1, y: pos.y};
    //     counter += check_next(input, reference, pos, idx+1, counter);
    // }
    // else if !under_y &&input[pos.x][pos.y-1] == reference[idx]{
    //     let pos = Coord{x: pos.x, y: pos.y-1};
    //     counter += check_next(input, reference, pos, idx+1, counter);
    // }
    // else if !under_x && input[pos.x-1][pos.y] == reference[idx] {
    //     let pos = Coord{x: pos.x-1, y: pos.y};
    //     counter += check_next(input, reference, pos, idx+1, counter);
    // }
    // else if input[pos.x+1][pos.y+1] == reference[idx] && !(over_x||over_y){
    //     let pos = Coord{x: pos.x+1, y: pos.y+1};
    //     counter += check_next(input, reference, pos, idx+1, counter);
    // }
    // else if !(under_x||under_y) && input[pos.x-1][pos.y-1] == reference[idx] {
    //     let pos = Coord{x: pos.x-1, y: pos.y-1};
    //     counter += check_next(input, reference, pos, idx+1, counter);
    // }
    // else if !(over_x||under_y) && input[pos.x+1][pos.y-1] == reference[idx]{
    //     let pos = Coord{x: pos.x+1, y: pos.y-1};
    //     counter += check_next(input, reference, pos, idx+1, counter);
    // }
    // else if !(over_y||under_x) && input[pos.x-1][pos.y+1] == reference[idx] {
    //     let pos = Coord{x: pos.x-1, y: pos.y+1};
    //     counter += check_next(input, reference, pos, idx+1, counter);
    // }
    // return counter;
    return 0
}


