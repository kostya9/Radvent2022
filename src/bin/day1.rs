use std::fs;
use std::env;

type Calories = i32;

struct Elf {
    calories: Vec<Calories>
}

fn toElf(elfStuff: &str) -> Elf {
    let calories: Vec<i32> = elfStuff.split("\n").map(|c| c.parse::<i32>().unwrap()).collect();
    return Elf {
        calories: calories
    }
}

fn parseElves(input: String) -> Vec<Elf> {
    return input.split("\n\n").filter(|s| s.len() > 0).map(|s| toElf(s)).collect();
}

fn readElvesInput() -> String {
    let args: Vec<String> = env::args().collect();
    let mut filePath = env::current_dir().unwrap();

    let defaultFile = "day1_input.txt".to_owned();
    filePath.push(args.get(1).unwrap_or(&defaultFile));

    return fs::read_to_string(filePath).expect("Should have been able to read the file. Is the file path correct?");
}

fn first() {
    let contents = readElvesInput();
    let elves = parseElves(contents);

    println!("Elves count: {}", elves.len());

    let mut maxCalories: i32 = 0;
    for elf in elves {
        let currentElfSum = elf.calories.iter().sum();

        if currentElfSum > maxCalories {
            maxCalories = currentElfSum
        }
    }

    println!("Max calories for elf = {}", maxCalories)
}

fn second() {
    let contents = readElvesInput();
    let elves = parseElves(contents);

    println!("Elves count: {}", elves.len());


    let mut currentMinIdx = 0;
    let mut maxCalories: [i32; 3] = [0, 0, 0];
    for elf in elves {
        let currentElfSum = elf.calories.iter().sum();

        let minElf = maxCalories[currentMinIdx];
        if currentElfSum > minElf {
            maxCalories[currentMinIdx] = currentElfSum;

            for i in 0..3 {
                if maxCalories[i] < maxCalories[currentMinIdx] {
                    currentMinIdx = i;
                }
            }
        }
    }

    println!("Max calories for elf = {:?}. Sum = {}", maxCalories, maxCalories.iter().sum::<i32>())
}

fn main() {
    println!("----First task of day1---");
    first();

    println!();

    println!("----Second task of day1---");
    second();
}
