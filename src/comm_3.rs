use structopt::StructOpt;
use serde_json::json;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long = "json")]
    json: bool,
}

fn main() {
    let args = Cli::from_args();

    if args.json {
        println!("{}", json!({
            "type": "message",
            "content": "Hello world",
        }));
    } else {
        println!("Hello world");
    }
}

// TODO: working with stdin

// convey is an in-development library that tries to make it easier to output messages in formats suitable for both humans and machines. You define your own message types, and implement a Render trait (either manually, with the help of macros, or using a derive attribute) to say how they should be formatted. Currently, it supports printing human output (incl. auto-detecting whether it should be colored), writing JSON documents (either to the stdout or to a file), or both at the same time.

// Even if you do not use this library, it might be a good idea to write a similar abstraction that fits your use case.
