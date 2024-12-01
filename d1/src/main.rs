//Result
//P1: O(nlogn)
//P2: O(n)

use ::utils::file_to_vec_str;
use std::collections::HashMap;
use std::error::Error;

// convert string with the following format: 5   10
// into a tuple of vectors(Am I overthinking in here?)
fn parse_lists(list: Vec<String>) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut v1: Vec<i32> = vec![];
    let mut v2: Vec<i32> = vec![];

    for line in list.iter() {
        //convert each line into a vec [1,2]

        //Im using tuple consirering each line contains two nums
        //separated by a whitepace(3)
        let str_pair = line.split_once("   ").unwrap_or(("0", "0"));
        v1.push(str_pair.0.parse::<i32>()?);
        v2.push(str_pair.1.parse::<i32>()?);
    }

    Ok((v1, v2))
}

fn part_one(input_file: &str) -> Result<i32, Box<dyn Error>> {
    let content = file_to_vec_str(input_file)?;
    let (mut v1, mut v2) = parse_lists(content)?;

    assert_eq!(v1.len(), v2.len());
    //log n
    v1.sort();
    v2.sort();

    let mut sum = 0;
    //zip combines two iters & forms a tuple. perfect, just what I needed.
    //Calculate distance based on both iterator values
    for (a, b) in v1.iter().zip(v2.iter()) {
        sum += std::cmp::max(a, b) - std::cmp::min(a, b);
    }

    Ok(sum)
}

fn part_two(input_file: &str) -> Result<i32, Box<dyn Error>> {
    let mut freq_map: HashMap<i32, i32> = HashMap::new();

    let content = file_to_vec_str(input_file)?;
    let (v1, v2) = parse_lists(content)?;

    //Populate hashmap by counting right list frequencies
    for num in v2.iter() {
        *freq_map.entry(*num).or_insert(0) += 1;
    }

    let mut sum = 0;
    //calculate final sum
    for num in v1.iter() {
        let curr_freq = freq_map.get(num).unwrap_or(&0);
        sum += num * curr_freq;
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let p1_res = part_one("p1-input.txt")?;
    println!("P1: {}", p1_res);

    let p2_res = part_two("p2-input.txt")?;
    println!("P2: {}", p2_res);

    Ok(())
}
