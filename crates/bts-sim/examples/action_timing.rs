//! Deterministic comparison of three capacity rules for a timed task.
//!
//! Elapsed minutes are relative to 17:00. The fixture fixes request order,
//! durations and deadlines so only the capacity policy varies.

#[derive(Clone, Copy, Debug)]
enum Policy {
    Immediate,
    Shared,
    ByDesk,
}

#[derive(Clone, Copy)]
enum Desk {
    Sales = 0,
    Studio = 1,
}

#[derive(Clone, Copy)]
struct Task {
    name: &'static str,
    desk: Desk,
    request: u16,
    duration: u16,
    closes: u16,
    cancel: Option<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Ready,
    Missed,
    Cancelled,
}

#[derive(Debug, PartialEq, Eq)]
struct Record {
    start: Option<u16>,
    released: u16,
    outcome: Outcome,
}

fn compare(policy: Policy, tasks: &[Task]) -> Vec<Record> {
    let mut available = [0; 2];
    tasks
        .iter()
        .map(|task| {
            let bucket = match policy {
                Policy::ByDesk => task.desk as usize,
                _ => 0,
            };
            let start = task.request.max(available[bucket]);
            let due = start
                + match policy {
                    Policy::Immediate => 0,
                    _ => task.duration,
                };
            let (outcome, released) =
                if let Some(cancel) = task.cancel.filter(|&cancel| cancel < due.min(task.closes)) {
                    (Outcome::Cancelled, cancel)
                } else if due >= task.closes {
                    (Outcome::Missed, task.closes)
                } else {
                    (Outcome::Ready, due)
                };
            available[bucket] = available[bucket].max(released);
            Record {
                start: (start < released || outcome == Outcome::Ready).then_some(start),
                released,
                outcome,
            }
        })
        .collect()
}

const LISTING: Task = Task {
    name: "Prepare early listing",
    desk: Desk::Sales,
    request: 15,
    duration: 30,
    closes: 60,
    cancel: None,
};
const STUDIO: Task = Task {
    name: "Reserve studio",
    desk: Desk::Studio,
    request: 15,
    duration: 15,
    closes: 40,
    cancel: None,
};
const SECOND_LISTING: Task = Task {
    name: "Prepare second listing",
    desk: Desk::Sales,
    request: 30,
    duration: 30,
    closes: 75,
    cancel: None,
};

fn main() {
    let fixtures = [
        ("Quiet afternoon", vec![LISTING]),
        (
            "Three competing commitments",
            vec![LISTING, STUDIO, SECOND_LISTING],
        ),
        (
            "Cancellation frees Sales",
            vec![
                Task {
                    cancel: Some(30),
                    ..LISTING
                },
                SECOND_LISTING,
            ],
        ),
        (
            "Completion exactly at airtime",
            vec![Task {
                request: 30,
                ..LISTING
            }],
        ),
    ];
    println!("All times are minutes after 17:00. Offers use a half-open selling window.");
    for (label, tasks) in fixtures {
        println!("\n{label}");
        for policy in [Policy::Immediate, Policy::Shared, Policy::ByDesk] {
            for (task, record) in tasks.iter().zip(compare(policy, &tasks)) {
                println!(
                    "{policy:?}\t{}\tstart {:?}\trelease {}\t{:?}",
                    task.name, record.start, record.released, record.outcome,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_capacity_preserves_independent_work() {
        let tasks = [LISTING, STUDIO, SECOND_LISTING];
        let states = |policy| {
            compare(policy, &tasks)
                .into_iter()
                .map(|item| item.outcome)
                .collect::<Vec<_>>()
        };
        assert_eq!(states(Policy::Immediate), [Outcome::Ready; 3]);
        assert_eq!(
            states(Policy::Shared),
            [Outcome::Ready, Outcome::Missed, Outcome::Missed]
        );
        assert_eq!(
            states(Policy::ByDesk),
            [Outcome::Ready, Outcome::Ready, Outcome::Missed]
        );
    }

    #[test]
    fn cancellation_releases_capacity_and_airtime_is_exclusive() {
        let cancelled = Task {
            cancel: Some(30),
            ..LISTING
        };
        let outcomes = compare(Policy::ByDesk, &[cancelled, SECOND_LISTING]);
        assert_eq!(outcomes[0].outcome, Outcome::Cancelled);
        assert_eq!(outcomes[1].start, Some(30));
        assert_eq!(outcomes[1].outcome, Outcome::Ready);
        let tied = Task {
            request: 30,
            ..LISTING
        };
        assert_eq!(compare(Policy::ByDesk, &[tied])[0].outcome, Outcome::Missed);
    }
}
