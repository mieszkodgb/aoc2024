use std::collections::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord{
    x: u8,
    y: u8
}
fn find_start(input_vec: &Vec<Vec<u32>>) -> Vec<Coord>{
    let start_points: Vec<Coord>;
    start_points = input_vec.iter()
        .enumerate()
        .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, v)| (x, y, v)))
        .filter_map(|(y,x, &v)| {
            if v==0 as u32{
                Some(Coord{x:x as u8, y:y as u8})
            }
            else{
                None
            }
        }).collect::<Vec<Coord>>();
    return start_points;
}

fn find_next(input_vec: &Vec<Vec<u32>>, current: Coord,mut counter: Vec<Coord>) -> Vec<Coord>{
    let current_val = input_vec[current.y as usize][current.x as usize];
    // println!("Current {:?} at {:?}", current_val, current);
    if current_val == 9{
        // println!("Found9 at {:?}",current);
        counter.push(current);
        return counter;
    }

    if current.x + 1 <= (input_vec[0].len()-1) as u8{
        // println!("Try right");
        let next = Coord{x:current.x+1, y:current.y};
        let next_val = input_vec[next.y as usize][next.x as usize];
        if next_val == current_val + 1{
            // println!("Next val right");
            counter = find_next(input_vec, next, counter);
        }
        // else{
        //     return 0;
        // }
    }
    if current.x  >= 1{
        let next = Coord{x:current.x-1, y:current.y};
        let next_val = input_vec[next.y as usize][next.x as usize];
        if next_val == current_val + 1{
            // println!("Next val left");
            counter = find_next(input_vec, next, counter);
        }
        // else{
        //     return 0;
        // }
    }
    if current.y + 1 <= (input_vec.len()-1) as u8{
        let next = Coord{x:current.x, y:current.y+1};
        let next_val = input_vec[next.y as usize][next.x as usize];
        if next_val == current_val + 1{
            // println!("Next val down");
            counter = find_next(input_vec, next, counter);
        }
        // else{
        //     return 0;
        // }
    }
    if current.y  >= 1{
        let next = Coord{x:current.x, y:current.y-1};
        let next_val = input_vec[next.y as usize][next.x as usize];
        if next_val == current_val + 1{
            // println!("Next val up");
            counter = find_next(input_vec, next, counter);
        }
        // else{
        //     return 0;
        // }
    }
    // println!("Back");
    return counter;
}
pub fn part_one(input_vec: &Vec<Vec<u32>>){
    let mut num_trails = 0;
    let all_starts = find_start(&input_vec);
    println!("There are {} starts", all_starts.len());
    for start in all_starts.iter(){
        let mut counter = Vec::new();
        // let start_val = input_vec[start.y as usize][start.x as usize];
        // println!("Start {:?} at {:?}", start_val, start);
        counter = find_next(&input_vec, *start, counter);
        // print!("Start {:?}, with {:?} paths", start, counter.len());
        // println!(" with {:?} unique ends", counter.iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>().len());
        num_trails += counter.iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>().len();
    }
    println!("{:?}", num_trails);
}

pub fn part_two(input_vec: &Vec<Vec<u32>>){
    let mut num_trails = 0;
    let all_starts = find_start(&input_vec);
    println!("There are {} starts", all_starts.len());
    for start in all_starts.iter(){
        let mut counter = Vec::new();
        // let start_val = input_vec[start.y as usize][start.x as usize];
        // println!("Start {:?} at {:?}", start_val, start);
        counter = find_next(&input_vec, *start, counter);
        // print!("Start {:?}, with {:?} paths", start, counter.len());
        // println!(" with {:?} unique ends", counter.iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>().len());
        // num_trails += counter.iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>().len();
        num_trails += counter.len()
    }
    println!("{:?}", num_trails);
}