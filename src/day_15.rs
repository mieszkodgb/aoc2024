use std::{clone, vec};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Direction{
    UP,
    RIGHT,
    #[default] DOWN,
    LEFT,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Coord{
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Default, Clone)]
pub struct GameState{
    player_pos: Coord,
    pub map: Vec<Vec<char>>,
}



fn show(storage_map: &Vec<Vec<char>>){
    let new_map: Vec<String> = storage_map.iter().map(|row|row.iter().collect::<String>()).collect();
    dbg!(new_map);
}

fn find_start(input_vec: &Vec<Vec<char>>) -> Coord{
    let start_point = input_vec.iter()
        .enumerate()
        .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, v)| (x, y, v)))
        .filter_map(|(y,x, &v)| {
            if v=='@'{
                Some(Coord{x:x, y:y})
            }
            else{
                None
            }
        }).collect::<Vec<Coord>>();
    return start_point[0];
}
fn get_next_pos(mut position: Coord, dir: &Direction) -> Coord{
    match dir {
        Direction::UP => position.y -= 1,
        Direction::RIGHT => position.x += 1,
        Direction::DOWN => position.y += 1,
        Direction::LEFT => position.x -= 1
    }
    return position;
}
fn push_barol(map: &Vec<Vec<char>>, current_pos: Coord, dir: &Direction) -> Option<i32>{
    let next_barol_pos = get_next_pos(current_pos, &dir);
    if map[next_barol_pos.y][next_barol_pos.x] == '#'{
        return None;
    }
    else if map[next_barol_pos.y][next_barol_pos.x] == '.' {
        return Some(1);
    }
    else if map[next_barol_pos.y][next_barol_pos.x] == 'O' {
        let next_move = push_barol(map, next_barol_pos, dir);
        match next_move {
            Some(x)=> return Some(x+1),
            None => return  None
        }
    }
    panic!();
}

fn play_next(current_state: GameState, next_move: Direction)-> GameState{
    let mut next_state = current_state.clone();
    let mut next_pos = get_next_pos(current_state.player_pos.clone(), &next_move);
    next_state.player_pos = next_pos;
    if current_state.map[next_pos.y][next_pos.x] == '#'{
        return  current_state;
    }
    else if current_state.map[next_state.player_pos.y][next_state.player_pos.x] == '.'{
        next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
        next_state.map[next_pos.y][next_pos.x] = '@';
        return next_state;
    }
    else if current_state.map[next_state.player_pos.y][next_state.player_pos.x] == 'O' {
        let barol_move = push_barol(&current_state.map, next_pos, &next_move);
        match barol_move {
            None => return current_state,
            Some(x) => {
                next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
                let mut last_barel_pos: Coord = next_state.player_pos.clone();
                for _ in 0..x { 
                    last_barel_pos = get_next_pos(last_barel_pos, &next_move);
                }
                next_state.map[last_barel_pos.y][last_barel_pos.x] = 'O';
                next_state.map[next_pos.y][next_pos.x] = '@';
            }
        }
        return next_state;


    }
    panic!("Should not be here {:?}", current_state.map[next_state.player_pos.y][next_state.player_pos.x]);
    // return  current_state;

}

pub fn part_one(input: Vec<String>){
    let mut storage_map: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();
    let mut is_map = true;
    for row in input{
        if row == ""{
            is_map = false;
        }
        if is_map{
            storage_map.push(row.chars().collect());
        }
        else{
            for mov in row.chars(){
                movements.push(mov)
            }
            
        }
    }
    // show(&storage_map);
    let position = find_start(&storage_map);
    let mut current_state = GameState{player_pos: position, map:storage_map};
    let mut dir;
    for player_move in movements{
        match player_move {
            '^' => dir = Direction::UP,
            'v' => dir = Direction::DOWN,
            '<' => dir = Direction::LEFT,
            '>' => dir = Direction::RIGHT,
            _ => panic!("Wrong direction")
        }
        current_state = play_next(current_state, dir);
        // show(&current_state.map);
    }
    let last_map = current_state.map.clone();
    let gps_pos = last_map.iter().enumerate().flat_map(|(y, v)| v.iter().enumerate().map(move |(x, v)| (x, y, v))
    .filter_map(|(x, y, v)|{
        if *v =='O'{
            Some(100*y+x)
        }
        else{
            None
        }
    })).sum::<usize>();
    println!("{:?}", gps_pos);

}
fn get_double(val: char)-> Vec<char>{
    match val {
        '.' => vec!['.';2],
        '#' => vec!['#';2],
        'O' => vec!['[', ']'],
        '@' => vec!['@', '.'],
        _ => panic!("Bad val to duplicate {:?}", val)
    }
}
fn double_map(map: Vec<Vec<char>>) ->Vec<Vec<char>>{
    let mut new_map: Vec<Vec<char>> = Vec::with_capacity(map.len()*2);
    for row in map{
        let mut new_row: Vec<char> = Vec::with_capacity(row.len()*2);
        for val in row{
            new_row.append(&mut get_double(val));
        }
        new_map.push(new_row);
    }
    return new_map;
}

