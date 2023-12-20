use std::fs;
use regex::Regex;

const INPUT_PATH: &str = "input.txt";
const REGEX_EXTRACT_NUMBER: &str = r"(?<number>\d+)";
fn main() {

    let lines = read_file();
    let re = Regex::new(REGEX_EXTRACT_NUMBER).unwrap();

    let result: u32 = lines.iter()
    .map(|line| -> String {
        let matches: Vec<String> = re.find_iter(&line).map(|f| f.as_str().to_string()).collect();
        return matches.join("");
    })
    .map(|f| {
        let mut numbers = f.to_owned();
        if numbers.len() == 1 {
            numbers.push_str(numbers.clone().as_str());
        } else if numbers.len() > 2 {
            numbers = format!("{}{}", numbers.chars().next().unwrap(), numbers.chars().last().unwrap());
        }
        return numbers;
    })
    .map(|f| -> u32 {
        let result_int: u32 =  f.parse().unwrap();
        return result_int;
    })
    .sum();
    println!("Result: {}", result);
}

fn read_file() -> Vec<String> {
    let content_files = fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<String> = content_files.lines().map(String::from).collect();
    return lines;
}
