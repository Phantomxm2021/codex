#![allow(clippy::expect_used)]

use std::process::Command;

use divan::Bencher;

fn main() {
    divan::main();
}

/// Exercises the Bazel-backed end-to-end benchmark path with a cheap,
/// deterministic Bgs invocation. Richer scenarios can add separate
/// benchmark binaries without making the shared harness depend on them.
#[divan::bench(sample_count = 20, sample_size = 1)]
fn bgs_help(bencher: Bencher) {
    let bgs = bgs_utils_cargo_bin::cargo_bin("bgs")
        .expect("bgs binary should be available through Bazel runfiles");

    bencher.bench_local(move || {
        let output = Command::new(&bgs)
            .arg("--help")
            .output()
            .expect("bgs --help should run");
        assert!(output.status.success(), "bgs --help should succeed");
    });
}
