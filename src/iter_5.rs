use structopt::StructOpt;

/// Seach for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{:?}`: {:?}", &args.path, err)))?;

    println!("File content: {}", content);
    Ok(())
}
