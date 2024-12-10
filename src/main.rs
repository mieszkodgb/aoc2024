// use reqwest::Error;
mod utils;
// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
mod day6_2;
// mod day6_visu;
// mod day_7;
mod day_9;

#[tokio::main]
async fn main() -> (){ //Box<dyn Error>>
    let day = "9";
    let year = "2024";
    let cookie = "53616c7465645f5f8314147db14eb79bcc737912132b1bb1f13c6e013f0f8deff934838c8f947722ec154ce0dc2d3794d3daf8c94b3a1f1d0cc9cc1f1dc16184";
    let http_api= format!("https://adventofcode.com/{year}/day/{day}/input");

    // let res = utils::read_api(&http_api, &cookie).await;
    let res = utils::read_file("dummy_input.txt");

    let mut res = utils::convert_str_to_vec(res);
    // res.pop();

    // println!("{:?}",res[0].clone());
    // day6_2::part_two(&res);

    // day6_visu::play_tui(res);


    // day_7::part_one2(res);
    day_9::part_one(res[0].clone());
}
