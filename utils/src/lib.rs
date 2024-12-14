use std::error::Error;
use std::fs::{self, File};
use std::io;

///Reads a typical aoc file and returns a vector of strings which represent the lines inside
pub fn file_to_vec_str(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let list: Vec<String> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    Ok(list)
}

//parse file with list of nums to vec
pub fn file_to_vec_int(path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let str_list = file_to_vec_str(path)?;
    //TODO: Improve this in a more 'idiomatic' way?
    let mut int_list : Vec<i32> = vec![];
    str_list.iter().for_each(|e| int_list.push(e.parse::<i32>().expect("Couldn't parse str to int")));

    Ok(int_list)
}

pub fn parse_grid(path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let grid: Vec<Vec<char>> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_to_vec() -> Result<(), Box<dyn Error>> {
        let result = file_to_vec_str("./test.txt")?;
        assert_eq!(result.len(), 7);

        let result = file_to_vec_int("./nums.txt")?;
        assert_eq!(result.len(), 10);
        Ok(())
    }

    #[test]
    fn test_parse_grid() -> Result<(), Box<dyn Error>> {
        let result = parse_grid("./grid.txt")?;
        assert_eq!(result.len(), 10);
        Ok(())
    }
}
