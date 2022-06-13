// Advent of Code - Day 1
fn main() {
    // Read input
    let input = std::fs::read_to_string("./input.ron").unwrap();
    let inp: Vec<i32> = ron::from_str(input.as_str()).unwrap();

    // Count measurements
    let cnt: i32 =
        inp.iter()
            .enumerate()
            .skip(1)
            .map(|(i, val)| {
                if inp[i - 1] < *val { 1 } else { 0 }
            }).sum();

    println!("Total measurements larger than previous one: {}", cnt);
}
