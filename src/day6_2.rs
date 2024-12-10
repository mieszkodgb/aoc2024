use strum::{Display, EnumIter};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, Clone, EnumIter, Copy, Default, Display, PartialEq, Eq, Hash)]
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
    player: PlayerState,
    pub map: Vec<Vec<char>>,
    done: bool
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum PossibleMoves{
    #[default] MOVE,
    TURN
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move{
    p_move: PossibleMoves,
    p_coord: Coord
}

#[derive(Debug, Default, Clone)]
pub struct PlayerState{
    pub step: i32,
    pub position: Coord,
    pub history: Vec<Move>,
    pub dir: Direction

}

pub fn get_init_state(input_vec: &Vec<Vec<char>>) -> GameState{
    let possible_positions = vec!['^', '<', '>', 'v'];
    for (r, row) in input_vec.iter().enumerate(){
        for (c, val) in row.iter().enumerate(){
            // println!("{:?}", val);
            if possible_positions.contains(&val) {
                let direction = possible_positions.iter().find(|&x| x == val).unwrap();
                // println!("{:?}", direction);
                let dir: Direction;
                match direction {
                    '^' => dir = Direction::UP,
                    'v' => dir = Direction::DOWN,
                    '<' => dir = Direction::LEFT,
                    '>' => dir = Direction::RIGHT,
                    _ => panic!("Wrong direction")

                };
                let position = Coord{x:c, y:r};
                let player = PlayerState{
                    position:position,
                    step:0,
                    history: Vec::new(),
                    dir: dir
                };
                let game_state = GameState{
                    player:player,
                    map: input_vec.clone(),
                    done: false
                };
                return game_state;
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
fn turn(direction: Direction) -> Direction{
    // println!("Turning {}", direction);
    match direction {
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN
    }
}
fn is_blocked(input_vec: &Vec<Vec<char>>, next_position: Coord) -> bool{
    // print!("{:?}",input_vec[next_position.y][next_position.x]);
    // println!("{}x{} for {}x{}",input_vec[0].len(), input_vec.len(), next_position.y, next_position.y);
    return input_vec[next_position.y][next_position.x] == '#' ||
    input_vec[next_position.y][next_position.x] == 'O';
}

pub fn get_next_player_state(current_player: &PlayerState) -> PlayerState{
    let mut next_position = current_player.position.clone();
    let mut history = current_player.history.clone();
    match &current_player.dir {
        Direction::DOWN => next_position.y += 1,
        Direction::UP => next_position.y -= 1,
        Direction::LEFT => next_position.x -= 1,
        Direction::RIGHT => next_position.x += 1,
    }
    let next_move = Move{p_move:PossibleMoves::MOVE, p_coord:current_player.position};
    // history.push(next_move);

    let mut next_player : PlayerState = current_player.clone();
    next_player.history = history;
    next_player.position = next_position;
    return next_player
}

pub fn play_next(current_state: GameState) -> GameState{
    let mut next_state = current_state.clone();
    if (current_state.player.dir == Direction::UP && current_state.player.position.y == 0) || 
        (current_state.player.dir == Direction::LEFT && current_state.player.position.x == 0)
        {
            next_state.player.step += 1;
            next_state.player.history.push(Move{p_move:PossibleMoves::MOVE, p_coord:current_state.player.position});
            next_state.done = true;
            return  next_state;
        }
    let mut next_player = get_next_player_state(&current_state.player);
    // println!("{}-{} out of {}x{}",next_player.position.x, next_player.position.y, current_state.map[0].len(),current_state.map.len());
    if is_out_of_boundaries(next_player.position, current_state.map[0].len(), current_state.map.len()){
        // println!("Is out of bound {:?}", next_player.position);
        next_state.done = true;
        // next_player.history.pop();
        next_player.history.push(Move{p_move:PossibleMoves::MOVE, p_coord:current_state.player.position});
        next_player.step += 1;
    }
    else if is_blocked(&current_state.map, next_player.position){
        next_player = current_state.player.clone();
        next_player.dir = turn(next_player.dir);
        // let mut last_move: Move;
        // let last_move_opt = next_player.history.pop();
        // if last_move_opt.is_some(){
        //     last_move = last_move_opt.unwrap();
        //     last_move.p_move = PossibleMoves::TURN;
        //     next_player.history.push(last_move);
        // }
        next_player.history.push(Move{p_move:PossibleMoves::TURN, p_coord:current_state.player.position});
        next_state.map[next_player.position.y][next_player.position.x] = '+';
    }
    else if current_state.map[next_player.position.y][next_player.position.x] == '.'{
        next_state.map[next_player.position.y][next_player.position.x] = '^';
        next_player.history.push(Move{p_move:PossibleMoves::MOVE, p_coord:current_state.player.position});
        next_player.step += 1;
    }
    if  next_state.map[current_state.player.position.y][current_state.player.position.x] != '+'{
        next_state.map[current_state.player.position.y][current_state.player.position.x] = 'X';
    }

    next_state.player = next_player;
    return next_state;
}


fn check_loop(history: &Vec<Move>) -> bool{
    // println!("Check loop");
    let hist_filt: Vec<&Move> = history.iter().filter(|m| m.p_move==PossibleMoves::TURN).collect();
    let moves_counter: bool = hist_filt.iter()
        .map(|n| (n, hist_filt.iter().filter(|&m| (m == n && m.p_move==PossibleMoves::TURN)).count()))
        .any(|(m, s)| m.p_move==PossibleMoves::TURN && s >= 4);
    // println!("Loop Counter {}", hist_filt.len());
    if moves_counter{
        return true;
    }
    return false;
}


pub fn part_one(input_vec: &Vec<Vec<char>>) -> GameState{
    let mut game_state = get_init_state(input_vec);

    while game_state.done == false {
        // println!("------ New turn --------");
        game_state = play_next(game_state);
        // println!("{}", map_to_string(&game_state.map));
        // println!("Current step {}", game_state.player.step);
        // println!("------------------------");
    }
    // println!("{}", get_steps_from_history(&game_state.player.history));
    println!("Steps: {}", game_state.player.step);
    return game_state;
}


pub fn part_two(input_vec: &Vec<Vec<char>>) -> i32{
    let mut init_game_state = get_init_state(input_vec);

    let mut loops = 0;
    
    let basic_state = part_one(input_vec);
    // println!("Basic done {:?}", basic_state.player.history);
    // let hist_unique:Vec<Move> = basic_state.player.history.clone().into_iter()
    //     .filter(|moves| moves.p_move == PossibleMoves::MOVE)
    //     .map(|moves| moves)
    //     .collect::<HashSet<_>>()
    //     .into_iter()
    //     .collect();
    let hist_unique:Vec<Move> = basic_state.player.history.into_iter()
        .filter(|moves| moves.p_move == PossibleMoves::MOVE)
        .collect();
    // println!("Hist size {} without turns {}", basic_state.player.history.len(),hist_unique2.len());
    for (k, moves_counter) in hist_unique.iter().enumerate(){
        println!("New Map {:?} from {}/{}", moves_counter.p_coord, k, hist_unique.len()-1);
        let mut new_game_state = init_game_state.clone();
        if moves_counter.p_coord.x != init_game_state.player.position.x || moves_counter.p_coord.y != init_game_state.player.position.y{
            new_game_state.map[moves_counter.p_coord.y][moves_counter.p_coord.x] = 'O';
            // println!("{}",map_to_string(&new_game_state.map));
            while new_game_state.done == false {
                // println!("{:?}", current_state.position);
                new_game_state = play_next(new_game_state);
                if new_game_state.player.history.len() > 4 && new_game_state.player.history.last().unwrap().p_move == PossibleMoves::TURN{
                // if new_game_state.player.history.iter()
                //     .find(|&p_move| p_move==new_game_state.player.history.last().unwrap())
                //     .is_some(){
                    // print!("Check loop ");
                    if check_loop(&new_game_state.player.history){
                        loops+=1;
                        println!("Loop found {}", loops);
                        // println!("{}",map_to_string(&new_game_state.map));
                        break;
                    }
                }
            }
            // println!("{}",map_to_string(&new_game_state.map));
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

