fn main() {
    println!("Hello World");

    let x = 42;
    println!("My lucky number is {}.", x);

    let xs = vec![1, 2 ,3];
    println!("The list is: {:?}", xs);

    println!("This is information");
    eprintln!("This is an error! :(");

    // Use ansi_term crate for raw escape codes

    // Using Buffers to Write to the Terminal without performance issues

    // use std::io::{self, Write};

    // let stdout = io::stdout(); // get the global stdout entity
    // let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    // writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

    // Use a lock on stdout (or stderr) to prevent the system from
    // locking and unlocking over and over again

    // use std::io::{self, Write};

    // let stdout = io::stdout(); // get the global stdout entity
    // let mut handle = stdout.lock(); // acquire a lock on it
    // writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

    // You can also combine these two approaches

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
}
