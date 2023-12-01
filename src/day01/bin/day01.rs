mod part1;

fn main() {
    let input = std::fs::read_to_string("src/day01/input.txt").unwrap();
    println!("---PART ONE---");
    println!("Result :{}", part1::part1(input));
}
