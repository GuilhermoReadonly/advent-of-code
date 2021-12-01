use std::{
    fmt::Debug,
    time::{Duration, Instant},
    vec::IntoIter,
};

pub trait Solver<A> {
    fn solve(&self) -> A;
}

pub struct Bench<'a, A> {
    bench_name: &'a str,
    solver: Box<dyn Solver<A>>,
}

pub struct BenchSuite<'a, A> {
    suite: Vec<Bench<'a, A>>,
    nb_passes: usize,
}

#[derive(Debug)]
pub struct RunResult<A: Debug> {
    pub result: A,
    pub duration: Duration,
}

pub struct BenchResult<'a, A: Debug> {
    bench_name: &'a str,
    run_results: Vec<RunResult<A>>,
}

pub struct BenchSuiteResult<'a, A: Debug> {
    results: Vec<BenchResult<'a, A>>,
}

impl<'a, A: Debug> Bench<'a, A> {
    fn bench(&self) -> RunResult<A> {
        let now = Instant::now();
        let result = self.solver.solve();
        let duration = now.elapsed();
        RunResult { result, duration }
    }

    pub fn new(bench_name: &'a str, solver: Box<dyn Solver<A>>) -> Bench<'a, A> {
        Bench { bench_name, solver }
    }
}

impl<'a, A: Debug + PartialEq> BenchSuite<'a, A> {
    pub fn new(nb_passes: usize) -> BenchSuite<'a, A> {
        BenchSuite {
            suite: vec![],
            nb_passes,
        }
    }

    pub fn add_bench(
        mut self,
        bench_name: &'a str,
        solver: Box<dyn Solver<A>>,
    ) -> BenchSuite<'a, A> {
        let bench = Bench::new(bench_name, solver);
        self.suite.push(bench);
        self
    }

    pub fn run_bench(self) -> BenchSuiteResult<'a, A> {
        let mut bench_suite_results = BenchSuiteResult::new();
        let passes = self.nb_passes;
        for bench in self {
            let mut bench_results = BenchResult::new(bench.bench_name);
            for _ in 0..passes {
                let result = bench.bench();
                bench_results.push(result);
            }
            bench_suite_results.push(bench_results);
        }
        bench_suite_results
    }
}

impl<'a, A> IntoIterator for BenchSuite<'a, A> {
    type Item = Bench<'a, A>;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.suite.into_iter()
    }
}

impl<'a, A: Debug + PartialEq> BenchResult<'a, A> {
    pub fn new(bench_name: &'a str) -> Self {
        Self {
            bench_name,
            run_results: vec![],
        }
    }

    pub fn push(&mut self, result: RunResult<A>) {
        self.run_results.push(result)
    }

    pub fn print_result(&self) {
        println!(
            "{}. Result of {} passes: {:?}, mean time {}µs, min time {}µs, max time {}µs",
            self.bench_name,
            self.len(),
            self.get_result(),
            self.mean_duration().as_micros(),
            self.min_duration().as_micros(),
            self.max_duration().as_micros()
        );
    }

    pub fn get_result(&self) -> &A {
        let result = self.run_results.get(0).expect("BenchResult empty !");
        for rr in &self.run_results {
            if rr.result != result.result {
                panic!(
                    "Error while verifiyng results: {:?} != {:?}",
                    rr.result, result.result
                )
            }
        }
        &result.result
    }

    pub fn mean_duration(&self) -> Duration {
        let results_iter = self.run_results[..].into_iter();
        results_iter.map(|r| r.duration).sum::<Duration>() / self.run_results.len() as u32
    }

    pub fn min_duration(&self) -> Duration {
        let results_iter = self.run_results[..].into_iter();
        results_iter
            .map(|r| r.duration)
            .min()
            .expect("The BenchResult was empty !")
    }

    pub fn max_duration(&self) -> Duration {
        let results_iter = self.run_results[..].into_iter();
        results_iter
            .map(|r| r.duration)
            .max()
            .expect("The BenchResult was empty !")
    }

    pub fn len(&self) -> usize {
        self.run_results.len()
    }
}

impl<A: Debug> IntoIterator for BenchResult<'_, A> {
    type Item = RunResult<A>;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.run_results.into_iter()
    }
}

impl<'a, A: Debug + PartialEq> BenchSuiteResult<'a, A> {
    pub fn push(&mut self, result: BenchResult<'a, A>) {
        self.results.push(result)
    }

    pub fn new() -> Self {
        Self { results: vec![] }
    }

    pub fn print_results(self) {
        for bench_results in self {
            bench_results.print_result();
        }
    }
}

impl<'a, A: Debug> IntoIterator for BenchSuiteResult<'a, A> {
    type Item = BenchResult<'a, A>;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}
