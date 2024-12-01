use std::error::Error;
use std::fs::{self, File};
use std::io;

///Reads a typical aoc file and returns a vector of strings which represent the lines inside
pub fn str_to_vec(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let list: Vec<String> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    Ok(list)
}

//parse file with list of nums to vec
pub fn str_to_vec_nums(path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let str_list = str_to_vec(path)?;
    //TODO: Improve this in a more 'idiomatic' way?
    let mut int_list : Vec<i32> = vec![];
    str_list.iter().for_each(|e| int_list.push(e.parse::<i32>().expect("Couldn't parse str to int")));

    Ok(int_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let result = str_to_vec("./test.txt")?;
        assert_eq!(result.len(), 7);

        let result = str_to_vec_nums("./nums.txt")?;
        assert_eq!(result.len(), 10);
        Ok(())
    }
}
