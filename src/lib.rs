// By convention, cargo will look for integration tests in the tests/
// directory. Similarly, it will look for benchmarks in benches/, and
// examples in examples/. These conventions also extend to your main
// source code: libraries have a src/lib.rs file, the main binary is
// src/main.rs, or, if there are multiple binaries, cargo expects them
// to be in src/bin/<name>.rs. Following these conventions will make
// your code base more discoverable by people used to reading Rust code.

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}
