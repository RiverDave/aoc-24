use std::error::Error;
use utils::file_to_vec_str;

fn parse_num_list_as_vec(list_lines: Vec<String>) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {

    //get vector of reports as strings
    //TODO: Utilize slices to avoid increased space complexity?
    let report_as_str: Vec<Vec<String>> = list_lines
        .into_iter()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    //String to num conversion inside each listed of reports :?>:?
    //["1", "2", "3"] ->  [1, 2, 3]
    let vec_list: Vec<Vec<i32>> = report_as_str
        .into_iter()
        .map(|vec| {
            vec.into_iter()
                .map(|s| s.parse::<i32>().expect("couldn't parse shit"))
                .collect()
        })
        .collect();

    Ok(vec_list)
}

fn is_safe(report: &Vec<i32>) -> bool {
    let slice_size = 2; //we'll compare the distance adjacent elements, hence we use a sliding
                        //window
    let window = report.windows(slice_size);

    let mut last_distance: i32 = 0;
    let mut is_safe = true;
    for w in window {
        let distance = w[0] - w[1];

        //Make sure elements don't break current pattern
        if distance.is_positive() && last_distance.is_negative()
            || distance.is_negative() && last_distance.is_positive()
        {
            is_safe = false;
            break;
        }

        if distance.abs() > 3 || distance == 0 {
            is_safe = false;
            break;
        }

        last_distance = distance;
    }

    is_safe
    // println!("{:?} => {}", vec, is_safe);
}

fn part_one(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;
    reports
        .into_iter()
        .for_each(|report| match is_safe(&report) {
            true => safe_count += 1,
            _ => (),
        });
    safe_count
}

fn part_two(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_report_count = 0;
    reports.into_iter().for_each(|report| {
        let mut is_safe_flag = false;
        for (i, _) in report.as_slice().iter().enumerate() {

            //Iterate through each element
            //Check if it is safe by removing the ith element
            //If it manages to be safe by removing it any element
            //else it is deemed as unsafe

            //try with slice without first level
            if i == 0 {
                is_safe(&report[i + 1..].to_vec()).then(|| {
                    is_safe_flag = true;
                });

            //try with slice without last level
            } else if i == (report.len() - 1) {
                is_safe(&report[..i].to_vec()).then(|| {
                    is_safe_flag = true;
                });
            } else {
                //try with sliced report without level in the middle
                //skip ith, so do a partition into two
                let p1 = &report[..i];
                let p2 = &report[i + 1..];

                //concat slices into vector
                is_safe(&[p1, p2].concat().to_vec()).then(|| {
                    is_safe_flag = true;
                    true
                });
            }
        }

        is_safe_flag.then(|| {
            safe_report_count += 1;
        });
    });

    safe_report_count
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = file_to_vec_str("p1-input.txt").expect("Error while reading input");
    let reports = parse_num_list_as_vec(input)?;

    let res = part_one(&reports);
    println!("P1: {res}");

    let input = file_to_vec_str("p2-input.txt").expect("Error while reading input");
    let reports = parse_num_list_as_vec(input)?;
    let res = part_two(reports);
    println!("P2: {res}");

    Ok(())
}
