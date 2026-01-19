#[allow(dead_code)]
pub struct SkEvent {
    id: String,
    title: String,
    start_date: Date,
    start_time: Time,
}

#[allow(dead_code)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
}

#[allow(dead_code)]
struct Time {
    hour: i32,
    min: i32,
    sec: i32,
}
