use feed_transformer::stream::StreamIterator;

const TOP_LEVEL_KEY: &str = "events";

fn main() {
    let stream_iterator = StreamIterator::new("./files/events.json", TOP_LEVEL_KEY).unwrap();
    println!("counting events!");

    let music_events = stream_iterator.filter(|event| event.as_ref().is_ok_and(|e| e.is_music()));

    for event in music_events {
        if let Ok(event) = event {
            println!("event - {:?}", event)
        }
    }
}
