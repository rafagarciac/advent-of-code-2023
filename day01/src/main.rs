use std::fs::read_to_string;

fn main() {
    let filename: &str = "./src/input.txt";
    // let digits: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut numbers: Vec<i32> = Vec::new();
    for line in lines {
        println!("Line: {}", line);

        let mut chars_by_line: Vec<char> = Vec::new();
        let chars: Vec<char> = line.chars().collect();

        println!("{:?}", chars);

        for char in chars {
            if char.is_numeric() {
                chars_by_line.push(char);
            }
        }

        if chars_by_line.len() > 2 {
            let first: char = *chars_by_line.first().unwrap();
            let last: char = *chars_by_line.last().unwrap();
            chars_by_line = Vec::new();
            chars_by_line.push(first);
            chars_by_line.push(last);
        }

        if chars_by_line.len() == 1 {
            let first: char = *chars_by_line.first().unwrap();
            chars_by_line = Vec::new();
            chars_by_line.push(first);
            chars_by_line.push(first);
        }

        let string: String = chars_by_line.iter().collect::<String>();
        let number: i32 = string.parse::<i32>().unwrap();
        println!("vec: {:?}, string: {}, number: {}", chars_by_line, string, number);

        numbers.push(number);
    }

    let sum: i32 = numbers.iter().sum();

    println!("The calibration values: {:?}", numbers);
    println!("The calibration sum: {}", sum)
}