#![allow(dead_code)]

extern crate regex;

#[macro_use]
extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    fn load_file(path: &str) -> String {
        let mut input = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut input).expect("Unable to read string");

        input
    }

    #[test]
    fn solve_day01() {
        use day01::{star_one, star_two};

        let input = load_file("day1.txt");

        assert_eq!(star_one(&input), 510);
        assert_eq!(star_two(&input), 69074);
    }
    #[test]
    fn solve_day02() {
        use day02::{star_one, star_two};

        let input = load_file("day2.txt");

        assert_eq!(star_one(&input), 5166);
        assert_eq!(star_two(&input), "cypueihajytordkgzxfqplbwn");
    }
    #[test]
    fn solve_day03() {
        use day03::{star_one, star_two};

        let input = load_file("day3.txt");

        assert_eq!(star_one(&input), 110891);
        assert_eq!(star_two(&input), 297);
    }
    #[test]
    fn solve_day04() {
        use day04::{star_one, star_two};

        let input = load_file("day4.txt");

        assert_eq!(star_one(&input), 19874);
        assert_eq!(star_two(&input), 22687);
    }
    #[test]
    fn solve_day05() {
        use day05::{star_one, star_two};

        let input = load_file("day5.txt");

        assert_eq!(star_one(&input), 10250);
        assert_eq!(star_two(&input), 6188);
    }
    #[test]
    fn solve_day06() {
        use day06::{star_one, star_two};

        let input = load_file("day6.txt");

        assert_eq!(star_one(&input), 3687);
        assert_eq!(star_two(&input, 10000), 40134);
    }
    #[test]
    fn solve_day07() {
        use day07::{star_one, star_two};

        let input = load_file("day7.txt");

        assert_eq!(star_one(&input), "EFHLMTKQBWAPGIVXSZJRDUYONC");
        assert_eq!(star_two(&input, 5, 60), 1056);
    }
    #[test]
    fn solve_day08() {
        use day08::{star_one, star_two};

        let input = load_file("day8.txt");

        assert_eq!(star_one(&input), 40977);
        assert_eq!(star_two(&input), 27490);
    }
    #[test]
    fn solve_day09() {
        use day09::solve_efficient;

        assert_eq!(solve_efficient(424, 71144), 405143);
        assert_eq!(solve_efficient(424, 71144 * 100), 3411514667);
    }
    #[test]
    fn solve_day10() {
        use day10::{star_one, star_two};

        let input = load_file("day10.txt");
        let expected = load_file("day10_expected.txt");

        assert_eq!(star_one(&input, 10081), expected.trim());
    }
    #[test]
    fn solve_day11() {
        use day11::{star_one, star_two};

        let input = load_file("day11.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day12() {
        use day12::{star_one, star_two};

        let input = load_file("day12.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day13() {
        use day13::{star_one, star_two};

        let input = load_file("day13.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day14() {
        use day14::{star_one, star_two};

        let input = load_file("day14.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day15() {
        use day15::{star_one, star_two};

        let input = load_file("day15.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day16() {
        use day16::{star_one, star_two};

        let input = load_file("day16.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day17() {
        use day17::{star_one, star_two};

        let input = load_file("day17.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day18() {
        use day18::{star_one, star_two};

        let input = load_file("day18.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day19() {
        use day19::{star_one, star_two};

        let input = load_file("day19.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day20() {
        use day20::{star_one, star_two};

        let input = load_file("day20.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day21() {
        use day21::{star_one, star_two};

        let input = load_file("day21.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day22() {
        use day22::{star_one, star_two};

        let input = load_file("day22.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day23() {
        use day23::{star_one, star_two};

        let input = load_file("day23.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day24() {
        use day24::{star_one, star_two};

        let input = load_file("day24.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
}
