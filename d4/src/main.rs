use std::fs;
use std::ops::Index;

use utils::{parse_grid , file_to_vec_str};
//return xmas occurrence inside greed
// fn get_horizontal() -> Result<[&str], ()> {
//
//
//
//
//
// }

fn part_one(grid: Vec<Vec<char>>) -> i32 {

    for line in grid.iter() {

        for (i,_) in line.iter().enumerate(){
            let end = (i + 5).min(line.len());
            println!("{:?}", &line[i..end]);
        }


    }

    0
}
fn main() {
    let grid = parse_grid("p1-example.txt").unwrap();
    let res = part_one(grid);

    println!("{:?}", res);

    



    println!("Hello, world!");
}