pub fn part_two(input: Vec<String>){
    let mut storage_map: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();
    let mut is_map = true;
    for row in input{
        if row == ""{
            is_map = false;
        }
        if is_map{
            storage_map.push(row.chars().collect());
        }
        else{
            for mov in row.chars(){
                movements.push(mov)
            }
            
        }
    }
    storage_map = double_map(storage_map);
    show(&storage_map);
    let position = find_start(&storage_map);
    let mut current_state = GameState{player_pos: position, map:storage_map};
    let mut dir;
    for player_move in movements{
        match player_move {
            '^' => dir = Direction::UP,
            'v' => dir = Direction::DOWN,
            '<' => dir = Direction::LEFT,
            '>' => dir = Direction::RIGHT,
            _ => panic!("Wrong direction")
        }
        current_state = play_next2(current_state, dir);
        show(&current_state.map);
    }
}

fn play_next2(current_state: GameState, next_move: Direction)-> GameState{
    let mut next_state = current_state.clone();
    let next_pos = get_next_pos(current_state.player_pos.clone(), &next_move);
    dbg!(next_move,next_pos);
    next_state.player_pos = next_pos;
    if current_state.map[next_pos.y][next_pos.x] == '#'{
        return  current_state;
    }
    else if current_state.map[next_state.player_pos.y][next_state.player_pos.x] == '.'{
        next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
        next_state.map[next_pos.y][next_pos.x] = '@';
        return next_state;
    }
    else if current_state.map[next_state.player_pos.y][next_state.player_pos.x] == '[' ||
        current_state.map[next_state.player_pos.y][next_state.player_pos.x] == ']' {
        let next_state = push_big_barol(&current_state, next_pos, &next_move);
        match next_state {
            None => return current_state,
            Some(mut x) => {
                x.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
                x.map[next_pos.y][next_pos.x] = '@';
                return x;
            }
        }
    }
    panic!("Should not be here {:?}", current_state.map[next_state.player_pos.y][next_state.player_pos.x]);
    // return  current_state;

}


fn push_two_big_barol(current_state: &GameState, barols_pos: Vec<Vec<Coord>>, dir: Direction)->Option<GameState>{
    if [Direction::LEFT, Direction::RIGHT].contains(&dir){
        panic!("Bad direction for 2 box");
    }
    let next_state_opt = push_one_big_barol(current_state, barols_pos[0].clone(), dir);
    match next_state_opt {
        Some(x) => {
            let next_state_opt = push_one_big_barol(current_state, barols_pos[0].clone(), dir);
            match next_state_opt {
                Some(y) => return Some(y),
                None => return None
                
            }
        }
        None => return None
        
    }
}

