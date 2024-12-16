use colored::Colorize;
use ratatui::layout::Offset;
pub fn part_one(input_vec: Vec<String>){
    let mut token: i32 = 0;
    let mut button_ax: i32 = 0;
    let mut button_ay: i32 = 0;
    let mut button_bx: i32 = 0;
    let mut button_by: i32 = 0;
    let mut price_x: i32 = 0;
    let mut price_y: i32 = 0;
    for (idx, row) in input_vec.iter().enumerate(){

        if idx%4 == 0{
            let row_splitted: Vec<&str> = row.split(",").collect();
            button_ax = row_splitted[0][row_splitted[0].len()-3..].parse::<i32>().unwrap();
            button_ay = row_splitted[1][row_splitted[1].len()-3..].parse::<i32>().unwrap();
            // println!("Button a {}-{}",button_ax, button_ay);

        }
        else if idx%4==1{
            let row_splitted: Vec<&str> = row.split(",").collect();
            button_bx = row_splitted[0][row_splitted[0].len()-3..].parse::<i32>().unwrap();
            button_by = row_splitted[1][row_splitted[1].len()-3..].parse::<i32>().unwrap();
            // println!("Button b {}-{}",button_bx, button_by);

        }
        else if idx%4==2{
            // dbg!(row.split(",").map(|val|val.split("=").last().unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>());
            let row_splitted: Vec<i32> = row.split(",").map(|val|val.split("=").last().unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
            price_x = row_splitted[0];
            price_y = row_splitted[1];
            // println!("{:?}",row_splitted);

        }
        
        else if button_ax !=0{
            for i in 0..101{
                for j in 0..101{
                    // println!("{} == {}", button_ax*i + button_bx*j, price_x);
                    // println!("{} == {}", button_ay*i + button_by*j, price_y);
                    if button_ax*i + button_bx*j == price_x && button_ay*i + button_by*j == price_y {
                        print!("For {},{}", i, j);
                        print!(" {} == {} ", button_ax*i + button_bx*j, price_x);
                        println!("{} == {}", button_ay*i + button_by*j, price_y);
                        token += 3*i;
                        token += 1*j;
                    }
                }
            }

        }        
    }
    println!("Tokens: {}", token);

 }


 pub fn part_two(input_vec: Vec<String>){
    let offset: i64 = 10000000000000;
    let mut token: i64 = 0;
    let mut button_ax: i64 = 0;
    let mut button_ay: i64 = 0;
    let mut button_bx: i64 = 0;
    let mut button_by: i64 = 0;
    let mut price_x: i64 = 0;
    let mut price_y: i64 = 0;
    for (idx, row) in input_vec.iter().enumerate(){

        if idx%4 == 0{
            let row_splitted: Vec<&str> = row.split(",").collect();
            button_ax = row_splitted[0][row_splitted[0].len()-3..].parse::<i64>().unwrap();
            button_ay = row_splitted[1][row_splitted[1].len()-3..].parse::<i64>().unwrap();
            // println!("Button a {}-{}",button_ax, button_ay);

        }
        else if idx%4==1{
            let row_splitted: Vec<&str> = row.split(",").collect();
            button_bx = row_splitted[0][row_splitted[0].len()-3..].parse::<i64>().unwrap();
            button_by = row_splitted[1][row_splitted[1].len()-3..].parse::<i64>().unwrap();
            // println!("Button b {}-{}",button_bx, button_by);

        }
        else if idx%4==2{
            // dbg!(row.split(",").map(|val|val.split("=").last().unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>());
            let row_splitted: Vec<i64> = row.split(",").map(|val|val.split("=").last().unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();
            price_x = row_splitted[0];
            price_y = row_splitted[1];
            // println!("{:?}",row_splitted);

        }
        else if button_ax != 0{ 
            // let mut button_ax: f64 = button_ax as f64;
            // let mut button_ay: f64 = button_ay as f64;
            // let mut button_bx: f64 = button_bx as f64;
            // let mut button_by: f64 = button_by as f64;
            // let mut price_x: f64 = price_x as f64;
            // let mut price_y: f64 = price_y as f64;
            // let mut offset: f64 = offset as f64;
            let ratio_a = button_ax / button_ay;
            
            let i = (price_x*button_by - price_y*button_bx + offset*(button_by-button_bx))/(button_ax*button_by - button_ay*button_bx);
            let j = (price_x*button_ay - price_y*button_ax + offset*(button_ay-button_ax))/(button_bx*button_ay - button_by*button_ax);

           
                print!("For {}-{}", i, j);
                // let i = i.round();
                // let j = j.round();
                print!(" to {}-{}", i, j);
                print!(" {} == {} ", button_ax*i + button_bx*j, price_x+offset);
                print!("{} == {} ", button_ay*i + button_by*j, price_y+offset);
                
                if button_ax*i + button_bx*j == price_x+offset && button_ay*i + button_by*j == price_y+offset {
                    println!("{}", "OK".green());
                    token += 3*i;
                    token += 1*j;
                    
                }
                else{
                    println!("{}", "NOPE".red());
                }
        
            
            
        }
        
        // else if button_ax !=0{
        //     for i in 0..10001{
        //         for j in 0..10001{
        //             // println!("{} == {}", button_ax*i + button_bx*j, price_x);
        //             // println!("{} == {}", button_ay*i + button_by*j, price_y);
        //             if button_ax*i + button_bx*j == price_x && button_ay*i + button_by*j == price_y {
        //                 println!("Price");
        //                 token += 3*i;
        //                 token += 1*j;
        //             }
        //         }
        //     }

        // }        
    }
    println!("Tokens: {}", token);

 }