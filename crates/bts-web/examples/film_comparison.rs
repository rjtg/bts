use bts_sim::planning::film::FilmBooking;
use bts_sim::planning::production::{PersonId, Subject};
use bts_sim::SimTick;
use serde_json::{json, Value};

const MODEST: [f64; 7] = [0.4, 0.35, 0.35, 0.75, 0.7, 0.12, 0.45];
const BOLD: [f64; 7] = [0.2, 0.2, 0.93, 0.32, 0.48, 0.95, 0.86];

fn run(label: &str, ambition: [f64; 7], writer: u16, revise: bool, studio: u8) -> Value {
    let mut p = bts_web::scenario::planner().unwrap();
    let id = p
        .pitch_film(
            "A Light in the Window".into(),
            Subject::EverydayLife,
            ambition,
        )
        .unwrap();
    p.write_film(id, PersonId(writer), None).unwrap();
    p.advance_to(SimTick(48)).unwrap();
    let initial = p.film(id).unwrap().script.as_ref().unwrap().profile;
    if revise {
        p.write_film(id, PersonId(writer), Some(5)).unwrap();
        p.advance_to(SimTick(80)).unwrap();
    }
    p.accept_script(id).unwrap();
    p.book_film(
        id,
        FilmBooking {
            cast: [PersonId(105), PersonId(106)],
            director: PersonId(104),
            studio,
            start: SimTick(88),
        },
    )
    .unwrap();
    p.advance_to(SimTick(112)).unwrap();
    let film = p.film(id).unwrap();
    let evidence = film.evidence.as_ref().unwrap();
    json!({
        "case": label, "ambition": ambition, "writer": writer,
        "first_script": initial, "final_script": film.script.as_ref().unwrap().profile,
        "weaknesses": film.script.as_ref().unwrap().weaknesses,
        "equipment_demand": film.script.as_ref().unwrap().equipment_demand,
        "finished": evidence.profile, "execution": evidence.execution,
        "equipment_fit": evidence.equipment, "floor_fit": evidence.floor, "spectacle_support": evidence.spectacle_support,
        "committed_credits": film.committed.0, "money_conserved": p.state().economy().total_money(),
    })
}

fn main() {
    let cases = [
        run("modest, Ada, Backlot", MODEST, 101, false, 1),
        run("demanding, Ada, Backlot", BOLD, 101, false, 1),
        run("demanding, Ada, revised, Backlot", BOLD, 101, true, 1),
        run("demanding, Ada, revised, Soundstage", BOLD, 101, true, 2),
        run("demanding, Leon, Backlot", BOLD, 102, false, 1),
    ];
    println!("{}", serde_json::to_string_pretty(&cases).unwrap());
}