fn push_one_big_barol(current_state: &GameState, barol_pos: Vec<Coord>, dir: Direction)->Option<GameState>{
    let mov: i32;
    match dir {
        Direction::DOWN => mov=1,
        Direction::LEFT => mov=-1,
        Direction::UP => mov=-1,
        Direction::RIGHT => mov=1,
    }
    if dir == Direction::UP || dir == Direction::DOWN{
        let mut next_barol = barol_pos.clone();
        next_barol[0].y +=  mov as usize;
        next_barol[1].y +=  mov as usize;
        if current_state.map[next_barol[0].y][next_barol[0].x] == '#' ||
            current_state.map[next_barol[1].y][next_barol[1].x] == '#'{
                return None;
            }
        else if current_state.map[next_barol[0].y][next_barol[0].x] == '[' &&
            current_state.map[next_barol[1].y][next_barol[1].x] == ']'{
                let mut next_state = current_state.clone();
                next_state.map[barol_pos[0].y][barol_pos[0].x] = '.';
                next_state.map[barol_pos[1].y][barol_pos[1].x] = '.';
                return  push_one_big_barol(&next_state, next_barol, dir);
            }
        else if current_state.map[next_barol[0].y][next_barol[0].x] == '.' &&
            current_state.map[next_barol[1].y][next_barol[1].x] == '.'{
                let mut next_state = current_state.clone();
                next_state.map[barol_pos[0].y][barol_pos[0].x] = '.';
                next_state.map[barol_pos[1].y][barol_pos[1].x] = '.';
                next_state.map[next_barol[0].y][next_barol[0].x] = '[';
                next_state.map[next_barol[1].y][next_barol[1].x] = ']';
            }
        else if current_state.map[next_barol[0].y][next_barol[0].x] == ']' {
            let mut next_state = current_state.clone();
            next_state.map[barol_pos[0].y][barol_pos[0].x] = '.';
            next_state.map[barol_pos[1].y][barol_pos[1].x] = '.';
            let mut next_barol = next_barol.clone();
            next_barol[0].x +=  mov as usize;
            next_barol[1].x +=  mov as usize;
            let mut next_state = push_one_big_barol(&next_state, next_barol, dir)?;
            next_state.map[next_barol[0].y][next_barol[0].x] = '[';
            next_state.map[next_barol[0].y][next_barol[0].x+1] = ']';
            return Some(next_state);
        }
    }
    else{
        let mut next_barol_pos: Vec<Coord> = barol_pos.clone();
        match dir {
            Direction::LEFT => {
                next_barol_pos[1].x -= 2;
                next_barol_pos[0].x -= 1;
            },
            Direction::RIGHT => {
                next_barol_pos[1].x += 2;
                next_barol_pos[0].x += 1;
            }
            _ => panic!()

        }
        if current_state.map[next_barol_pos[1].y][next_barol_pos[1].x] == '#'{
            return None;
        }
        else if current_state.map[next_barol_pos[1].y][next_barol_pos[1].x] == '.'{
            let mut next_state = current_state.clone();
            next_state.map[next_barol_pos[1].y][next_barol_pos[1].x] = next_state.map[next_barol_pos[0].y][next_barol_pos[0].x];
            next_state.map[next_barol_pos[0].y][next_barol_pos[0].x] = next_state.map[barol_pos[1].y][barol_pos.x];
            return Some(next_state)
        }
        else if current_state.map[next_barol_pos[1].y][next_barol_pos[1].x] == '['{
            let mut next_barol_pos = current_barol_pos.clone();
            next_barol_pos.x +=2;
            let mut next_state= current_state.clone();
            next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
            next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
            next_state.player_pos = current_barol_pos;
            let next_state = push_big_barol(&next_state, next_barol_pos, dir);
            match next_state {
                Some(x)=> return Some(x),
                None => return  None
            }
        }
        else if current_state.map[next_barol_pos[1].y][next_barol_pos[1].x] == ']'{
            let mut next_barol_pos = current_barol_pos.clone();
            next_barol_pos.x -=2;
            let mut next_state= current_state.clone();
            next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
            next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
            next_state.player_pos = current_barol_pos;
            let next_state = push_big_barol(&next_state, next_barol_pos, dir);
            match next_state {
                Some(x)=> return Some(x),
                None => return  None
            }
        }
    }
    todo!()
}
// fn push_one_big_barol(current_state: &GameState, barol_pos: Vec<Coord>, dir: Direction)->Option<GameState>{
//     if dir is up or donw{
//         if barol next{
//             if 2 barols{
//                 push_two_big_barol();
//             }
//             else{
//                 push_one_big_barol();
//             }
//         }
//         else{
//             move_barol()
//         }
//     }
//     if dir is left or right{
//         if barol next{
//             push_one_big_barol();
//         }
//         else{
//             move_barol()
//         }
//     }
// }


