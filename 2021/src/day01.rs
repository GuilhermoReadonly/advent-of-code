fn main() {
    let data = include_str!("../data/01.input");

    let lines = data
        .lines()
        .map(|line| {
            line.parse::<usize>()
                .expect(&format!("Line is not parsable {}", line))
        })
        .collect::<Vec<_>>();

    let mut previous_depth: Option<usize> = None;
    let mut number_of_increases = 0;

    for depth in lines.iter() {
        match previous_depth {
            None => (),
            Some(d) => {
                if depth > &d {
                    number_of_increases += 1;
                }
            }
        }

        previous_depth = Some(*depth);
    }

    println!("Response of day01 part 1 : {}", number_of_increases);

    let window3sum: Vec<usize> = lines.windows(3).map(|w| w.into_iter().sum()).collect();
    let increases = window3sum.windows(2).filter(|w| w[1] > w[0]).count();
    println!("Response of day01 part 2 : {}", increases);
}
