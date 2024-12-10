use std::collections::HashSet;

use strum::{Display, EnumIter};

#[derive(Debug, Clone, EnumIter, Copy, Default, Display, PartialEq, Eq, Hash)]
pub enum direction{
    UP,
    RIGHT,
    #[default] DOWN,
    LEFT,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Hash)]
pub struct Coord{
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Default, Clone)]
pub struct GameState{
    player: PlayerState,
    done: bool
}

#[derive(Debug, Default, Clone)]
pub struct PlayerState{
    pub step: i32,
    pub position: Coord,
    pub map: Vec<Vec<char>>,
    pub history: Vec<Coord>,

}

pub fn get_current_location(input_vec: &Vec<Vec<char>>) -> Coord{
    let possible_positions = vec!['^', '<', '>', 'v'];
    for (r, row) in input_vec.iter().enumerate(){
        for (c, val) in row.iter().enumerate(){
            // println!("{:?}", val);
            if possible_positions.contains(&val) {
                let direction = possible_positions.iter().find(|&x| x == val).unwrap();
                // println!("{:?}", direction);
                let dir: direction;
                match direction {
                    '^' => dir = direction::UP,
                    'v' => dir = direction::DOWN,
                    '<' => dir = direction::LEFT,
                    '>' => dir = direction::RIGHT,
                    _ => panic!("Wrong direction")

                };

                let position = Coord{x:c, y:r, dir:dir};
                // println!("{:?}", position);
                return position;
            }
        }
    }
    panic!("No location found");
}

fn is_out_of_boundaries(position: Coord, max_x:usize,max_y: usize) -> bool{
    // println!("Check Boundaries");
    return (position.x < 0 || position.y < 0) || 
    (position.x > max_x-1 || position.y > max_y-1);
}
fn turn(direction: direction)->direction{
    println!("Turning {}", direction);
    match direction {
        direction::DOWN => direction::LEFT,
        direction::LEFT => direction::UP,
        direction::UP => direction::RIGHT,
        direction::RIGHT => direction::DOWN
    }
}
fn is_blocked(input_vec: &Vec<Vec<char>>, next_position: Coord) -> bool{
    // print!("{:?}",input_vec[next_position.y][next_position.x]);
    return input_vec[next_position.y][next_position.x] == '#' ||
    input_vec[next_position.y][next_position.x] == 'O';
}

pub fn get_next_state(current_state: &PlayerState) -> PlayerState{
    let mut current_position = current_state.position;
    let mut next_position = current_position.clone();
    let mut history = current_state.history.clone();
    match &next_position.dir {
        direction::DOWN => next_position.y += 1,
        direction::UP => next_position.y -= 1,
        direction::LEFT => next_position.x -= 1,
        direction::RIGHT => next_position.x += 1,
    }
    history.push(next_position);
    let next_state: PlayerState = PlayerState{position:next_position, 
        step:current_state.step,
        map:current_state.map.clone(),
        history:history
    };
    return next_state
}

pub fn play_next2(current_state: GameState) -> GameState{
    let mut next_state = get_next_state(&current_state.player);
    if is_out_of_boundaries(next_state.position, next_state.map[0].len(), next_state.map.len()){
        println!("Is out of bound {:?}", next_state.position);
        return GameState{player: current_state.player, done:true};
    }
    else if is_blocked(&next_state.map, next_state.position){
        next_state = current_state.player.clone();
        next_state.position.dir = turn(next_state.position.dir);
        next_state.map[next_state.position.y][next_state.position.x] = '^';
    }
    else if current_state.player.map[next_state.position.y][next_state.position.x] == '.'{
        next_state.step += 1;
    }
    
    next_state.map[current_state.player.position.y][current_state.player.position.x] = 'X';
    next_state.map[next_state.position.y][next_state.position.x] = '^';
    return GameState{player: next_state, done:false};
}
pub fn play_next(current_state: PlayerState) -> PlayerState{
    let mut map = current_state.map.clone();
    let mut current_position = current_state.position;
    let mut step = current_state.step;
    let mut next_position = current_position.clone();
    let mut history = current_state.history.clone();
    // println!("First next {:?}", map[next_position.y][next_position.x]);
    match &next_position.dir {
        direction::DOWN => next_position.y += 1,
        direction::UP => next_position.y -= 1,
        direction::LEFT => next_position.x -= 1,
        direction::RIGHT => next_position.x += 1,
    }
    // println!("Second next {:?}", next_position);
    if is_out_of_boundaries(next_position, current_state.map[0].len(),current_state.map.len()){
        // println!("Out of boundaries");
        // println!("{:?} {} {}", next_position, current_state.map[0].len(), current_state.map.len());
        // history.push(current_position);
        let next_state: PlayerState = PlayerState{position:next_position, step:step+1, map:map, history:history};
        return next_state;
    }
    if is_blocked(&map, next_position) {
        // println!("Blocked next {:?}", next_position);
        next_position = current_position.clone();
        next_position.dir = turn(next_position.dir);
        map[current_position.y][current_position.x] = '^';
    }
    else{
        // println!("Third next {:?}", next_position);
        if map[next_position.y][next_position.x] == '.'{
            step += 1;
        }
        map[current_position.y][current_position.x] = 'X';
        map[next_position.y][next_position.x] = '^';
        // current_position = next_position.clone();
        // history.push(current_position);
    } 
    // println!("Fourth next {:?}", next_position);
    let next_state: PlayerState = PlayerState{position:next_position, step:step, map:map, history:history};
    return next_state;
}

