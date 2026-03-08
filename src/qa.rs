use crate::events::Event;
use regex::Regex;

fn month_name(m: u32) -> &'static str {
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

fn normalize(s: &str) -> String {
    let s = s.to_lowercase().replace('’', "'");

    // keep only letters, numbers, and spaces (remove punctuation)
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Answers:
/// - Date/event questions: "When is <event> in 2026?"
/// - Count questions: "How many times did <keyword> meet in 2024?"
pub fn answer_question(question: &str, events: &[Event]) -> String {
    let q = normalize(question);

    // COUNT questions
    let count_re = Regex::new(
        r"how many times did (the )?(?P<kw>.+?) (meet|hold|have|occur).*(in|for)\s+(?P<year>\d{4})"
    ).unwrap();

    if let Some(c) = count_re.captures(&q) {
        let kw = c.name("kw").unwrap().as_str().trim();
        let year: i32 = c.name("year").unwrap().as_str().parse().unwrap_or(0);

        let count = events
            .iter()
            .filter(|e| e.year == year)
            .filter(|e| normalize(&e.title).contains(kw))
            .count();

        return format!("Count in {} for '{}': {}", year, kw, count);
    }

    // DATE / EVENT questions
    let when_re = Regex::new(
        r"(when is|date of|what is the date of)\s+(?P<ev>.+?)(\s+in\s+(?P<year>\d{4}))?\??$"
    ).unwrap();

    if let Some(c) = when_re.captures(&q) {
        let ev_raw = c.name("ev").unwrap().as_str().trim();
        let ev = normalize(ev_raw);
        let year_opt = c
            .name("year")
            .and_then(|m| m.as_str().parse::<i32>().ok());

        let mut matches: Vec<&Event> = events
            .iter()
            .filter(|e| year_opt.is_none() || Some(e.year) == year_opt)
            .filter(|e| normalize(&e.title).contains(&ev))
            .collect();

        matches.sort_by_key(|e| (e.year, e.month, e.day));

        if matches.is_empty() {
            return format!("No match found for '{}'. Try different wording.", ev);
        }

        let mut out = String::new();
        for (i, e) in matches.iter().take(10).enumerate() {
            out.push_str(&format!(
                "{}. {} {} {}, {} (from {})\n",
                i + 1,
                month_name(e.month),
                e.day,
                e.year,
                e.title,
                e.source
            ));
        }
        if matches.len() > 10 {
            out.push_str(&format!("(Showing 10 of {})\n", matches.len()));
        }
        return out;
    }

    // fallback: keyword search
    let kw = q.trim();
    let mut hits: Vec<&Event> = events
        .iter()
        .filter(|e| normalize(&e.title).contains(kw))
        .collect();

    hits.sort_by_key(|e| (e.year, e.month, e.day));

    if hits.is_empty() {
        "Try: 'When is <event> in 2026?' or 'How many times did <keyword> meet in 2024?'"
            .to_string()
    } else {
        let mut out = String::new();
        for (i, e) in hits.iter().take(10).enumerate() {
            out.push_str(&format!(
                "{}. {} {} {}, {} (from {})\n",
                i + 1,
                month_name(e.month),
                e.day,
                e.year,
                e.title,
                e.source
            ));
        }
        out
    }
}