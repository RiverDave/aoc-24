use regex::Regex;
use std::fs;

//Experimenting with regex:
fn part_one(input: &str) -> i32 {
    //sequence to find:
    //LITERAL: num
    //SYMBOL: (
    //NUM_STR: d{0-10}
    //comma
    //NUM_STR: d{0-10}
    //SYMBOL: )

    let re = Regex::new(r"(mul\((\d+),(\d+)\))").unwrap(); //match anythin pr
                                                           //whole regexp is in parenthesis so that matches are grouped
                                                           //nesten within nums are also grouped, so that they can be retrieved with matched idx

    re.captures_iter(input).fold(0, |acc, c| {
        acc + (c.get(2).unwrap().as_str().parse::<i32>().unwrap()
            * c.get(3).unwrap().as_str().parse::<i32>().unwrap())
    })
}

fn part_two(input: &str) -> i32 {
    //The structure remains similar with p2 what changes is :
    //1.now there are optional capture values(either parse mul or parse do's and dont's)
    //
    //(Need to index them based on the absolute position in their groups)
    let re = Regex::new(r"(mul)\((\d+),(\d+)\)|((do|don't)\(\))").unwrap();

    let mut operate: bool = true;
    let mut sum = 0;
    //I could certainly use a more elegant solution, but this will sufice
    re.captures_iter(input).into_iter().for_each(|e| {
        match e.get(4) {
            //This should be do or don't
            Some(exp) => {
                match exp.as_str() {
                    "do()" => operate = true,
                    "don't()" => operate = false,
                    _ => (),
                };
            }
            None => (),
        };

        println!("{:?}", e);
        match e.get(1) {
            Some(exp) => {
                match operate {
                    true => {
                        // assert_eq!(exp.as_str(), "do");
                        sum += e.get(2).unwrap().as_str().parse::<i32>().unwrap()
                            * e.get(3).unwrap().as_str().parse::<i32>().unwrap();
                        return;
                    }
                    false => (),
                };
            }
            None => (),
        };
    });


    sum
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("p1-input.txt").expect("Failed to read input");
    let res = part_one(&input);
    println!("P1: {res}");

    let input = fs::read_to_string("p2-input.txt").expect("Failed to read input");
    let res = part_two(&input);
    println!("P2: {res}");

    Ok(())
}
