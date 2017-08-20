#[macro_use] extern crate log;
extern crate loglog;

fn main() {
    loglog::init().unwrap();

    trace!("TRACE MESSAGE");
    debug!("LOGS OF BUGS HERE");

    info!("Starting progress");
    debug!("progress: 0%");
    debug!("progress: 10%");
    debug!("progress: 25%");
    debug!("progress: 43%");
    debug!("progress: 68%");
    debug!("progress: 97%");
    debug!("progress: 99%");
    debug!("progress: 100%");
    info!("Progress done!");

    info!("This is an info message");
    warn!("uh ohhhhh");
    warn!("you should really read these");

    error!("aaaah noooo there's an error");
    error!("oh no another error!");

    info!("Multiple\nlines\ninfo");
}
