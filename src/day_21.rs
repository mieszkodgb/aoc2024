use ratatui::init;


#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord{
    x: usize,
    y: usize
}

enum NumberPad{
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    ZERO,
    A
}
fn number_to_coord(input: char)-> Coord{
    match input {
        '0' => Coord{x:1, y:0},
        '1' => Coord{x:0, y:1},
        '2' => Coord{x:1, y:1},
        '3' => Coord{x:2, y:1},
        '4' => Coord{x:0, y:2},
        '5' => Coord{x:1, y:2},
        '6' => Coord{x:2, y:2},
        '7' => Coord{x:0, y:3},
        '8' => Coord{x:1, y:3},
        '9' => Coord{x:2, y:3},
        'A' => Coord{x:2, y:0},
        _ => panic!("Bad number pad input")
    }
}

fn dir_to_coord(input: char)-> Coord{
    match input {
        '^' => Coord{x:1, y:1},
        '<' => Coord{x:0, y:0},
        '>' => Coord{x:2, y:0},
        'v' => Coord{x:1, y:0},
        'A' => Coord{x:2, y:1},
        _ => panic!("Bad direction pad input")
    }
}

fn up_left_path(target_coord: Coord, curent_pos: &mut Coord) -> Vec<char>{
    let mut new_dir: Vec<char> = Vec::with_capacity(target_coord.y.abs_diff(curent_pos.y)+target_coord.x.abs_diff(curent_pos.x));
    while target_coord != *curent_pos{
        // print!("Going from {:?} to {:?} ",curent_pos , target_coord);
        while target_coord.x < curent_pos.x && !(curent_pos.x == 1 && curent_pos.y == 0){
            new_dir.push('<');
            curent_pos.x -= 1;
        }
        while target_coord.y > curent_pos.y{
            new_dir.push('^');
            curent_pos.y += 1;
        }
        while target_coord.y < curent_pos.y && !(curent_pos.x == 0 && curent_pos.y == 1){
            new_dir.push('v');
            curent_pos.y -= 1;
        }
        while target_coord.x > curent_pos.x{
            new_dir.push('>');
            curent_pos.x += 1;
        }
        // println!("by {:?}", new_dir);
    }
    new_dir.push('A');
    return new_dir;
}

fn right_down_path(target_coord: Coord, curent_pos: &mut Coord) -> Vec<char>{
    let mut new_dir: Vec<char> = Vec::with_capacity(target_coord.y.abs_diff(curent_pos.y)+target_coord.x.abs_diff(curent_pos.x));
    while target_coord != *curent_pos{
        // print!("Going from {:?} to {:?} ",curent_pos , target_coord);
        while target_coord.y < curent_pos.y && !(curent_pos.x == 0 && curent_pos.y == 1){
            new_dir.push('v');
            curent_pos.y -= 1;
        }
        while target_coord.x > curent_pos.x{
            new_dir.push('>');
            curent_pos.x += 1;
        }
        while target_coord.x < curent_pos.x && !(curent_pos.x == 1 && curent_pos.y == 0){
            new_dir.push('<');
            curent_pos.x -= 1;
        }
        while target_coord.y > curent_pos.y{
            new_dir.push('^');
            curent_pos.y += 1;
        }
        // println!("by {:?}", new_dir);
    }
    new_dir.push('A');
    return new_dir;
}

fn get_all_path(target_coord: Coord, curent_pos: &mut Coord) -> Vec<Vec<char>>{
    
}

fn up_left_path2(target_coord: Coord, curent_pos: &mut Coord) -> Vec<char>{
    let mut new_dir: Vec<char> = Vec::with_capacity(target_coord.y.abs_diff(curent_pos.y)+target_coord.x.abs_diff(curent_pos.x));
    while target_coord != *curent_pos{
        // print!("Going from {:?} to {:?} ",curent_pos , target_coord);
        while target_coord.x < curent_pos.x && !(curent_pos.x == 1 && curent_pos.y == 1){
            new_dir.push('<');
            curent_pos.x -= 1;
        }
        while target_coord.y > curent_pos.y && !(curent_pos.x == 0 && curent_pos.y == 0){
            new_dir.push('^');
            curent_pos.y += 1;
        }
        while target_coord.y < curent_pos.y{
            new_dir.push('v');
            curent_pos.y -= 1;
        }
        while target_coord.x > curent_pos.x{
            new_dir.push('>');
            curent_pos.x += 1;
        }
        // println!("by {:?}", new_dir);
    }
    new_dir.push('A');
    return new_dir;
}

fn right_down_path2(target_coord: Coord, curent_pos: &mut Coord) -> Vec<char>{
    let mut new_dir: Vec<char> = Vec::with_capacity(target_coord.y.abs_diff(curent_pos.y)+target_coord.x.abs_diff(curent_pos.x));
    while target_coord != *curent_pos{
        // print!("Going from {:?} to {:?} ",curent_pos , target_coord);
        while target_coord.y < curent_pos.y{
            new_dir.push('v');
            curent_pos.y -= 1;
        }
        while target_coord.x > curent_pos.x{
            new_dir.push('>');
            curent_pos.x += 1;
        }
        while target_coord.x < curent_pos.x && !(curent_pos.x == 1 && curent_pos.y == 1){
            new_dir.push('<');
            curent_pos.x -= 1;
        }
        while target_coord.y > curent_pos.y && !(curent_pos.x == 0 && curent_pos.y == 0){
            new_dir.push('^');
            curent_pos.y += 1;
        }
        // println!("by {:?}", new_dir);
    }
    new_dir.push('A');
    return new_dir;
}

