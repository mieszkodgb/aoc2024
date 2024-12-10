

fn string_to_list(input: &String){
    let input_vec:Vec<usize> = input.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect();
    let vec_size: usize = input_vec.iter().sum();
    println!("{vec_size}");
    let mut layout: Vec<char> = vec!['.'; vec_size];// Vec::with_capacity(vec_size);
    let mut current_st: usize = 0;
    for (id, val) in input_vec.iter().enumerate(){
        println!("{}-{} for val {}",current_st,current_st+val, val);

        if id%2 == 0{
            
            layout[current_st..current_st+val].fill((id/2).to_string().chars().next().unwrap());
        }
        else{
            layout[current_st..current_st+val].fill('.');
        }
        current_st += val;
    }
    for id in 0..input_vec.len(){

        if(layout[id]) == '.'{
            let pos = *layout.iter().rfind(|val| val.is_numeric()).unwrap();
            
        }
    }
    dbg!(layout);
}
pub fn part_one(input: String){
    string_to_list(&input);
}