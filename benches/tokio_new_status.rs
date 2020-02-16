#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use git_global::subcommands::new_status::get_results;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

// fn vec_from_vecs<T>(s: Vec<&str>, f: impl FnMut(&str) -> T) -> Vec<T> {
//     s.into_iter().map(f).collect::<Vec<T>>()
// }

// pub fn repos_from_vecs(s: Vec<&str>) -> Vec<Repo> {
//     vec_from_vecs(s, Box::new(|s: &str| Repo::new(s.to_owned())))
// }

fn new_tokio_bench(c: &mut Criterion) {
    // let result_vecs = repos_from_vecs(vec!["a", "aa", "aap"]);

    c.bench_function("thread 1", |b| {
        // let n: Result<GitGlobalResult, GitGlobalError> =
        //     test::black_box(Ok(GitGlobalResult::new(&result_vecs)));
        b.iter(|| get_results(false, false, None, Some(black_box(1))))
    });
    c.bench_function("thread 2", |b| {
        // let n: Result<GitGlobalResult, GitGlobalError> =
        //     test::black_box(Ok(GitGlobalResult::new(&result_vecs)));
        b.iter(|| get_results(false, false, None, Some(black_box(2))))
    });
    c.bench_function("thread 2", |b| {
        // let n: Result<GitGlobalResult, GitGlobalError> =
        //     test::black_box(Ok(GitGlobalResult::new(&result_vecs)));
        b.iter(|| get_results(false, false, None, Some(black_box(2))))
    });
    c.bench_function("thread 8", |b| {
        // let n: Result<GitGlobalResult, GitGlobalError> =
        //     test::black_box(Ok(GitGlobalResult::new(&result_vecs)));
        b.iter(|| get_results(false, false, None, Some(black_box(8))))
    });
    c.bench_function("thread 32", |b| {
        // let n: Result<GitGlobalResult, GitGlobalError> =
        //     test::black_box(Ok(GitGlobalResult::new(&result_vecs)));
        b.iter(|| get_results(false, false, None, Some(black_box(32))))
    });
    c.bench_function("thread 64", |b| {
        // let n: Result<GitGlobalResult, GitGlobalError> =
        //     test::black_box(Ok(GitGlobalResult::new(&result_vecs)));
        b.iter(|| get_results(false, false, None, Some(black_box(64))))
    });
    c.bench_function("thread 128", |b| {
        // let n: Result<GitGlobalResult, GitGlobalError> =
        //     test::black_box(Ok(GitGlobalResult::new(&result_vecs)));
        b.iter(|| get_results(false, false, None, Some(black_box(128))))
    });
    c.bench_function("all threads", |b| {
        // let n: Result<GitGlobalResult, GitGlobalError> =
        //     test::black_box(Ok(GitGlobalResult::new(&result_vecs)));
        b.iter(|| get_results(false, false, None, black_box(None)))
    });
}

criterion_group!(benches, new_tokio_bench);
// criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
