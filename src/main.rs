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

    tacoda_grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}


// We’ve just seen how to make this piece of code easily testable. We have
// 1. identified one of the core pieces of our application,
// 2. put it into its own function,
// 3. and made it more flexible.

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    tacoda_grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
