// Advent of Code - Day 3
use std::io::{BufRead, BufReader};

struct DiagnosticReport {
    input_lines: Vec<String>,
}

impl DiagnosticReport {
    pub fn new() -> DiagnosticReport {
        DiagnosticReport {
            input_lines: vec![],
        }
    }

    pub fn add_line(&mut self, line: String) {
        self.input_lines.push(line);
    }

    pub fn calc_oxygen_generator_rate(&self) -> String {
        let mut winner_lines = self.input_lines.clone();

        for i in 0..winner_lines.len() {
            let line_digits = extract_digits(i, winner_lines.clone());

            // println!("line digits: {:?}", line_digits);
            let sign_bit = find_oxygen_bit(&count_bits(&line_digits));
            winner_lines = calc_winner_lines(i, sign_bit, winner_lines.clone());

            println!(
                "oxy index: {} sign_bit: {}, winner_lines: {:?}",
                i, sign_bit, winner_lines
            );
            if winner_lines.len() == 1 {
                break;
            }
        }

        let oxygen_gen_rate = winner_lines.get(0).unwrap();
        println!("Oxygen Generator Rating: {}", oxygen_gen_rate);
        oxygen_gen_rate.into()
    }

    pub fn calc_co2_scrubber_rate(&self) -> String {
        let mut winner_lines = self.input_lines.clone();

        for i in 0..winner_lines.len() {
            let line_digits = extract_digits(i, winner_lines.clone());

            // println!("line digits: {:?}", line_digits);
            let sign_bit = find_co2_bit(&count_bits(&line_digits));
            winner_lines = calc_winner_lines(i, sign_bit, winner_lines.clone());

            println!(
                "co2 index: {} sign_bit: {}, winner_lines: {:?}",
                i, sign_bit, winner_lines
            );
            if winner_lines.len() == 1 {
                break;
            }
        }

        let co2_scubber_rate = winner_lines.get(0).unwrap();
        println!("CO2 Scrubber Rating: {}", co2_scubber_rate);
        co2_scubber_rate.into()
    }

    pub fn calc_life_support_rating(&self) {
        let oxy_rate = self.calc_oxygen_generator_rate();
        let co2_rate = self.calc_co2_scrubber_rate();
        println!(
            "Life support rating: {}",
            i32::from_str_radix(&oxy_rate, 2).unwrap() * i32::from_str_radix(&co2_rate, 2).unwrap()
        )
    }
}

fn main() {
    // Read input
    let file = std::fs::File::open("./input.ron").unwrap();
    let reader = BufReader::new(file);

    let mut report = DiagnosticReport::new();
    for l in reader.lines() {
        let line = l.as_ref().unwrap();
        report.add_line(line.into());
    }

    report.calc_life_support_rating();
}

fn convert_line_to_vec(line: String) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let chars = line.chars();
    for c in chars {
        match c {
            '0' => res.push(0),
            '1' => res.push(1),
            _ => unreachable!(),
        }
    }
    res
}

fn calc_winner_lines(index: usize, sign_bit: i32, input_lines: Vec<String>) -> Vec<String> {
    let winner_lines: Vec<String> = input_lines
        .iter()
        .filter(|l| {
            l.get(index..)
                .unwrap()
                .starts_with(char::from_digit(sign_bit as u32, 2).unwrap())
        })
        .cloned()
        .collect();
    winner_lines
}

fn extract_digits(index: usize, input_lines: Vec<String>) -> Vec<i32> {
    let mut asd: Vec<i32> = vec![];
    input_lines
        .iter()
        .map(|line| convert_line_to_vec(line.clone()))
        .map(|digits: Vec<i32>| {
            let digit = digits.get(index).unwrap();
            asd.push(*digit)
        })
        .count();
    asd
}

fn count_bits(line: &[i32]) -> Vec<i32> {
    let mut cnt_0 = 0;
    let mut cnt_1 = 1;
    line.iter()
        .map(|x| {
            if *x == 0 {
                cnt_0 += 1;
            } else {
                cnt_1 += 1;
            }
        })
        .count();
    vec![cnt_0, cnt_1]
}

fn find_oxygen_bit(bits: &[i32]) -> i32 {
    if bits.get(0).unwrap() < bits.get(1).unwrap() {
        1
    } else {
        0
    }
}

fn find_co2_bit(bits: &[i32]) -> i32 {
    if bits.get(0).unwrap() < bits.get(1).unwrap() {
        0
    } else {
        1
    }
}
