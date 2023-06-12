pub fn ticks_to_seconds(ticks: usize) -> usize {
    ticks / 10_000_000
}

pub fn seconds_to_ticks(seconds: usize) -> usize {
    seconds * 10_000_000
}
