const DATA: &str = include_str!("../../data/03.input");

fn main() {
    println!("Day 03");

    let data: Vec<&str> = DATA.lines().collect();

    println!("Puzzle 1: {:#?}", count(3, 1, &data));

    let slope_ratios: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;
    for (right, down) in slope_ratios.iter() {
        result = result * count(*right, *down, &data);
    }
    println!("Puzzle 2: {:#?}", result);
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
