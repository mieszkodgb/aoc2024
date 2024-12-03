
use reqwest::Error;
// use std::error::Error;
mod utils;
mod day1;
mod day2;
mod day3;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let day = "3";
    let year = "2024";
    let cookie = "53616c7465645f5f8314147db14eb79bcc737912132b1bb1f13c6e013f0f8deff934838c8f947722ec154ce0dc2d3794d3daf8c94b3a1f1d0cc9cc1f1dc16184";
    let http_api= format!("https://adventofcode.com/{year}/day/{day}/input");
    let res = utils::get_str_input(&http_api, &cookie).await?;
    // let res = utils::get_dummy_input("dummy_input.txt")?;
    day3::part_two(res.clone());


    Ok(())
}
