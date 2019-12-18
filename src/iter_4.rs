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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content }
        Err(error) => { return Err(error.into()); }
    };
    // let content = std::fs::read_to_string("test.txt")?;

    println!("File content: {}", content);
    Ok(())
}
