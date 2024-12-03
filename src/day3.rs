use regex::Regex;

fn find_regex(string: &str, regex_pattern: &str)-> Vec<String>{
    let pattern = Regex::new(regex_pattern).unwrap();
    let matches = pattern.find_iter(string);
    // let matches2 = pattern.find_iter(string);
    // for val in matches2{
    //     println!("{:?}", val.as_str());
    // }
    return matches.map(|v| v.as_str().to_owned()).collect::<Vec<String>>()
}

pub fn part_one(input_vec: String){
    let instr = find_regex(&input_vec, r"mul\(\d{1,4}\,\d{1,4}\)");

    let vals: Vec<i32> = instr.into_iter()
        .map(|v| find_regex(&v, r"\d{1,4}").into_iter()
        .map(|w,| w.parse::<i32>().unwrap()).collect()).collect::<Vec<Vec<i32>>>().into_iter()
        .map(|x:Vec<i32>| x[0]*x[1]).collect();

    println!("{}",vals.into_iter().sum::<i32>());
}

pub fn part_two(input_vec: String){
    let instructions = find_regex(&input_vec, r"mul\(\d{1,4}\,\d{1,4}\)|don?\'?t?\(\)");
    let mut run_val = true;
    let mut collector = 0;
    for instr in instructions{
        match instr.as_str() {
            "do()" => run_val=true,
            "don't()" => run_val=false,
            _ => {
                if run_val{
                    collector += find_regex(&instr, r"\d{1,4}").into_iter()
                    .map(|w,| w.parse::<i32>().unwrap()).collect::<Vec<i32>>().into_iter().product::<i32>()
                }
            }
        };
        
    }
    println!("{}",collector);
}
