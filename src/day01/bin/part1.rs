pub fn part1(input: String) -> usize {
    input.lines().enumerate().for_each(|(_idx, ch)| {
        ch.chars().enumerate().for_each(|(_idx, ch)| {
            if ch.is_numeric() {
                print!("{}\n", ch)
            }
        })
    });
    return 0;
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn test_part1() {
        let a = String::new();
        assert_eq!(part1(a), 0);
    }
}