fn robot_number_to_direction(sequence_to_press: String)-> (Vec<char>, Vec<char>){
    let mut init_pos = number_to_coord('A');
    let mut direction_input: Vec<char> = Vec::new();
    let mut direction_input2: Vec<char> = Vec::new();
    for button_to_press in sequence_to_press.chars(){
        // println!("Button to press is {}", button_to_press);
        let target_coord = number_to_coord(button_to_press);
        // println!("Going from {:?} to {:?}", init_pos , target_coord);

        // let mut curent_pos = init_pos.clone();
        
        let mut new_dir = right_down_path(target_coord, &mut init_pos.clone());
        let mut new_dir2 = up_left_path(target_coord, &mut init_pos.clone());
        // println!("Path {} vs {}", new_dir.len(), new_dir2.len());
        // println!("Going from {:?} to {:?} by {:?}", init_pos , target_coord, new_dir);
        direction_input.append(&mut new_dir);
        direction_input2.append(&mut new_dir2);
        // if new_dir.len() > new_dir2.len(){
        //     direction_input.append(&mut new_dir2);
        // }
        // else{
        //     direction_input.append(&mut new_dir);
        // }
        
        init_pos = target_coord;
    }
    return (direction_input, direction_input2);
}

fn robot_direction_to_direction(sequence_to_press: Vec<char>)-> (Vec<char>, Vec<char>){
    let mut init_pos = dir_to_coord('A');
    let mut direction_input: Vec<char> = Vec::new();
    let mut direction_input2: Vec<char> = Vec::new();
    for button_to_press in sequence_to_press.iter(){
        let target_coord = dir_to_coord(*button_to_press);
        // let mut curent_pos = init_pos.clone();
        let mut new_dir = right_down_path2(target_coord, &mut init_pos.clone());
        let mut new_dir2 = up_left_path2(target_coord, &mut init_pos.clone());
        // println!("Path {} vs {}", new_dir.len(), new_dir2.len());
        // println!("Going from {:?} to {:?} {:?} by {:?}",init_pos ,button_to_press, target_coord, new_dir);
        direction_input.append(&mut new_dir);
        direction_input2.append(&mut new_dir2);
        init_pos = target_coord;
    }
    return (direction_input, direction_input2);
}


pub fn part_one(input: Vec<String>){
    let mut total_complexity = 0;
    for sequence in input{
        println!("Start sequance {}", sequence);
        let (mut direction_input, mut direction_input_bis)= robot_number_to_direction(sequence.clone());
        // println!("First input is {:?} with size {}", direction_input, direction_input.len());

        let (mut direction_input2_11, mut direction_input2_12) = robot_direction_to_direction(direction_input);
        let (mut direction_input2_21, mut direction_input2_22) = robot_direction_to_direction(direction_input_bis);

        let (mut direction_input3_111, mut direction_input3_112) = robot_direction_to_direction(direction_input2_11);
        let (mut direction_input3_121, mut direction_input3_122) = robot_direction_to_direction(direction_input2_12);
        let (mut direction_input3_211, mut direction_input3_212) = robot_direction_to_direction(direction_input2_21);
        let (mut direction_input3_221, mut direction_input3_222) = robot_direction_to_direction(direction_input2_22);


        // let (mut direction_input2, mut direction_input2_bis) = robot_direction_to_direction(direction_input);
        // let mut direction_input2_bis = robot_direction_to_direction(direction_input_bis);
        // println!("Second input is {:?} with size {}", direction_input2, direction_input2.len());
        // let (mut direction_input3_11, mut direction_input3_12) = robot_direction_to_direction(direction_input2);
        // let (mut direction_input3_21, mut direction_input3_22) = robot_direction_to_direction(direction_input2_bis);
        // println!("Third input is {:?} with size {}", direction_input3, direction_input3.len());
        println!("Path {} vs {} vs {} vs {} vs {} vs {} vs {} vs {}",
            direction_input3_111.len(), direction_input3_112.len(),
            direction_input3_121.len(), direction_input3_122.len(),
            direction_input3_211.len(), direction_input3_212.len(),
            direction_input3_221.len(), direction_input3_222.len());
        // dbg!(direction_input4);
        let shortest_path = direction_input3_111.len() as i32;
        let numeric_part = sequence[..sequence.len()-1].parse::<i32>().unwrap();
        let complexity = shortest_path*numeric_part;
        println!("For sequence {}, shortest_path is {} with number {}",sequence, shortest_path, numeric_part);
        total_complexity += complexity;
    }
    dbg!(total_complexity);
    
}