//! Same varied evening used by the browser test, exported through the player boundary.
use bts_web::protocol::Command;
use bts_web::Session;
fn main() {
    let mut s = Session::new().unwrap();
    for (programme, start_tick) in [(1, 72), (2, 74), (5, 80)] {
        s.execute(Command::Place {
            programme,
            start_tick,
            advertiser: if programme == 5 { Some(1) } else { None },
        })
        .unwrap();
    }
    let movie = s
        .report()
        .schedule
        .iter()
        .find(|a| a.station == 1 && a.programme == 5)
        .unwrap()
        .airing_id;
    let token = s
        .execute(Command::RequestOffer { airing_id: movie })
        .unwrap()
        .offer
        .unwrap()
        .offer_id;
    s.execute(Command::AcceptOffer {
        airing_id: movie,
        offer_id: token,
    })
    .unwrap();
    let report = s.execute(Command::Advance { to_tick: 96 }).unwrap();
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
