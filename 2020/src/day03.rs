use advent_of_code_2020::Solver;

pub struct Day3Puzzle1<'a> {
    pub data: &'a str,
}

impl<'a> Solver<usize> for Day3Puzzle1<'a> {
    fn solve(&self) -> usize {
        let data: Vec<&str> = self.data.lines().collect();
        count(3, 1, &data)
    }
}

pub struct Day3Puzzle2<'a> {
    pub data: &'a str,
}

impl<'a> Solver<usize> for Day3Puzzle2<'a> {
    fn solve(&self) -> usize {
        let data: Vec<&str> = self.data.lines().collect();

        let slope_ratios: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut result = 1;
        for (right, down) in slope_ratios.iter() {
            result = result * count(*right, *down, &data);
        }
        result
    }
}

fn count(right: usize, down: usize, data: &Vec<&str>) -> usize {
    data.iter()
        .enumerate()
        .filter_map(|(i, line)| if i % down == 0 { Some(line) } else { None })
        .enumerate()
        .filter(|(i, &line)| {
            let index = (i * right) % line.len() as usize;
            &line[index..index + 1] == "#"
        })
        .count()
}
