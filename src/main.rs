use feed_transformer::stream::StreamIterator;
use std::env;

const TOP_LEVEL_KEY: &str = "events";

fn parse_file_path() -> Option<String> {
    let mut args = env::args();
    args.next();
    args.next()
}

fn main() {
    let file_path = parse_file_path().unwrap();
    let stream_iterator = StreamIterator::new(&file_path, TOP_LEVEL_KEY).unwrap();
    println!("{}", stream_iterator.count());
}
