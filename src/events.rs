use regex::Regex;

#[derive(Debug, Clone)]
pub struct Event {
    pub year: i32,
    pub month: u32, // 1-12
    pub day: u32,   // 1-31
    pub title: String,
    pub source: String,
}

fn month_to_num(m: &str) -> Option<u32> {
    match m {
        "JANUARY" => Some(1),
        "FEBRUARY" => Some(2),
        "MARCH" => Some(3),
        "APRIL" => Some(4),
        "MAY" => Some(5),
        "JUNE" => Some(6),
        "JULY" => Some(7),
        "AUGUST" => Some(8),
        "SEPTEMBER" => Some(9),
        "OCTOBER" => Some(10),
        "NOVEMBER" => Some(11),
        "DECEMBER" => Some(12),
        _ => None,
    }
}

fn is_weekday(line: &str) -> bool {
    matches!(
        line,
        "SUNDAY" | "MONDAY" | "TUESDAY" | "WEDNESDAY" | "THURSDAY" | "FRIDAY" | "SATURDAY"
    )
}

/// Parses events using simple rules tailored to your calendar formatting:
/// - Month header like: "JANUARY 202 6" (digits may be spaced)
/// - Day number lines like: "1", "2", ...
/// - Any non-empty line after a day number is an event title until the next day/month.
pub fn parse_events_from_text(doc_name: &str, text: &str) -> Vec<Event> {
    let mut events = Vec::new();

    let month_header_re = Regex::new(
        r"^(JANUARY|FEBRUARY|MARCH|APRIL|MAY|JUNE|JULY|AUGUST|SEPTEMBER|OCTOBER|NOVEMBER|DECEMBER)\s+([0-9 ]{4,8})\s*$"
    ).unwrap();

    let day_re = Regex::new(r"^\d{1,2}\s*$").unwrap();

    let mut current_year: Option<i32> = None;
    let mut current_month: Option<u32> = None;
    let mut current_day: Option<u32> = None;

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        // Month header
        if let Some(cap) = month_header_re.captures(line) {
            let month_str = cap.get(1).unwrap().as_str();
            let year_str = cap.get(2).unwrap().as_str().replace(' ', "");
            if let (Some(m), Ok(y)) = (month_to_num(month_str), year_str.parse::<i32>()) {
                current_month = Some(m);
                current_year = Some(y);
                current_day = None;
            }
            continue;
        }

        // Skip weekday header lines
        if is_weekday(line) {
            continue;
        }

        // Day number
        if day_re.is_match(line) {
            current_day = line.parse::<u32>().ok();
            continue;
        }

        // Event title
        if let (Some(y), Some(m), Some(d)) = (current_year, current_month, current_day) {
            if line.len() >= 3 {
                events.push(Event {
                    year: y,
                    month: m,
                    day: d,
                    title: line.to_string(),
                    source: doc_name.to_string(),
                });
            }
        }
    }

    events
}