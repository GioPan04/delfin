pub fn ticks_to_seconds(ticks: i64) -> isize {
    ticks as isize / 10_000_000
}

pub fn seconds_to_ticks(seconds: usize) -> usize {
    seconds * 10_000_000
}
