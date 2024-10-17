use std::{thread, time::Duration};

use record::event_loop::{EventLoop, EventLoopConfig};

// The definition of this function depends on the context in which record is used
fn sleep(duration: Duration) {
    thread::sleep(duration);
}

fn main() {
    let el = EventLoop::new(EventLoopConfig { fps: 1 });

    el.start(|_config| println!("Hello! in the event loop"), sleep);
}
