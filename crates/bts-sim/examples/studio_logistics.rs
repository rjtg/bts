//! Independent setup and stock counterfactuals for the rental pilot.
use bts_sim::economy::Money;

#[derive(Clone, Copy)]
struct Recording {
    show: u8,
    set: u8,
    start: u32,
    end: u32,
}

fn setup_cost(recordings: &[Recording], retain_set: bool) -> Money {
    const SETUP_AND_STRIKE: u64 = 120 + 80;
    let continuations = recordings
        .windows(2)
        .filter(|pair| {
            let [first, next] = [pair[0], pair[1]];
            retain_set
                && first.end == next.start
                && first.show == next.show
                && first.set == next.set
        })
        .count();
    Money(SETUP_AND_STRIKE * (recordings.len() - continuations) as u64)
}

fn prop_cost(recordings: &[Recording], retain_props: bool) -> Money {
    const FABRICATION: u64 = 160;
    const PER_USE: u64 = 20;
    let mut copies = Vec::<(u8, u32)>::new();
    for recording in recordings {
        if let Some((_, available)) = copies.iter_mut().find(|(set, available)| {
            retain_props && *set == recording.set && *available <= recording.start
        }) {
            *available = recording.end;
        } else {
            copies.push((recording.set, recording.end));
        }
    }
    Money(FABRICATION * copies.len() as u64 + PER_USE * recordings.len() as u64)
}

fn main() {
    let first = Recording {
        show: 1,
        set: 1,
        start: 40,
        end: 48,
    };
    let next = Recording {
        start: 48,
        end: 56,
        ..first
    };
    let adjacent = [first, next];
    let separated = [
        first,
        Recording {
            start: 56,
            end: 64,
            ..next
        },
    ];
    let overlapping = [
        first,
        Recording {
            start: 44,
            end: 52,
            ..next
        },
    ];
    let setup = [
        setup_cost(&adjacent, false),
        setup_cost(&adjacent, true),
        setup_cost(&separated, true),
    ];
    let props = [
        prop_cost(&separated, false),
        prop_cost(&separated, true),
        prop_cost(&overlapping, true),
    ];
    assert_eq!(setup.map(|cost| cost.0), [400, 200, 400]);
    assert_eq!(props.map(|cost| cost.0), [360, 200, 360]);
    assert_eq!(
        setup_cost(&[first, Recording { set: 2, ..next }], true),
        Money(400)
    );
    println!("Same two recordings, changing setup only (Funds): individually {}, adjacent with retained set {}, interrupted {}.", setup[0].0, setup[1].0, setup[2].0);
    println!("Same two uses, changing props only (Funds): fabricate twice {}, retain stock {}, overlapping reservations {}.", props[0].0, props[1].0, props[2].0);
}