fn check_loop(history: &Vec<Coord>) -> bool{
    for (w1, win1) in history.windows(2).enumerate(){
        for (w2, win2) in history.windows(2).enumerate(){
            if w1 != w2 && win1==win2 {
                // println!("loop found");
                return true;
            }
        }
    }
    return false;
}

pub fn get_steps_from_history(history: &Vec<Coord>) -> usize{
    let hist = history.clone();
    hist.into_iter()
        .map(|pos| pos)
        .collect::<HashSet<Coord>>()
        .into_iter()
        .len()
}

pub fn part_one2(input_vec: &Vec<Vec<char>>) -> PlayerState{
    let mut current_position = get_current_location(input_vec);
    let mut step_count = 0;
    let mut player_state: PlayerState = PlayerState{position:current_position, 
        step:step_count, 
        map:input_vec.clone(),
        history:Vec::new()
};
    let mut game_state: GameState = GameState{player:player_state, done:false};

    while game_state.done == false {
        println!("------ New turn --------");
        game_state = play_next2(game_state);
        println!("{}", map_to_string(&game_state.player.map));
        println!("Current step {}", game_state.player.step);
        println!("------------------------");
    }
    let steps = 
    // println!("{}", get_steps_from_history(&game_state.player.history));
    return game_state.player;
}
pub fn part_one(input_vec: &Vec<Vec<char>>) -> PlayerState{
    let mut current_position = get_current_location(input_vec);
    // println!("First position {:?}", input_vec[current_position.y][current_position.x]);
    let mut step_count = 0;
    let mut current_state: PlayerState = PlayerState{position:current_position, 
            step:step_count, 
            map:input_vec.clone(),
            history:Vec::new()
    };
    while ! is_out_of_boundaries(current_state.position, input_vec[0].len(),input_vec.len()) {
        println!("{:?}", current_state.position);
        println!("{:?}", is_out_of_boundaries(current_state.position, input_vec[0].len(),input_vec.len()));
        current_state = play_next(current_state);
        current_state.history.push(current_state.position);
    }
    current_state.step += 1;
   
    println!("{}", current_state.step);
    return  current_state;
}


pub fn part_two(input_vec: &Vec<Vec<char>>) -> i32{
    let mut init_position = get_current_location(input_vec);
    // println!("First position {:?}", input_vec[current_position.y][current_position.x]);
    let mut loops = 0;
    
    let basic_state = part_one(input_vec);
    println!("Basic done {:?}", basic_state.history);
    for (k, coord) in basic_state.history.iter().enumerate(){
        println!("New Map {:?} from {}/{}", coord, k, basic_state.history.len());
        let current_position = init_position.clone();
        if coord.x != current_position.x || coord.y != current_position.y{
            let mut map_modif = input_vec.clone();
            let mut current_state: PlayerState = PlayerState{position:current_position, 
                step:0, 
                map:input_vec.clone(),
                history:Vec::new()
        };
            map_modif[coord.y][coord.x] = 'O';
            current_state.map = map_modif;
            println!("{}",map_to_string(&current_state.map));
            while ! is_out_of_boundaries(current_state.position, input_vec[0].len(),input_vec.len()) {
                // println!("{:?}", current_state.position);
                if current_state.history.iter().find(|&pos| pos==&current_state.position).is_some(){
                    print!("Check loop ");
                    if check_loop(&current_state.history){
                        loops+=1;
                        println!("Loop found {}", loops);
                        break;
                    }
                }
                
                current_state = play_next(current_state);
            }
        }
    }
   
    println!("{}", loops);
    return  loops;
}

pub fn map_to_string(raw_map : &Vec<Vec<char>>) -> String{
    let map_str = raw_map
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n");
    return  map_str;
}

