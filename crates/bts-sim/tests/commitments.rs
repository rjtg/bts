use bts_sim::{
    demo::{self, MAIN, SNACKS},
    economy::*,
    *,
};
fn world() -> World {
    let mut w = demo::world();
    w.stations.truncate(1);
    w.brands.truncate(1);
    w.programmes.truncate(2);
    w.audiences.truncate(2);
    let template = w.audiences[0].clone();
    for (i, a) in w.audiences.iter_mut().enumerate() {
        *a = template.clone();
        a.id = AudienceId(i as u16 + 1);
        a.population = 10000;
        a.availability = [1.0; 24];
        a.preferred_tone = 0.8;
        a.purchasing_power = 1.0;
        a.product_interests = [1.0; 2];
        a.genre_tastes = if i == 0 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 0.0, 1.0]
        };
    }
    for (i, p) in w.programmes.iter_mut().enumerate() {
        p.genre = if i == 0 {
            Genre::Comedy
        } else {
            Genre::Science
        };
        p.tone = 0.8;
        p.awareness = 0.0;
        p.cast_appeal = 0.0;
        p.execution = 1.0;
        p.broadcast_cost = Money(1);
    }
    w.brands[0].targets = vec![
        Target {
            audience: AudienceId(1),
            priority: 1.0,
            desired_hype: 5.0,
        },
        Target {
            audience: AudienceId(2),
            priority: 1.0,
            desired_hype: 85.0,
        },
    ];
    w.brands[0].rate_per_thousand = Money(1000);
    w.brands[0].maximum_offer = Money(10000);
    w
}
fn slot(programme: u16) -> Slot {
    Slot {
        at: SimHour(18),
        broadcasts: vec![Broadcast {
            station: MAIN,
            programme: ProgrammeId(programme),
            advertisement: Some(SNACKS),
        }],
    }
}
#[test]
fn satisfied_audience_cannot_substitute_for_purchased_underserved_viewers() {
    let mut sim = Simulation::new(world(), Parameters::default()).unwrap();
    let forecast = slot(2);
    let offer = sim.quote(&forecast, MAIN, SNACKS, forecast.at).unwrap();
    let id = sim.accept_offer(&offer).unwrap();
    let c = sim.state().economy().contract(id).unwrap();
    assert_eq!(
        c.commitments
            .iter()
            .map(|c| c.allocated_payment.0)
            .sum::<u64>(),
        offer.payment.0
    );
    assert_eq!(c.commitments[0].allocated_payment, Money(0));
    assert_eq!(c.commitments[1].allocated_payment, offer.payment);
    let event = sim.step(&slot(1)).unwrap();
    let actual = &sim.state().economy().contract(id).unwrap().actual;
    assert!((actual.relevant_delivery - offer.forecast.relevant_delivery).abs() < 1e-9);
    let settled = &event.financial.settlements[0];
    assert!(settled.payment.0 < offer.payment.0 / 4);
    assert!(settled.refund.0 > 0);
    assert_eq!(settled.audiences[0].payment, Money(0));
    assert!(settled.audiences[1].credited_units < settled.audiences[1].promised_units);
    assert_eq!(settled.payment.0 + settled.refund.0, offer.payment.0);
}
#[test]
fn capped_payment_apportionment_is_exact_deterministic_and_excludes_zero_value() {
    for cap in 1..40 {
        let mut w = world();
        let a = w.audiences[1].clone();
        w.audiences = (1..=4)
            .map(|i| {
                let mut a = a.clone();
                a.id = AudienceId(i);
                a
            })
            .collect();
        w.brands[0].targets = w
            .audiences
            .iter()
            .map(|a| Target {
                audience: a.id,
                priority: 1.0,
                desired_hype: if a.id == AudienceId(4) { 5.0 } else { 85.0 },
            })
            .collect();
        w.brands[0].maximum_offer = Money(cap);
        let mut sim = Simulation::new(w.clone(), Parameters::default()).unwrap();
        let action = slot(2);
        let offer = sim.quote(&action, MAIN, SNACKS, action.at).unwrap();
        let id = sim.accept_offer(&offer).unwrap();
        let rows = sim
            .state()
            .economy()
            .contract(id)
            .unwrap()
            .commitments
            .clone();
        assert_eq!(rows.iter().map(|r| r.allocated_payment.0).sum::<u64>(), cap);
        assert_eq!(rows[3].allocated_payment, Money(0));
        if cap == 10 {
            assert_eq!(
                rows.iter()
                    .map(|r| r.allocated_payment.0)
                    .collect::<Vec<_>>(),
                vec![3, 3, 4, 0]
            );
        }
        w.brands[0].targets.reverse();
        let mut other = Simulation::new(w, Parameters::default()).unwrap();
        let q = other.quote(&action, MAIN, SNACKS, action.at).unwrap();
        let i = other.accept_offer(&q).unwrap();
        assert_eq!(
            rows,
            other.state().economy().contract(i).unwrap().commitments
        );
        let total = sim.state().economy().total_money();
        let e = sim.step(&action).unwrap();
        assert_eq!(e.financial.settlements[0].payment, Money(cap));
        assert_eq!(e.financial.settlements[0].refund, Money(0));
        assert_eq!(sim.state().economy().total_money(), total);
    }
}
#[test]
fn positive_subwhole_delivery_is_payable_and_subquantum_delivery_is_explicitly_uncontractible() {
    for (priority, payable) in [(0.00001, true), (1e-12, false)] {
        let mut w = world();
        w.brands[0].targets = vec![Target {
            audience: AudienceId(2),
            priority,
            desired_hype: 85.0,
        }];
        w.brands[0].rate_per_thousand = Money(1_000_000_000_000);
        let mut sim = Simulation::new(w, Parameters::default()).unwrap();
        let a = slot(2);
        let q = sim.quote(&a, MAIN, SNACKS, a.at).unwrap();
        assert!(q.forecast.relevant_delivery > 0.0 && q.forecast.relevant_delivery < 1.0);
        assert_eq!(q.promised_delivery, 0);
        assert_eq!(q.valuation[0].contractible, payable);
        if payable {
            let id = sim.accept_offer(&q).unwrap();
            assert!(sim.state().economy().contract(id).unwrap().commitments[0].promised_units > 0);
            let e = sim.step(&a).unwrap();
            assert_eq!(e.financial.settlements[0].payment, q.payment);
        } else {
            assert_eq!(q.payment, Money(0));
            assert!(sim.accept_offer(&q).is_err());
        }
    }
}

#[test]
fn accepted_terms_are_frozen_against_mutation_of_the_callers_quote() {
    let mut sim = Simulation::new(world(), Parameters::default()).unwrap();
    let plan = slot(2);
    let mut quote = sim.quote(&plan, MAIN, SNACKS, plan.at).unwrap();
    let price = quote.payment;
    let id = sim.accept_offer(&quote).unwrap();
    let frozen = sim
        .state()
        .economy()
        .contract(id)
        .unwrap()
        .commitments
        .clone();
    quote.payment = Money(0);
    for a in &mut quote.forecast.audiences {
        a.priority = 0.0;
    }
    for v in &mut quote.valuation {
        v.deficit = 0.0;
        v.uncapped_value = 0.0;
    }
    let event = sim.step(&plan).unwrap();
    assert_eq!(event.financial.settlements[0].payment, price);
    assert_eq!(
        sim.state().economy().contract(id).unwrap().commitments,
        frozen
    );
}
