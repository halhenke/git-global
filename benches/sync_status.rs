#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use git_global::subcommands::sync_status::get_results;

fn new_tync(c: &mut Criterion) {
    // let result_vecs = repos_from_vecs(vec!["a", "aa", "aap"]);

    // let d = c.sample_size(10);
    // Criterion::
    c.bench_function("sync-status", |b| {
        // let n: Result<GitGlobalResult, GitGlobalError> =
        //     test::black_box(Ok(GitGlobalResult::new(&result_vecs)));
        b.iter(|| get_results(false, false, None))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(2);
    targets = new_tync
}
// criterion_group!(benches, new_tync);
criterion_main!(benches);