fn push_big_barol(current_state: &GameState, current_barol_pos: Coord, dir: &Direction) -> Option<GameState>{
    let mut next_state = current_state.clone();
    next_state.player_pos = current_barol_pos;
    
    
    
    if [Direction::UP, Direction::DOWN].contains(dir){
        dbg!("Pushing barol ",dir);
        let next_barols_pos = get_next_barol_pos(current_barol_pos,  next_state.map[current_barol_pos.y][current_barol_pos.x], &dir);
        if next_barols_pos.iter().any(|next_barol_pos|current_state.map[next_barol_pos.y][next_barol_pos.x] == '#'){
                return None;
            }
        else if current_state.map[next_barols_pos[0].y][next_barols_pos[0].x] == '[' &&
            current_state.map[next_barols_pos[1].y][next_barols_pos[1].x] == ']'{
                let next_move_left = push_big_barol(current_state, next_barols_pos[0], dir);
        }
        else{
            let next_move_left = push_big_barol(current_state, next_barols_pos[0], dir);
            let next_move_right = push_big_barol(current_state, next_barols_pos[1], dir);
            if next_move_left.is_some() && next_move_right.is_some(){

            }
            return None;
        }
    }
    else{
        let mut next_barol_pos: Vec<Coord> = vec![current_barol_pos.clone();2];
        match dir {
            Direction::LEFT => {
                next_barol_pos[1].x -= 2;
                next_barol_pos[0].x -= 1;
            },
            Direction::RIGHT => {
                next_barol_pos[1].x += 2;
                next_barol_pos[0].x += 1;
            }
            _ => panic!()

        }
        if current_state.map[next_barol_pos[1].y][next_barol_pos[1].x] == '#'{
            return None;
        }
        else if current_state.map[next_barol_pos[1].y][next_barol_pos[1].x] == '.'{
            let mut next_state = next_state.clone();
            next_state.map[next_barol_pos[1].y][next_barol_pos[1].x] = next_state.map[next_barol_pos[0].y][next_barol_pos[0].x];
            next_state.map[next_barol_pos[0].y][next_barol_pos[0].x] = next_state.map[current_barol_pos.y][current_barol_pos.x];
            return Some(next_state)
        }
        else if current_state.map[next_barol_pos[1].y][next_barol_pos[1].x] == '['{
            let mut next_barol_pos = current_barol_pos.clone();
            next_barol_pos.x +=2;
            let mut next_state= current_state.clone();
            next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
            next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
            next_state.player_pos = current_barol_pos;
            let next_state = push_big_barol(&next_state, next_barol_pos, dir);
            match next_state {
                Some(x)=> return Some(x),
                None => return  None
            }
        }
        else if current_state.map[next_barol_pos[1].y][next_barol_pos[1].x] == ']'{
            let mut next_barol_pos = current_barol_pos.clone();
            next_barol_pos.x -=2;
            let mut next_state= current_state.clone();
            next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
            next_state.map[current_state.player_pos.y][current_state.player_pos.x] = '.';
            next_state.player_pos = current_barol_pos;
            let next_state = push_big_barol(&next_state, next_barol_pos, dir);
            match next_state {
                Some(x)=> return Some(x),
                None => return  None
            }
        }
    }
    
    panic!();
}

enum BarolSide{
    LEFT,
    RIGHT
}
fn get_next_barol_pos(mut position: Coord, barol_val: char, dir: &Direction) -> Vec<Coord>{
    let mut new_positions = vec![position.clone();2];
    match dir {
        Direction::UP => {
            new_positions[0].y -= 1;
            new_positions[1].y -= 1;
            if barol_val == '['{
                new_positions[1].x += 1;
            }
            else{
                new_positions[1].x -= 1;
            }
        },
        Direction::RIGHT => {
            new_positions[0].x += 1;
            if barol_val == '['{
                new_positions[1].x += 2;
            }
            else{
                panic!("Cannot push to the right a right part barol");
            }
        },
        Direction::DOWN => {
            new_positions[0].y += 1;
            if barol_val == '['{
                new_positions[1].x += 1;
                new_positions[1].y += 1;
            }
            else{
                new_positions[1].x -= 1;
                new_positions[1].y += 1;
            }
        },
        Direction::LEFT => {
            new_positions[0].x -= 1;
            if barol_val == '['{
                panic!("Cannot push to the left a left part barol");
            }
            else{
                new_positions[1].x -= 2;
            }
        },
    }
    dbg!(position,barol_val,&new_positions);
    return new_positions;
}
