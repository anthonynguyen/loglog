#[macro_use] extern crate log;
extern crate loglog;

fn main() {
    loglog::build()
        .target(false)
        .select("TRACE")
        .time(Some("%H:%M"))
        .init()
        .unwrap();

    trace!("TRACE MESSAGE");
    debug!("LOGS OF BUGS HERE");
    info!("This is an info message");
    warn!("uh ohhhhh");
    error!("aaaah noooo there's an error");

    info!("Multiple\nlines\ninfo");
}
