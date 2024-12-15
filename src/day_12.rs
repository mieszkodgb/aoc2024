
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord{
    x: usize,
    y: usize
}

type Group = Vec<Coord>;

pub fn part_one(input_vec: Vec<Vec<char>>){
    let start_vec = input_vec.clone();
    let all_groups = get_all_groups(start_vec);
    let price: i32 = all_groups.iter().map(|group| get_stat(group)).sum();
    println!("Price is {price}");
}

pub fn part_two(input_vec: Vec<Vec<char>>){
    let start_vec = input_vec.clone();
    let all_groups = get_all_groups(start_vec);
    let price: i32 = all_groups.iter().map(|group| get_stat_discount(group)).sum();
    println!("Price is {price}");
}

fn get_all_groups(mut start_vec: Vec<Vec<char>>)-> Vec<Group>{
    let mut groups: Vec<Group> = Vec::new();
    let mut start_opt= start_vec.iter()
        .enumerate()
        .flat_map(|(x, v)| v.iter().enumerate().filter(|(_, &val)|val!='.').map(move |(y, v)| (Coord{x:x, y:y}, v)))
        .next();
    while start_opt.is_some() {
        // println!("start at {:?}", start_opt);
        let (start_pos, _) = start_opt.unwrap();
        let mut new_group = vec![start_pos];
        let new_group = find_group(&mut start_vec, &mut new_group);
        groups.push(new_group);
        start_opt = start_vec.iter()
        .enumerate()
        .flat_map(|(y, v)| v.iter().enumerate().filter(|(_, &val)|val!='.').map(move |(x, v)| (Coord{x:x, y:y}, v)))
        .next();
    }
    // println!("Map end {:?}", start_vec);
    return groups;

}

fn find_group(current_vec: &mut Vec<Vec<char>>,current_group: &mut Group)-> Group{
    let current_coord = *current_group.last().unwrap();
    // println!("Map is {:?} start at {:?}", current_vec, current_coord);
    let val = current_vec[current_coord.y][current_coord.x];
    if val == '.'{
        // println!("Val is '.'");
        return current_group.clone();
    }
    current_vec[current_coord.y][current_coord.x] = '.';
    if current_coord.x+1 < current_vec[0].len() {
        let next_val = current_vec[current_coord.y][current_coord.x+1];
        // println!("Try Right {:?} {:?}", val, next_val);
        if next_val == val{
            current_group.push(Coord{x:current_coord.x+1, y:current_coord.y});
            find_group(current_vec, current_group);
        }
    }
    if current_coord.x > 0{
        let next_val = current_vec[current_coord.y][current_coord.x-1];
        if next_val == val{
            // println!("Try Left {:?} {:?}", val, next_val);
            current_group.push(Coord{x:current_coord.x-1, y:current_coord.y});
            find_group(current_vec, current_group);
        }
    }
    if current_coord.y+1 < current_vec[0].len(){
        let next_val = current_vec[current_coord.y+1][current_coord.x];
        if next_val == val{
            // println!("Try Down {:?} {:?}", val, next_val);
            current_group.push(Coord{x:current_coord.x, y:current_coord.y+1});
            find_group(current_vec, current_group);
        }
    }
    if current_coord.y > 0{
        let next_val = current_vec[current_coord.y-1][current_coord.x];
        if next_val == val{
            // println!("Try Up {:?} {:?}", val, next_val);
            current_group.push(Coord{x:current_coord.x, y:current_coord.y-1});
            find_group(current_vec, current_group);
        }
    }
    // println!("current group {:?}", current_group);
    return current_group.clone();
}

fn get_perimeter(group: &Group) -> i32{
    let mut perimeter = 0;
    // println!("There are {} in group", group.len());
    for val in group{
        let mut val_peri = 0;
        let has_down = group.iter().find(|other_val| other_val.x == val.x && other_val.y == val.y+1).is_some();
        let has_up = val.y > 0 && group.iter().find(|other_val| other_val.x == val.x && other_val.y == val.y-1).is_some();
        let has_left = val.x > 0 && group.iter().find(|other_val| other_val.y == val.y && other_val.x == val.x-1).is_some();
        let has_right = group.iter().find(|other_val| other_val.y == val.y && other_val.x == val.x+1).is_some();
        // println!("{:?}:  {} {} {} {}",val, has_up, has_right, has_down, has_left);
        if !has_left{
            val_peri+=1;
        }
        if !has_right{
            val_peri+=1
        }
        if !has_up{
            val_peri+=1
        }
        if !has_down{
            val_peri+=1
        }
        // println!("Perimeter of val is {:?}", val_peri);
        perimeter += val_peri;
        
    }
    // println!("Perimeter is {:?}", perimeter);
    return perimeter;
}

fn get_corners(group: &Group) -> i32{
    let mut corners = 0;
    // println!("There are {} in group", group.len());
    for val in group{
        let has_down = group.iter().find(|other_val| other_val.x == val.x && other_val.y == val.y+1).is_some();
        let has_up = val.y > 0 && group.iter().find(|other_val| other_val.x == val.x && other_val.y == val.y-1).is_some();
        let has_left = val.x > 0 && group.iter().find(|other_val| other_val.y == val.y && other_val.x == val.x-1).is_some();
        let has_right = group.iter().find(|other_val| other_val.y == val.y && other_val.x == val.x+1).is_some();
        let mut ext_val_corners = 0;
        if !(has_up || has_left){
            ext_val_corners+=1;
        }
        if !(has_up || has_right){
            ext_val_corners+=1;
        }
        if !(has_down || has_left){
            ext_val_corners+=1;
        }
        if !(has_down || has_right){
            ext_val_corners+=1;
        }

        
        let mut int_val_corners = 0;
        if has_up && has_right{
            let has_ur = group.iter().find(|other_val| other_val.x == val.x+1 && other_val.y == val.y-1).is_some();
            if !has_ur{
                int_val_corners += 1;
            }
        }
        if has_up && has_left{
            let has_ul = group.iter().find(|other_val| other_val.x == val.x-1 && other_val.y == val.y-1).is_some();
            if !has_ul{
                int_val_corners += 1;
            }
        }
        if has_down && has_right{
            let has_dr = group.iter().find(|other_val| other_val.x == val.x+1 && other_val.y == val.y+1).is_some();
            if !has_dr{
                int_val_corners += 1;
            }
        }
        if has_down && has_left{
            let has_dl = group.iter().find(|other_val| other_val.x == val.x-1 && other_val.y == val.y+1).is_some();
            if !has_dl{
                int_val_corners += 1;
            }
        }
        corners += ext_val_corners;
        corners += int_val_corners;

    }
    // println!("Perimeter is {:?}", perimeter);
    return corners;
}

fn get_stat(group: &Group) -> i32{
    let area = group.len();
    let perimeter = get_perimeter(group);
    // println!("Area of {} and perimeter of {}", area, perimeter);
    return area as i32 * perimeter

}
fn get_stat_discount(group: &Group) -> i32{
    let area = group.len();
    let corners = get_corners(group);
    // println!("Area of {} and corners of {}", area, corners);
    return area as i32 * corners

}