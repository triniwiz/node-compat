use std::sync::mpsc::{channel, RecvError};

pub fn spawn<F>(f: F) -> Result<(), RecvError>
    where F: FnOnce(),
          F: Send + 'static
{
    let (tx, rx) = channel();

    let _ = std::thread::spawn(move || {
        let _ = tx.send(());
        f()
    });

    rx.recv()
}