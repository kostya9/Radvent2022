use std::fs;
use std::env;

type Calories = i32;

struct Elf {
    calories: Vec<Calories>
}

fn to_elf(elf_stuff: &str) -> Elf {
    let calories: Vec<i32> = elf_stuff.split("\n").map(|c| c.parse::<i32>().unwrap()).collect();
    return Elf {
        calories
    }
}

fn parse_elves(input: String) -> Vec<Elf> {
    return input.split("\n\n").filter(|s| s.len() > 0).map(|s| to_elf(s)).collect();
}

fn read_elves_input() -> String {
    let args: Vec<String> = env::args().collect();
    let mut file_path = env::current_dir().unwrap();

    let default_file = "day1_input.txt".to_string();
    file_path.push(args.get(1).unwrap_or(&default_file));

    return fs::read_to_string(file_path).expect("Should have been able to read the file. Is the file path correct?");
}

fn first() {
    let contents = read_elves_input();
    let elves = parse_elves(contents);

    println!("Elves count: {}", elves.len());

    let mut max_calories: i32 = 0;
    for elf in elves {
        let current_elf_sum = elf.calories.iter().sum();

        if current_elf_sum > max_calories {
            max_calories = current_elf_sum
        }
    }

    println!("Max calories for elf = {max_calories}")
}

fn second() {
    let contents = read_elves_input();
    let elves = parse_elves(contents);

    println!("Elves count: {}", elves.len());


    let mut current_min_idx = 0;
    let mut max_calories: [i32; 3] = [0, 0, 0];
    for elf in elves {
        let current_elf_sum = elf.calories.iter().sum();

        let min_elf = max_calories[current_min_idx];
        if current_elf_sum > min_elf {
            max_calories[current_min_idx] = current_elf_sum;

            for i in 0..3 {
                if max_calories[i] < max_calories[current_min_idx] {
                    current_min_idx = i;
                }
            }
        }
    }

    let sum_calories = max_calories.iter().sum::<i32>();
    println!("Max calories for elf = {max_calories:?}. Sum = {sum_calories}");
}

fn main() {
    println!("----First task of day1---");
    first();

    println!();

    println!("----Second task of day1---");
    second();
}
