// Advent of Code - Day 3
use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader},
};

struct DiagnosticReport {
    digits: BTreeMap<i32, Vec<i32>>,
}

impl DiagnosticReport {
    pub fn new() -> DiagnosticReport {
        DiagnosticReport {
            digits: BTreeMap::new(),
        }
    }

    pub fn add_digit(&mut self, index: i32, digit: i32) {
        self.digits.entry(index).or_insert(vec![]);
        self.digits.get_mut(&index).unwrap().push(digit);
    }

    pub fn calc_energy_consumption(&self) {
        let mut rate_digits = String::new();

        for val in self.digits.values() {
            let mut cnt_0 = 0;
            let mut cnt_1 = 1;
            val.iter()
                .map(|x| {
                    if *x == 0 {
                        cnt_0 += 1;
                    } else {
                        cnt_1 += 1;
                    }
                })
                .count();
            let digit = if cnt_0 > cnt_1 { 0 } else { 1 };
            rate_digits.push_str(digit.to_string().as_str());
        }

        println!("Rate Digits: {}", rate_digits);
        let gama_rate = i32::from_str_radix(&rate_digits, 2).unwrap();
        let epsilon_rate = !(i32::MAX << rate_digits.len()) & (!gama_rate);
        println!("Gama Rate: {} - {:b}", gama_rate, gama_rate);
        println!("Epsilon Rate: {} - {:b}", epsilon_rate, epsilon_rate);
        println!("Power Consumption: {}", gama_rate * epsilon_rate);
    }
}

fn main() {
    // Read input
    let file = std::fs::File::open("./input.ron").unwrap();
    let reader = BufReader::new(file);

    let mut report = DiagnosticReport::new();
    for l in reader.lines() {
        let line = l.as_ref().unwrap();
        let chars = line.chars();
        for (index, c) in chars.enumerate() {
            let i = index as i32;
            match c {
                '0' => report.add_digit(i, 0),
                '1' => report.add_digit(i, 1),
                _ => unreachable!(),
            }
        }
    }

    report.calc_energy_consumption();
}
