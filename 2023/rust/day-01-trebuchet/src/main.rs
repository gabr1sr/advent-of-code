use std::fs;

fn part_one(contents: &str) -> u32 {
    let mut total_value = 0;
    
    for line in contents.lines() {
        let numerics: Vec<&str> = line.matches(char::is_numeric).collect();
        let first_value = numerics[0].parse::<u32>().unwrap() * 10;
        let last_value = numerics[numerics.len() - 1].parse::<u32>().unwrap();
        total_value += first_value + last_value;
    }

    total_value
}

fn part_two(contents: &str) -> u32 {
    let mut total_value = 0;

    let words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for line in contents.lines() {
        let mut numerics_indices: Vec<_> = Vec::new();
        
        for word in words {
            let mut values: Vec<_> = line.match_indices(word).collect();
            numerics_indices.append(&mut values);
        }

        numerics_indices.sort_by(|a, b| a.0.cmp(&b.0));

        let (_, first_value_str) = numerics_indices[0];
        let (_, last_value_str) = numerics_indices[numerics_indices.len() - 1];

        let first_value = cast_word(first_value_str, words) * 10;
        let last_value = cast_word(last_value_str, words);
        
        total_value += first_value + last_value;
    }
        
    total_value
}

fn cast_word(word: &str, words: [&str; 20]) -> u32 {
    (words.iter().position(|&val| val == word).unwrap() % 10).try_into().unwrap()
}

fn main() {
    let path = "./calibration.txt";

    let contents = fs::read_to_string(path)
        .expect("No calibration.txt file exists in project root");

    let part_one_result = part_one(&contents);

    println!("[PART 1] Sum of all the calibration values: {}", part_one_result);

    let part_two_result = part_two(&contents);

    println!("[PART 2] Sum of all the calibration values: {}", part_two_result);
}
