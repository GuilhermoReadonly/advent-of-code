use advent_of_code::Solver;

pub struct Day1Puzzle1<'a> {
    pub data: &'a str,
    pub sum: usize,
}

impl<'a> Solver<(usize, usize, usize, usize)> for Day1Puzzle1<'a> {
    fn solve(&self) -> (usize, usize, usize, usize) {
        let data = self
            .data
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let result = Combinator2::new(&data)
            .filter(|elt| elt.0 + elt.1 == self.sum)
            .nth(0)
            .expect("ergh.. something went wrong !");
        (result.0, result.1, result.0 + result.1, result.0 * result.1)
    }
}

pub struct Day1Puzzle2<'a> {
    pub data: &'a str,
    pub sum: usize,
}
impl<'a> Solver<(usize, usize, usize, usize, usize)> for Day1Puzzle2<'a> {
    fn solve(&self) -> (usize, usize, usize, usize, usize) {
        let data = self
            .data
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let result = Combinator3::new(&data)
            .filter(|elt| elt.0 + elt.1 + elt.2 == self.sum)
            .nth(0)
            .expect("ergh.. something went wrong !");
        (
            result.0,
            result.1,
            result.2,
            result.0 + result.1 + result.2,
            result.0 * result.1 * result.2,
        )
    }
}

#[derive(Debug)]
struct Combinator2<'a> {
    data: &'a Vec<usize>,
    index1: usize,
    index2: usize,
}

#[derive(Debug)]
struct Combinator3<'a> {
    data: &'a Vec<usize>,
    index1: usize,
    index2: usize,
    index3: usize,
}

impl<'a> Combinator2<'a> {
    fn new(data: &Vec<usize>) -> Combinator2 {
        Combinator2 {
            data,
            index1: 0,
            index2: 0,
        }
    }
}

impl<'a> Combinator3<'a> {
    fn new(data: &Vec<usize>) -> Combinator3 {
        Combinator3 {
            data,
            index1: 0,
            index2: 0,
            index3: 0,
        }
    }
}

impl<'a> Iterator for Combinator2<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        if self.index2 < self.data.len() {
            result = Some((self.data[self.index1], self.data[self.index2]))
        }

        self.index1 = self.index1 + 1;

        if self.index1 >= self.data.len() {
            self.index2 = self.index2 + 1;
            self.index1 = self.index2;
        }
        result
    }
}

impl<'a> Iterator for Combinator3<'a> {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index3 < self.data.len() {
            Some((
                self.data[self.index1],
                self.data[self.index2],
                self.data[self.index3],
            ))
        } else {
            None
        };

        self.index1 = self.index1 + 1;

        if self.index1 >= self.data.len() {
            self.index2 = self.index2 + 1;
            self.index1 = self.index2;
        }

        if self.index2 >= self.data.len() {
            self.index3 = self.index3 + 1;
            self.index2 = self.index3;
            self.index1 = self.index2;
        }

        result
    }
}
