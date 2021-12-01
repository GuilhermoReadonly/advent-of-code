use advent_of_code_2020::Solver;

pub struct Day2Puzzle1<'a> {
    pub data: &'a str,
}

fn map_line(line: &str) -> PasswordPolicy {
    let line_split: Vec<&str> = line.split(" ").collect();

    let password = line_split[2];
    let char = &line_split[1][..1];
    let minmax: Vec<&str> = line_split[0].split("-").collect();
    let min = minmax[0].parse::<usize>().unwrap();
    let max = minmax[1].parse::<usize>().unwrap();
    PasswordPolicy::new(password, char, min, max)
}

impl<'a> Solver<usize> for Day2Puzzle1<'a> {
    fn solve(&self) -> usize {
        self.data
            .lines()
            .map(map_line)
            .filter(|pp| {
                let matches = pp.password.match_indices(pp.char).count();
                let result = matches >= pp.min && matches <= pp.max;
                result
            })
            .count()
    }
}

pub struct Day2Puzzle2<'a> {
    pub data: &'a str,
}

impl<'a> Solver<usize> for Day2Puzzle2<'a> {
    fn solve(&self) -> usize {
        self.data
            .lines()
            .map(map_line)
            .filter(|pp| {
                let char1 = &pp.password[pp.min - 1..pp.min];
                let char2 = &pp.password[pp.max - 1..pp.max];
                let result = (char1 == pp.char && char2 != pp.char)
                    || (char1 != pp.char && char2 == pp.char);
                result
            })
            .count()
    }
}

#[derive(Debug)]
struct PasswordPolicy<'a> {
    pub password: &'a str,
    char: &'a str,
    min: usize,
    max: usize,
}

impl<'a> PasswordPolicy<'a> {
    fn new(password: &'a str, char: &'a str, min: usize, max: usize) -> PasswordPolicy<'a> {
        PasswordPolicy {
            password,
            char,
            min,
            max,
        }
    }
}
