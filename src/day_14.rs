use std::{thread::sleep, time};

use colored::Color;
#[derive(Debug, Clone, Copy)]
struct Cood{
    x: i32,
    y: i32
}

#[derive(Debug, Clone, Copy)]
struct Direction{
    x: i32,
    y: i32
}

#[derive(Debug, Clone, Copy)]
struct Robot{
    pos: Cood,
    vec: Direction
}
pub fn part_one(input: &Vec<String>){
    
    let mut robots: Vec<Robot> = Vec::with_capacity(input.len());
    for row in input{
        let mut data = row.split(" ");
        let mut position_iter = data.clone().next().unwrap()
            .split("=");
        position_iter.next();
        let position = position_iter.next().unwrap().split(",")
            .map(|val|val.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        data.next();
        let mut direction_iter = data.clone().next().unwrap()
            .split("=");
        direction_iter.next();
        let direction = direction_iter.next().unwrap().split(",")
            .map(|val|val.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        robots.push(Robot{pos:Cood{x:position[0],y:position[1]}, vec:Direction{x:direction[0], y:direction[1]}});
    }
    // dbg!(&robots);
    let max_x: i32 = 101;
    let max_y = 103;
    for k in 0..max_x*max_y+1{
        let mut new_robots: Vec<Robot> = Vec::new();
        // show(robots.clone(), max_x, max_y);
        for robot in robots.iter_mut(){
            // dbg!("st ",&robot);
            robot.pos.x += robot.vec.x;
            robot.pos.y += robot.vec.y;
            
            if robot.pos.x > max_x-1{
                robot.pos.x -= max_x;
            }
            if robot.pos.x < 0{
                robot.pos.x += max_x;
            }
            if robot.pos.y > max_y-1{
                robot.pos.y -= max_y;
            }
            if robot.pos.y < 0{
                robot.pos.y += max_y;
            }
            // dbg!("end ",robot.pos);
            new_robots.push(robot.clone());
        }
        robots = new_robots;
        let quad_val = find_perquadrant(robots.clone(), max_x, max_y);
        // if quad_val[0] == quad_val[1] && quad_val[2] == quad_val[3]{
            println!("Iteration {}",k);
            show(robots.clone(), max_x, max_y);
            let millis = time::Duration::from_millis(200);
            // sleep(millis);
        // }
    }
    // dbg!(robots[robots.clone().len()-2]);
    let quad_val = find_perquadrant(robots, max_x, max_y);
    dbg!(quad_val.iter().product::<i32>());
}

fn find_perquadrant(robots: Vec<Robot>, max_x: i32, max_y: i32) -> Vec<i32>{

    let mut val_per_quadrant = vec![0;4];
    // println!("Quadran mid {:?}x - {:?}y", max_x/2,max_y/2);
    for robot in robots{
        // dbg!(robot.pos);
        if robot.pos.x > max_x/2 && robot.pos.y > max_y/2{
            val_per_quadrant[2] += 1;
        }
        else if robot.pos.x > max_x/2 && robot.pos.y < max_y/2{
            val_per_quadrant[1] += 1;
        }
        else if robot.pos.x < max_x/2 && robot.pos.y < max_y/2{
            val_per_quadrant[0] += 1;
        }
        else if robot.pos.x < max_x/2 && robot.pos.y > max_y/2{
            val_per_quadrant[3] += 1;
        }
        else {
            // println!("Noting for {:?}", robot.pos);
        }
    }
    return  val_per_quadrant;
}

fn show(robots: Vec<Robot>, max_x: i32, max_y: i32){
    let mut map = vec![vec!['.';max_x as usize];max_y as usize];
    for robot in robots{
        // dbg!(robot.pos);
        let current = map[robot.pos.y as usize][robot.pos.x as usize];
        
        if current == '.'{
            map[robot.pos.y as usize][robot.pos.x as usize] = '1';
        }
        if current == '1'{
            map[robot.pos.y as usize][robot.pos.x as usize] = '2';
        }
        if current == '2'{
            map[robot.pos.y as usize][robot.pos.x as usize] = '3';
        }
    }
    let beauti_map: Vec<String> = map.iter().map(|row| row.iter().collect::<String>()).collect();
    // println!("{:?}",beauti_map);
    dbg!(beauti_map);
}