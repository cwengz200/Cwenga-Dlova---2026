use crate::events::Event;

pub fn month_name(m: u32) -> &'static str {
    match m {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

/// Build samples for a simple supervised task:
/// Input text contains the month + title, label is the month index (0..11).
pub fn build_month_samples(events: &[Event]) -> Vec<(String, usize)> {
    events
        .iter()
        .filter(|e| (1..=12).contains(&e.month))
        .map(|e| {
            let text = format!("MONTH={} TITLE={}", month_name(e.month), e.title);
            let label = (e.month - 1) as usize;
            (text, label)
        })
        .collect()
}