// Unix philosophy:
// Expect the output of every program to become the input to another, as yet unknown, program.

use atty::Stream;

// Is our output for a human in front of a colorful terminal, or for another program?
fn main() {
    if atty::is(Stream::Stdout) {
        println!("I'm a terminal");
    } else {
        println!("I'm not");
    }
}

// Depending on who will read our output, we can then add extra information. Humans tend to like colors, for example, if you run ls in a random Rust project, you might see something like this:

// $ ls
// CODE_OF_CONDUCT.md   LICENSE-APACHE       examples
// CONTRIBUTING.md      LICENSE-MIT          proptest-regressions
// Cargo.lock           README.md            src
// Cargo.toml           convey_derive        target

// Because this style is made for humans, in most configurations it’ll even print some of the names (like src) in color to show that they are directories. If you instead pipe this to a file, or a program like cat, ls will adapt its output. Instead of using columns that fit my terminal window it will print every entry on its own line. It will also not emit any colors.

// $ ls | cat
// CODE_OF_CONDUCT.md
// CONTRIBUTING.md
// Cargo.lock
// Cargo.toml
// LICENSE-APACHE
// LICENSE-MIT
// README.md
// convey_derive
// examples
// proptest-regressions
// src
// target

// TSV (Tab-separated values)
// CSV
// JSON
