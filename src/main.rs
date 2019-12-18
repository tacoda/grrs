use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

/// Seach for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;
    println!("File content: {}", content);

    tacoda_grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

// Using Buffers to Write to the Terminal without performance issues

// use std::io::{self, Write};
//
// let stdout = io::stdout(); // get the global stdout entity
// let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
// writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

// use std::io::{self, Write};
//
// let stdout = io::stdout(); // get the global stdout entity
// let mut handle = stdout.lock(); // acquire a lock on it
// writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here


// Progress Bar
// indicatif crate
// fn main() {
//     let pb = indicatif::ProgressBar::new(100);
//     for i in 0..100 {
//         do_hard_work();
//         pb.println(format!("[+] finished #{}", i));
//         pb.inc(1);
//     }
//     pb.finish_with_message("done");
// }

// Logging
// use log::{info, warn};
//
// fn main() {
//     env_logger::init();
//     info!("starting up");
//     warn!("oops, nothing implemented!");
// }
//
// $ env RUST_LOG=output_log=info cargo run --bin output-log

#[test]
fn check_answer_validity() {
    let answer = 42;
    assert_eq!(answer, 42);
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    tacoda_grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
