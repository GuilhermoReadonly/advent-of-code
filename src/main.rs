use advent_of_code::BenchSuite;

mod day01;
pub use day01::*;

mod day02;
pub use day02::*;

mod day03;
pub use day03::*;

mod day04;
pub use day04::*;

fn main() {
    let passes_per_puzzle = 30;

    BenchSuite::new(passes_per_puzzle)
        .add_bench(
            "Day 1 Puzzle 1",
            Box::new(Day1Puzzle1 {
                data: include_str!("../data/01.input"),
                sum: 2020,
            }),
        )
        .run_bench()
        .print_results();

    BenchSuite::new(passes_per_puzzle)
        .add_bench(
            "Day 1 Puzzle 2",
            Box::new(Day1Puzzle2 {
                data: include_str!("../data/01.input"),
                sum: 2020,
            }),
        )
        .run_bench()
        .print_results();

    BenchSuite::new(passes_per_puzzle)
        .add_bench(
            "Day 2 Puzzle 1",
            Box::new(Day2Puzzle1 {
                data: include_str!("../data/02.input"),
            }),
        )
        .run_bench()
        .print_results();

    BenchSuite::new(passes_per_puzzle)
        .add_bench(
            "Day 2 Puzzle 2",
            Box::new(Day2Puzzle2 {
                data: include_str!("../data/02.input"),
            }),
        )
        .run_bench()
        .print_results();

    BenchSuite::new(passes_per_puzzle)
        .add_bench(
            "Day 3 Puzzle 1",
            Box::new(Day3Puzzle1 {
                data: include_str!("../data/03.input"),
            }),
        )
        .run_bench()
        .print_results();

    BenchSuite::new(passes_per_puzzle)
        .add_bench(
            "Day 3 Puzzle 2",
            Box::new(Day3Puzzle2 {
                data: include_str!("../data/03.input"),
            }),
        )
        .run_bench()
        .print_results();

    BenchSuite::new(passes_per_puzzle)
        .add_bench(
            "Day 4 Puzzle 1",
            Box::new(Day4Puzzle1 {
                data: include_str!("../data/04.input"),
            }),
        )
        .run_bench()
        .print_results();

    BenchSuite::new(passes_per_puzzle)
        .add_bench(
            "Day 4 Puzzle 2",
            Box::new(Day4Puzzle2 {
                data: include_str!("../data/04.input"),
            }),
        )
        .run_bench()
        .print_results();
}
