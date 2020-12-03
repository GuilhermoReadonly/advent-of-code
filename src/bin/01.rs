const DATA: &str = include_str!("../../data/01.input");

fn main() {
    println!("Day 01");

    let data = DATA
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for elt in Combinator2::new(&data).filter(|elt| elt.0 + elt.1 == 2020) {
        println!(
            "Puzzle 1: {elt1} {elt2}, sum = {sum}, mul = {mul}",
            elt1 = elt.0,
            elt2 = elt.1,
            sum = elt.0 + elt.1,
            mul = elt.0 * elt.1
        )
    }

    for elt in Combinator3::new(&data).filter(|elt| elt.0 + elt.1 + elt.2 == 2020) {
        println!(
            "Puzzle 2: {elt1} {elt2} {elt3}, sum = {sum}, mul = {mul}",
            elt1 = elt.0,
            elt2 = elt.1,
            elt3 = elt.2,
            sum = elt.0 + elt.1 + elt.2,
            mul = elt.0 * elt.1 * elt.2
        )
    }
}

#[derive(Debug)]
struct Combinator2<'a> {
    data: &'a Vec<i64>,
    index1: usize,
    index2: usize,
}

#[derive(Debug)]
struct Combinator3<'a> {
    data: &'a Vec<i64>,
    index1: usize,
    index2: usize,
    index3: usize,
}

impl<'a> Combinator2<'a> {
    fn new(data: &Vec<i64>) -> Combinator2 {
        Combinator2 {
            data,
            index1: 0,
            index2: 0,
        }
    }
}

impl<'a> Combinator3<'a> {
    fn new(data: &Vec<i64>) -> Combinator3 {
        Combinator3 {
            data,
            index1: 0,
            index2: 0,
            index3: 0,
        }
    }
}

impl<'a> Iterator for Combinator2<'a> {
    type Item = (i64, i64);

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
    type Item = (i64, i64, i64);

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
