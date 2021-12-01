use advent_of_code_2020::Solver;

pub struct Day4Puzzle1<'a> {
    pub data: &'a str,
}

impl<'a> Solver<usize> for Day4Puzzle1<'a> {
    fn solve(&self) -> usize {
        solve(
            self.data,
            filter_not_none,
            filter_not_none,
            filter_not_none,
            filter_not_none,
            |_| true,
            filter_not_none,
            filter_not_none,
            filter_not_none,
        )
    }
}

pub struct Day4Puzzle2<'a> {
    pub data: &'a str,
}

impl<'a> Solver<usize> for Day4Puzzle2<'a> {
    fn solve(&self) -> usize {
        solve(
            self.data,
            |o_ecl| {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&o_ecl.unwrap_or(""))
            },
            |o_byr| {
                o_byr
                    .iter()
                    .filter(|s| s.len() == 4)
                    .flat_map(|s| s.parse::<usize>())
                    .filter(|&p| p >= 1920 && p <= 2002)
                    .count()
                    == 1
            },
            |o_iyr| {
                o_iyr
                    .iter()
                    .filter(|s| s.len() == 4)
                    .flat_map(|s| s.parse::<usize>())
                    .filter(|&p| p >= 2010 && p <= 2020)
                    .count()
                    == 1
            },
            |o_pid| {
                o_pid
                    .iter()
                    .filter(|s| s.len() == 9)
                    .flat_map(|s| s.parse::<usize>())
                    .count()
                    == 1
            },
            |_| true,
            |o_hgt| {
                o_hgt
                    .iter()
                    .filter(|&s| {
                        let len = s.len();
                        match (&s[len - 2..], s[..len - 2].parse::<usize>()) {
                            ("cm", Ok(num)) => num >= 150 && num <= 193,
                            ("in", Ok(num)) => num >= 59 && num <= 76,
                            _ => false,
                        }
                    })
                    .count()
                    == 1
            },
            |o_eyr| {
                o_eyr
                    .iter()
                    .filter(|s| s.len() == 4)
                    .flat_map(|s| s.parse::<usize>())
                    .filter(|&p| p >= 2020 && p <= 2030)
                    .count()
                    == 1
            },
            |o_hcl| {
                o_hcl
                    .iter()
                    .filter(|s| s.len() == 7)
                    .filter(|s| s.starts_with("#"))
                    .filter_map(|s| i64::from_str_radix(&s[1..], 16).ok())
                    .count()
                    == 1
            },
        )
    }
}

fn filter_not_none(p: &Option<&str>) -> bool {
    p != &None
}

fn solve<F1, F2, F3, F4, F5, F6, F7, F8>(
    data: &str,
    f_ecl: F1,
    f_byr: F2,
    f_iyr: F3,
    f_pid: F4,
    f_cid: F5,
    f_hgt: F6,
    f_eyr: F7,
    f_hcl: F8,
) -> usize
where
    F1: Fn(&Option<&str>) -> bool,
    F2: Fn(&Option<&str>) -> bool,
    F3: Fn(&Option<&str>) -> bool,
    F4: Fn(&Option<&str>) -> bool,
    F5: Fn(&Option<&str>) -> bool,
    F6: Fn(&Option<&str>) -> bool,
    F7: Fn(&Option<&str>) -> bool,
    F8: Fn(&Option<&str>) -> bool,
{
    data.split("\n\n")
        .map(|l| {
            let pi = l
                .trim()
                .split(char::is_whitespace)
                .fold(PassportInfo::new(), |mut acc, i| {
                    let info: Vec<&str> = i.split(":").collect();

                    match info[0] {
                        "ecl" => {
                            acc.ecl = Some(info[1]);
                        }
                        "byr" => {
                            acc.byr = Some(info[1]);
                        }
                        "iyr" => {
                            acc.iyr = Some(info[1]);
                        }
                        "pid" => {
                            acc.pid = Some(info[1]);
                        }
                        "cid" => {
                            acc.cid = Some(info[1]);
                        }
                        "hgt" => {
                            acc.hgt = Some(info[1]);
                        }
                        "eyr" => {
                            acc.eyr = Some(info[1]);
                        }
                        "hcl" => {
                            acc.hcl = Some(info[1]);
                        }
                        _ => {
                            panic!("Key unknown : {}", info[0]);
                        }
                    };
                    acc
                });
            pi
        })
        .filter(|p| f_ecl(&p.ecl))
        .filter(|p| f_byr(&p.byr))
        .filter(|p| f_iyr(&p.iyr))
        .filter(|p| f_pid(&p.pid))
        .filter(|p| f_cid(&p.cid))
        .filter(|p| f_hgt(&p.hgt))
        .filter(|p| f_eyr(&p.eyr))
        .filter(|p| f_hcl(&p.hcl))
        .count()
}

#[derive(Debug)]
struct PassportInfo<'a> {
    ecl: Option<&'a str>,
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
    hgt: Option<&'a str>,
    eyr: Option<&'a str>,
    hcl: Option<&'a str>,
}

impl<'a> PassportInfo<'a> {
    fn new() -> PassportInfo<'a> {
        PassportInfo {
            ecl: None,
            byr: None,
            iyr: None,
            pid: None,
            cid: None,
            hgt: None,
            eyr: None,
            hcl: None,
        }
    }
}
