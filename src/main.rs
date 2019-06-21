use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

/// Seach for a patter in a file and display the lines that contain it.
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

// fn main() {
// fn main() -> Result<(), Box<dyn std::error::Error>> {
// fn main() -> Result<(), CustomError> {
fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    // let content = std::fs::read_to_string(&args.path)
    //     .expect("could not read file");
    //
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => { content }
    //     Err(error) => { return Err(error.into()); }
    // };
    //
    // println!("File content: {}", content);
    // Ok(())

    // let content = std::fs::read_to_string(&args.path)?;
    // println!("File content: {}", content);
    // Ok(())

    // let content = std::fs::read_to_string(&args.path)
    //     .map_err(|err| CustomError(format!("Error reading `{:?}`: {:?}", &args.path, err)))?;
    // println!("File content: {}", content);
    // Ok(())

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{:?}`", &args.path))?;
    println!("File content: {}", content);
    Ok(())
}
