use regex::Regex;
use std::collections::HashMap;

use std::cmp::Ordering;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"\[\s*(\d+)\-(\d+)\-(\d+)\s+(\d+):(\d+)\s*").unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DateTime {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl<'a> From<&'a str> for DateTime {
    fn from(input: &'a str) -> Self {
        let groups = PATTERN
            .captures(input)
            .expect("Expected all date times to match the regex.");
        assert!(
            groups.len() == 6,
            "Expected six groups for each input found {} for {}",
            groups.len(),
            input
        );

        Self {
            year: groups[1].parse::<usize>().expect("Expected a valid year"),
            month: groups[2].parse::<usize>().expect("Expected a valid month"),
            day: groups[3].parse::<usize>().expect("Expected a valid day"),
            hour: groups[4].parse::<usize>().expect("Expected a valid hour"),
            minute: groups[5].parse::<usize>().expect("Expected a valid minute"),
        }
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> Ordering {
        let components = [self.year, self.month, self.day, self.hour, self.minute];
        let other_components = [other.year, other.month, other.day, other.hour, other.minute];

        components
            .into_iter()
            .zip(other_components.into_iter())
            .map(|(lhs, rhs)| lhs.cmp(rhs))
            .skip_while(|&order| order == Ordering::Equal)
            .nth(0)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
enum Event {
    FellAsleep,
    WokeUp,
    StartShift { id: usize },
}

impl<'a> From<&'a str> for Event {
    fn from(input: &'a str) -> Self {
        if input.contains("falls asleep") {
            Event::FellAsleep
        } else if input.contains("wakes up") {
            Event::WokeUp
        } else {
            let id = input
                .split("#")
                .nth(1)
                .expect(&format!(
                    "Expected a parsable guard id, but found none in {}",
                    input
                )).chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .expect(&format!(
                    "Expected a parsable guard id, but found none in {}",
                    input
                ));
            Event::StartShift { id: id }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Record {
    at: DateTime,
    event: Event,
}

impl<'a> From<&'a str> for Record {
    fn from(input: &str) -> Self {
        let parts = input.split("]").collect::<Vec<_>>();
        assert!(
            parts.len() == 2,
            "Each record should have two parts when split at `]`. Found {} for {}",
            input.len(),
            input
        );

        Self {
            at: DateTime::from(parts[0]),
            event: Event::from(parts[1]),
        }
    }
}

fn parse(input: &str) -> Vec<Record> {
    let mut records = input
        .lines()
        .filter(|l| l.len() > 0)
        .map(Record::from)
        .collect::<Vec<_>>();
    records.sort_by_key(|r| r.at);

    records
}

pub fn star_one(input: &str) -> usize {
    let records = parse(input);

    let mut total_minutes_asleep = HashMap::<usize, usize>::new();
    let mut asleep_per_minute_count = HashMap::<usize, Vec<usize>>::new();
    let mut current_asleep_record: Option<Record> = None;
    let mut active_guard_id: Option<usize> = None;

    for record in records {
        match record.event {
            Event::WokeUp => {
                let asleep_record =
                    current_asleep_record.expect("Someone must be asleep before waking up");

                match asleep_record.event {
                    Event::FellAsleep => {
                        let counter = total_minutes_asleep
                            .entry(
                                active_guard_id
                                    .expect("Can't wake up with no active guard on duty"),
                            ).or_insert(0);

                        *counter += record.at.minute - asleep_record.at.minute - 1;
                        let per_minute_count = asleep_per_minute_count
                            .entry(
                                active_guard_id
                                    .expect("Can't wake up with no active guard on duty"),
                            ).or_insert(vec![0; 60]);
                        (asleep_record.at.minute..record.at.minute).for_each(|minute| {
                            per_minute_count[minute] += 1;
                        });
                        current_asleep_record = None;
                    }
                    _ => assert!(false, "Invalid asleep record {:?}", asleep_record),
                }
            }
            Event::FellAsleep => {
                current_asleep_record = Some(record.clone());
            }
            Event::StartShift { id } => {
                active_guard_id = Some(id);
            }
        }
    }

    let (id, _) = total_minutes_asleep
        .iter()
        .max_by_key(|(_, &minutes)| minutes)
        .unwrap();

    let (most_slept_minute, _) = asleep_per_minute_count
        .get(&id)
        .unwrap()
        .iter()
        .enumerate()
        .max_by_key(|(_, &count)| count)
        .unwrap();

    id * most_slept_minute
}

pub fn star_two(input: &str) -> usize {
    let records = parse(input);

    let mut asleep_per_minute_count = HashMap::<usize, Vec<usize>>::new();
    let mut current_asleep_record: Option<Record> = None;
    let mut active_guard_id: Option<usize> = None;

    for record in records {
        match record.event {
            Event::WokeUp => {
                let asleep_record =
                    current_asleep_record.expect("Someone must be asleep before waking up");

                match asleep_record.event {
                    Event::FellAsleep => {
                        let per_minute_count = asleep_per_minute_count
                            .entry(
                                active_guard_id
                                    .expect("Can't wake up with no active guard on duty"),
                            ).or_insert(vec![0; 60]);
                        (asleep_record.at.minute..record.at.minute).for_each(|minute| {
                            per_minute_count[minute] += 1;
                        });
                        current_asleep_record = None;
                    }
                    _ => assert!(false, "Invalid asleep record {:?}", asleep_record),
                }
            }
            Event::FellAsleep => {
                current_asleep_record = Some(record.clone());
            }
            Event::StartShift { id } => {
                active_guard_id = Some(id);
            }
        }
    }

    let (id, (most_slept_minute, _)) = asleep_per_minute_count
        .iter()
        .map(|(id, minutes)| {
            let result = minutes.iter().enumerate().max_by_key(|&(_, c)| c).unwrap();
            (id, result)
        }).max_by_key(|(_, (_, &c))| c)
        .unwrap();

    id * most_slept_minute
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    static EXAMPLE: &'static str = r#"
[1518-11-01 00:30] falls asleep
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(EXAMPLE), 240);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(EXAMPLE), 4455)
    }
}
