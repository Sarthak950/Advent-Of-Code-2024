use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct NumberHash {
    key: i32,
    value: i32,
}

fn read_file_line_by_line_and_parse(
    path: &str,
    left_list: &mut Vec<i32>,
    right_list: &mut Vec<i32>,
) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut i = 0;
    for line in reader.lines() {
        for word in line?.split_whitespace() {
            if i % 2 == 0 {
                left_list.push(word.parse().unwrap());
            } else {
                right_list.push(word.parse().unwrap());
            }
            i += 1;
        }
    }

    Ok(())
}

fn get_count_of_number_in_list(list: &Vec<i32>, number: i32) -> i32 {
    let mut count = 0;
    for i in list {
        if *i == number {
            count += 1;
        }
    }
    count
}

fn main() {
    let path = "./input/01.txt";

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    read_file_line_by_line_and_parse(path, &mut left_list, &mut right_list).unwrap();

    //println!("{:?}", left_list);
    //println!("{:?}", right_list);

    left_list.sort();
    right_list.sort();

    let mut diff: Vec<i32> = Vec::new();
    for i in 0..left_list.len() {
        diff.push((right_list[i] - left_list[i]).abs());
    }

    //println!("{:?}", diff);
    println!("solution for first: {:?}", diff.iter().sum::<i32>());

    let mut hash_vec: Vec<NumberHash> = Vec::new();
    for i in 0..=*right_list.last().unwrap() {
        hash_vec.push({
            NumberHash {
                key: i,
                value: get_count_of_number_in_list(&right_list, i) as i32,
            }
        });
    }

    let mut sum: i32 = 0;
    for &i in left_list.iter() {
        if let Some(rl) = hash_vec.iter().find(|rl| rl.key == i) {
            sum += i * rl.value;
        }
    }

    println!("solution for second: {:?}", sum);
}
