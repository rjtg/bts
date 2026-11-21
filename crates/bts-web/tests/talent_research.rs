use bts_sim::planning::production::Subject;
use bts_web::{protocol::Command, Session};

#[test]
fn script_research_and_auditions_are_optional_dated_and_replayable() {
    let mut live = Session::new().unwrap();
    let before = live.report().research.unwrap().insight;
    let created = live
        .execute(Command::PitchFilm {
            title: "Evening Lights".into(),
            subject: Subject::EverydayLife,
            ambition: [0.45; 7],
        })
        .unwrap();
    let id = created.film_desk.projects.last().unwrap().id;
    let writing = live
        .execute(Command::WriteFilm {
            film_id: id,
            writer: 101,
            focus: None,
        })
        .unwrap();
    let script = writing.film_desk.projects.last().unwrap().due.unwrap();
    live.execute(Command::Advance { to_tick: script }).unwrap();
    let studying = live
        .execute(Command::StudyScript { film_id: id })
        .unwrap()
        .research
        .unwrap();
    assert_eq!(studying.insight, before);
    assert_eq!(studying.script_studies[0].ready.0, script + 4);
    assert!(live.execute(Command::StudyScript { film_id: id }).is_err());
    live.execute(Command::AcceptScript { film_id: id }).unwrap();
    let collected = live
        .execute(Command::Advance {
            to_tick: script + 4,
        })
        .unwrap();
    assert_eq!(collected.research.as_ref().unwrap().insight, before + 1);
    assert!(collected.results.is_empty());
    let reading = live
        .execute(Command::Audition {
            film_id: id,
            person: 105,
            role: 0,
            insight: 3,
        })
        .unwrap();
    assert_eq!(reading.research.as_ref().unwrap().insight, before - 2);
    let evidence = &reading.research.as_ref().unwrap().auditions[0];
    assert_eq!(evidence.observed.0, script + 4);
    assert_eq!(evidence.uncertainty, "Moderate");
    let public = serde_json::to_value(evidence).unwrap();
    assert!(public["fit"].is_string());
    assert!(public.get("capability").is_none());
    assert!(live
        .execute(Command::Audition {
            film_id: id,
            person: 105,
            role: 0,
            insight: 3
        })
        .is_err());
    assert_eq!(live.report().research, reading.research);
    let mut restored = Session::new().unwrap();
    assert_eq!(
        restored
            .restore_game(&live.save_game().unwrap())
            .unwrap()
            .research,
        reading.research
    );
}
