use tracing::{info, instrument};
use tracing_subscriber::fmt::format;

#[instrument]
fn hanoi(n: u32, from: char, to: char, by: char, depth: usize, previous: usize) -> usize {
    if n == 0 { return 0; }
    let counter = hanoi(n-1, from, by, to, depth + 1, previous);
    info!(current = previous + counter + 1, "Move");
    //let _indentation = " ".repeat(depth + 1);
    println!("{:>7}:{}->{}", previous + counter + 1, from, to);
    return counter + 1 + hanoi(n-1, by, to, from, depth + 1, previous + counter + 1);
}

fn main() {
    let formatter =
    format::debug_fn(
        |writer, field, value| 
            write!(writer, "{}: {:?},", field, value)
    );

    let logfile = tracing_appender::rolling::minutely("./logs", "recursion.txt");
    
    tracing_subscriber::fmt()
        .with_writer(logfile)
        .event_format(format().pretty().with_ansi(false))
        .fmt_fields(formatter)
        .init();

    let _ = hanoi(6, 'A', 'C', 'B', 0, 0);
}