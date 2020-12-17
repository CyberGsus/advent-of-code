use aoc_rust::day16;
fn main() {
    let contents: &'static [u8] = include_bytes!("../inputs/day16.txt");
    println!("{}", day16::part2(&contents));
}
