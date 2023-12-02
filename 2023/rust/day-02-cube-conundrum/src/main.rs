use std::fs;

fn part_one(contents: &str) -> u32 {
    let mut valid_games: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let values: Vec<&str> = line.split(": ").collect();
        let game_vec: Vec<&str> = values[0].rsplit(' ').collect();
        let game_num = game_vec[0].parse::<u32>().unwrap();

        let mut last_green = 0u32;
        let mut last_red = 0u32;
        let mut last_blue = 0u32;

        let cube_sets_split: Vec<&str> = values[1].split("; ").collect();
        for cube_set_str in cube_sets_split.iter() {
            let cubes_vec: Vec<&str> = cube_set_str.split(", ").collect();
            for cube_str in cubes_vec.iter() {
                let cube_vec: Vec<&str> = cube_str.rsplit(' ').collect();
                let color = cube_vec[0];
                let value = cube_vec[1].parse::<u32>().unwrap();

                match color {
                    "green" => {
                        last_green = if value > last_green {
                            value
                        } else {
                            last_green
                        }
                    }
                    "red" => last_red = if value > last_red { value } else { last_red },
                    "blue" => last_blue = if value > last_blue { value } else { last_blue },
                    _ => (),
                }
            }
        }

        if last_green <= 13 && last_red <= 12 && last_blue <= 14 {
            valid_games.push(game_num);
        }
    }

    valid_games
        .into_iter()
        .try_fold(0u32, |acc, x| acc.checked_add(x))
        .unwrap()
}

fn part_two(contents: &str) -> u32 {
    let mut fewest: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let values: Vec<&str> = line.split(": ").collect();

        let mut last_green = 0u32;
        let mut last_red = 0u32;
        let mut last_blue = 0u32;

        let cube_sets_split: Vec<&str> = values[1].split("; ").collect();
        for cube_set_str in cube_sets_split.iter() {
            let cubes_vec: Vec<&str> = cube_set_str.split(", ").collect();
            for cube_str in cubes_vec.iter() {
                let cube_vec: Vec<&str> = cube_str.rsplit(' ').collect();
                let color = cube_vec[0];
                let value = cube_vec[1].parse::<u32>().unwrap();

                match color {
                    "green" => {
                        last_green = if value > last_green {
                            value
                        } else {
                            last_green
                        }
                    }
                    "red" => last_red = if value > last_red { value } else { last_red },
                    "blue" => last_blue = if value > last_blue { value } else { last_blue },
                    _ => (),
                }
            }
        }

        let power_colors = last_green * last_red * last_blue;
        fewest.push(power_colors);
    }

    fewest
        .into_iter()
        .try_fold(0u32, |acc, x| acc.checked_add(x))
        .unwrap()
}

fn main() {
    let path = "./games.txt";

    let contents = fs::read_to_string(path).expect("No games.txt file exists in project root");

    let part_one_result = part_one(&contents);

    println!("[PART 1] What is the sum of the IDs of those games? {:?}", part_one_result);

    let part_two_result = part_two(&contents);

    println!("[PART 2] What is the sum of the power of these sets? {:?}", part_two_result);
}
