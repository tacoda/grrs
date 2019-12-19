use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

fn main() -> Result<(), exitfailure::ExitFailure> {
    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}

// Futures and Streams

// If you are using tokio, you are most likely already writing your application with asynchronous patterns and an event-driven design. Instead of using crossbeam’s channels directly, you can enable signal-hook’s tokio-support feature. This allows you to call .into_async() on signal-hook’s Signals types to get a new type that implements futures::Stream.

// If you receive another Ctrl-C while handling the first one,
// the typical behavior is to quit immediately.
