//! Bounded advertiser delivery contracts, integer accounts, and inspectable offers.
//! No market clearing, credit, budget replenishment, or player information layer.
//!
//! ```
//! use bts_sim::{demo, economy::ContractStatus, Parameters, Simulation};
//! let mut simulation = Simulation::new(demo::world(), Parameters::default())?;
//! let action = demo::schedule(demo::Case::Broad).remove(0);
//! let offer = simulation.quote(&action, demo::MAIN, demo::EQUIPMENT, action.at)?;
//! let id = simulation.accept_offer(&offer)?;
//! simulation.step(&action)?;
//! assert!(matches!(simulation.state().economy().contract(id).unwrap().status,
//!                  ContractStatus::Settled(_)));
//! # Ok::<(), bts_sim::Error>(())
//! ```
use crate::*;

/// Nonnegative integer credits. Configuration amounts are capped at 10^12.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Money(pub u64);
impl Money {
    pub const MAX_CONFIGURED: u64 = 1_000_000_000_000;
    pub(crate) fn validate(self) -> Result<(), Error> {
        if self.0 > Self::MAX_CONFIGURED {
            Err(Error::Invalid(
                "configured money exceeds 10^12 credits".into(),
            ))
        } else {
            Ok(())
        }
    }
    fn add(self, other: Self) -> Result<Self, Error> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or_else(|| Error::Invalid("money addition overflow".into()))
    }
    fn subtract(self, other: Self) -> Result<Self, Error> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or_else(|| Error::Invalid("insufficient account funds".into()))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ContractId(pub u64);
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Target {
    pub audience: AudienceId,
    pub priority: f64,
    pub desired_hype: f64,
}
pub(crate) fn validate_brand(brand: &Brand, audiences: &[Audience]) -> Result<(), Error> {
    for value in [
        brand.marketing_budget,
        brand.maximum_offer,
        brand.rate_per_thousand,
    ] {
        value.validate()?;
    }
    unique(brand.targets.iter().map(|t| t.audience), "brand target")?;
    for target in &brand.targets {
        if !audiences.iter().any(|a| a.id == target.audience) {
            return Err(Error::Invalid("unknown target audience".into()));
        }
        unit(target.priority, "target priority")?;
        bounded(target.desired_hype, 0.0, 100.0, "desired hype")?;
    }
    Ok(())
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TargetDemand {
    pub audience: AudienceId,
    pub priority: f64,
    pub desired_hype: f64,
    pub current_hype: f64,
    pub deficit: f64,
    pub weighted_deficit: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Demand {
    pub brand: BrandId,
    pub at: SimHour,
    pub targets: Vec<TargetDemand>,
    pub total_priority: f64,
    pub pressure: f64,
    pub available_budget: Money,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ForecastAudience {
    pub audience: AudienceId,
    pub priority: f64,
    pub viewers: f64,
    pub effectiveness: ModelEvaluation,
    pub relevant_delivery: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeliveryForecast {
    pub audiences: Vec<ForecastAudience>,
    pub relevant_delivery: f64,
}
/// Inspectable quote; acceptance recomputes and compares every field.
#[derive(Clone, Debug, PartialEq)]
pub struct Offer {
    pub brand: BrandId,
    pub station: StationId,
    pub deadline: SimHour,
    pub demand: Demand,
    pub forecast: DeliveryForecast,
    pub promised_delivery: u64,
    pub rate_per_thousand: Money,
    pub maximum_offer: Money,
    pub uncapped_price: f64,
    pub payment: Money,
    proposal: Slot,
    revision: u64,
}
impl Offer {
    pub fn proposal(&self) -> &Slot {
        &self.proposal
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActualAudience {
    pub audience: AudienceId,
    pub priority: f64,
    pub viewers: f64,
    pub effectiveness: f64,
    pub relevant_delivery: f64,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ActualDelivery {
    pub audiences: Vec<ActualAudience>,
    pub relevant_delivery: f64,
    pub credited_units: u64,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Settlement {
    pub contract: ContractId,
    pub at: SimHour,
    pub promised_delivery: u64,
    pub credited_delivery: u64,
    pub payment: Money,
    pub refund: Money,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ContractStatus {
    Reserved,
    Settled(Settlement),
}
#[derive(Clone, Debug, PartialEq)]
pub struct Contract {
    pub id: ContractId,
    pub offer: Offer,
    pub actual: ActualDelivery,
    pub delivered_at: Option<SimHour>,
    pub status: ContractStatus,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Account {
    Brand(BrandId),
    Escrow(ContractId),
    Station(StationId),
    BroadcastCosts,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum LedgerReason {
    Reserve,
    DeliveryPayment,
    UnderdeliveryRefund,
    BroadcastCost,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LedgerEntry {
    pub at: SimHour,
    pub from: Account,
    pub to: Account,
    pub amount: Money,
    pub reason: LedgerReason,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct FinancialEvent {
    pub ledger: Vec<LedgerEntry>,
    pub deliveries: Vec<(ContractId, ActualDelivery)>,
    pub settlements: Vec<Settlement>,
}
/// Internal balances and obligations. No serialization: this is not a save format.
#[derive(Clone, Debug, PartialEq)]
pub struct EconomyState {
    budgets: BTreeMap<BrandId, Money>,
    cash: BTreeMap<StationId, Money>,
    cost_sink: Money,
    contracts: BTreeMap<ContractId, Contract>,
    sold: BTreeMap<(SimHour, StationId), ContractId>,
    ledger: Vec<LedgerEntry>,
    revision: u64,
    next_contract: u64,
}
impl EconomyState {
    pub(crate) fn new(world: &World) -> Self {
        Self {
            budgets: world
                .brands
                .iter()
                .map(|b| (b.id, b.marketing_budget))
                .collect(),
            cash: world
                .stations
                .iter()
                .map(|s| (s.id, s.initial_cash))
                .collect(),
            cost_sink: Money(0),
            contracts: BTreeMap::new(),
            sold: BTreeMap::new(),
            ledger: vec![],
            revision: 0,
            next_contract: 1,
        }
    }
    pub fn budget(&self, brand: BrandId) -> Option<Money> {
        self.budgets.get(&brand).copied()
    }
    pub fn cash(&self, station: StationId) -> Option<Money> {
        self.cash.get(&station).copied()
    }
    pub fn cost_sink(&self) -> Money {
        self.cost_sink
    }
    pub fn ledger(&self) -> &[LedgerEntry] {
        &self.ledger
    }
    pub fn contracts(&self) -> impl Iterator<Item = &Contract> {
        self.contracts.values()
    }
    pub fn contract(&self, id: ContractId) -> Option<&Contract> {
        self.contracts.get(&id)
    }
    pub fn escrow(&self) -> u128 {
        self.contracts
            .values()
            .filter(|c| matches!(c.status, ContractStatus::Reserved))
            .map(|c| u128::from(c.offer.payment.0))
            .sum()
    }
    /// Available budgets + station cash + escrow + broadcast-cost sink.
    pub fn total_money(&self) -> u128 {
        self.budgets
            .values()
            .chain(self.cash.values())
            .map(|m| u128::from(m.0))
            .sum::<u128>()
            + self.escrow()
            + u128::from(self.cost_sink.0)
    }
    pub(crate) fn advance_revision(&mut self) -> Result<(), Error> {
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| Error::Invalid("state revision overflow".into()))?;
        Ok(())
    }
    fn transfer(
        &mut self,
        at: SimHour,
        from: Account,
        to: Account,
        amount: Money,
        reason: LedgerReason,
    ) {
        if amount.0 > 0 {
            self.ledger.push(LedgerEntry {
                at,
                from,
                to,
                amount,
                reason,
            });
        }
    }
}
fn whole_delivery(value: f64) -> Result<u64, Error> {
    if !value.is_finite() || value < 0.0 || value >= u64::MAX as f64 {
        return Err(Error::Invalid(
            "target delivery cannot be represented as whole units".into(),
        ));
    }
    Ok(value.floor() as u64)
}
impl Simulation {
    /// Current actual hype pressure; finite budget is shown separately.
    pub fn demand(&self, brand: BrandId) -> Result<Demand, Error> {
        let brand = self
            .world
            .brands
            .iter()
            .find(|b| b.id == brand)
            .ok_or_else(|| Error::Invalid("unknown advertiser".into()))?;
        let targets: Vec<_> = brand
            .targets
            .iter()
            .map(|t| {
                let current_hype = self.state.hype[&(brand.id, t.audience)];
                let deficit = (t.desired_hype - current_hype).max(0.0) / 100.0;
                TargetDemand {
                    audience: t.audience,
                    priority: t.priority,
                    desired_hype: t.desired_hype,
                    current_hype,
                    deficit,
                    weighted_deficit: t.priority * deficit,
                }
            })
            .collect();
        let total_priority = targets.iter().map(|t| t.priority).sum::<f64>();
        let pressure = if total_priority > 0.0 {
            targets.iter().map(|t| t.weighted_deficit).sum::<f64>() / total_priority
        } else {
            0.0
        };
        unit(pressure, "demand pressure")?;
        Ok(Demand {
            brand: brand.id,
            at: self.state.now,
            targets,
            total_priority,
            pressure,
            available_budget: self.state.economy.budgets[&brand.id],
        })
    }
    /// Read-only complete-schedule forecast; never debits costs or changes state.
    pub fn quote(
        &self,
        proposal: &Slot,
        station: StationId,
        brand: BrandId,
        deadline: SimHour,
    ) -> Result<Offer, Error> {
        if deadline < proposal.at {
            return Err(Error::Invalid("deadline precedes booked broadcast".into()));
        }
        if self
            .state
            .economy
            .sold
            .contains_key(&(proposal.at, station))
        {
            return Err(Error::Invalid("advertising slot already sold".into()));
        }
        let broadcast = proposal
            .broadcasts
            .iter()
            .find(|b| b.station == station)
            .ok_or_else(|| Error::Invalid("quote requires a scheduled station".into()))?;
        if broadcast.advertisement != Some(brand) {
            return Err(Error::Invalid(
                "quoted broadcast must carry this advertiser".into(),
            ));
        }
        validate_inventory(&self.state.economy, proposal)?;
        let demand = self.demand(brand)?;
        let advertiser = self.world.brands.iter().find(|b| b.id == brand).unwrap();
        let (_, event) = self.evaluate_slot(proposal)?;
        let audiences = advertiser
            .targets
            .iter()
            .map(|t| {
                let hype = event
                    .hype
                    .iter()
                    .find(|h| h.brand == brand && h.audience == t.audience)
                    .unwrap();
                let ad = hype
                    .advertisements
                    .iter()
                    .find(|a| a.station == station)
                    .unwrap();
                ForecastAudience {
                    audience: t.audience,
                    priority: t.priority,
                    viewers: ad.viewers,
                    effectiveness: ad.effectiveness.clone(),
                    relevant_delivery: ad.viewers * t.priority * ad.effectiveness.trace.output,
                }
            })
            .collect::<Vec<_>>();
        let relevant_delivery = audiences.iter().map(|a| a.relevant_delivery).sum::<f64>();
        let promised_delivery = whole_delivery(relevant_delivery)?;
        let uncapped_price =
            advertiser.rate_per_thousand.0 as f64 * relevant_delivery / 1000.0 * demand.pressure;
        bounded(uncapped_price, 0.0, f64::MAX, "offer price arithmetic")?;
        let cap = advertiser.maximum_offer.0.min(demand.available_budget.0);
        let payment = Money(uncapped_price.min(cap as f64).floor() as u64);
        Ok(Offer {
            brand,
            station,
            deadline,
            demand,
            forecast: DeliveryForecast {
                audiences,
                relevant_delivery,
            },
            promised_delivery,
            rate_per_thousand: advertiser.rate_per_thousand,
            maximum_offer: advertiser.maximum_offer,
            uncapped_price,
            payment,
            proposal: proposal.clone(),
            revision: self.state.economy.revision,
        })
    }
    /// Reserve a fresh quote atomically; no application-supplied prices are trusted.
    pub fn accept_offer(&mut self, offer: &Offer) -> Result<ContractId, Error> {
        if &self.quote(&offer.proposal, offer.station, offer.brand, offer.deadline)? != offer {
            return Err(Error::Invalid("stale or modified offer".into()));
        }
        if offer.payment.0 == 0 || offer.promised_delivery == 0 {
            return Err(Error::Invalid("offer has no payable delivery".into()));
        }
        let mut next = self.state.economy.clone();
        let id = ContractId(next.next_contract);
        next.next_contract = next
            .next_contract
            .checked_add(1)
            .ok_or_else(|| Error::Invalid("contract identifier overflow".into()))?;
        next.budgets.insert(
            offer.brand,
            next.budgets[&offer.brand].subtract(offer.payment)?,
        );
        next.sold.insert((offer.proposal.at, offer.station), id);
        next.transfer(
            self.state.now,
            Account::Brand(offer.brand),
            Account::Escrow(id),
            offer.payment,
            LedgerReason::Reserve,
        );
        next.contracts.insert(
            id,
            Contract {
                id,
                offer: offer.clone(),
                actual: ActualDelivery::default(),
                delivered_at: None,
                status: ContractStatus::Reserved,
            },
        );
        next.advance_revision()?;
        self.state.economy = next;
        Ok(id)
    }
}
fn validate_inventory(book: &EconomyState, action: &Slot) -> Result<(), Error> {
    for broadcast in &action.broadcasts {
        if let Some(id) = book.sold.get(&(action.at, broadcast.station)) {
            let brand = book.contracts[id].offer.brand;
            if broadcast
                .advertisement
                .is_some_and(|actual| actual != brand)
            {
                return Err(Error::Invalid(
                    "advertiser conflicts with reserved slot".into(),
                ));
            }
        }
    }
    Ok(())
}
/// Called only against a tentative state; the caller commits after every check.
pub(crate) fn apply_slot(
    world: &World,
    book: &mut EconomyState,
    event: &SlotEvent,
) -> Result<FinancialEvent, Error> {
    let action = &event.action;
    validate_inventory(book, action)?;
    let start = book.ledger.len();
    for station in &world.stations {
        if let Some(broadcast) = action.broadcasts.iter().find(|b| b.station == station.id) {
            let cost = world
                .programmes
                .iter()
                .find(|p| p.id == broadcast.programme)
                .unwrap()
                .broadcast_cost;
            book.cash
                .insert(station.id, book.cash[&station.id].subtract(cost)?);
            book.cost_sink = book.cost_sink.add(cost)?;
            book.transfer(
                action.at,
                Account::Station(station.id),
                Account::BroadcastCosts,
                cost,
                LedgerReason::BroadcastCost,
            );
        }
    }
    let mut deliveries = Vec::new();
    let mut settlements = Vec::new();
    let ids = book.contracts.keys().copied().collect::<Vec<_>>();
    for id in ids {
        let contract = book.contracts.get_mut(&id).unwrap();
        if !matches!(contract.status, ContractStatus::Reserved) {
            continue;
        }
        if contract.offer.proposal.at == action.at && contract.delivered_at.is_none() {
            let audiences = contract
                .offer
                .demand
                .targets
                .iter()
                .map(|target| {
                    let ad = event
                        .hype
                        .iter()
                        .find(|h| h.brand == contract.offer.brand && h.audience == target.audience)
                        .and_then(|h| {
                            h.advertisements
                                .iter()
                                .find(|a| a.station == contract.offer.station)
                        });
                    let (viewers, effectiveness) = ad
                        .map(|a| (a.viewers, a.effectiveness.trace.output))
                        .unwrap_or((0.0, 0.0));
                    ActualAudience {
                        audience: target.audience,
                        priority: target.priority,
                        viewers,
                        effectiveness,
                        relevant_delivery: viewers * target.priority * effectiveness,
                    }
                })
                .collect::<Vec<_>>();
            let relevant_delivery = audiences.iter().map(|a| a.relevant_delivery).sum();
            contract.actual = ActualDelivery {
                audiences,
                relevant_delivery,
                credited_units: whole_delivery(relevant_delivery)?,
            };
            contract.delivered_at = Some(action.at);
            deliveries.push((id, contract.actual.clone()));
        }
        if contract.offer.deadline <= action.at {
            let promised = contract.offer.promised_delivery;
            let credited = contract.actual.credited_units;
            let payment = Money(
                (u128::from(contract.offer.payment.0) * u128::from(credited.min(promised))
                    / u128::from(promised)) as u64,
            );
            let refund = contract.offer.payment.subtract(payment)?;
            let station = contract.offer.station;
            let brand = contract.offer.brand;
            let settlement = Settlement {
                contract: id,
                at: action.at,
                promised_delivery: promised,
                credited_delivery: credited,
                payment,
                refund,
            };
            contract.status = ContractStatus::Settled(settlement.clone());
            book.cash.insert(station, book.cash[&station].add(payment)?);
            book.budgets
                .insert(brand, book.budgets[&brand].add(refund)?);
            book.transfer(
                action.at,
                Account::Escrow(id),
                Account::Station(station),
                payment,
                LedgerReason::DeliveryPayment,
            );
            book.transfer(
                action.at,
                Account::Escrow(id),
                Account::Brand(brand),
                refund,
                LedgerReason::UnderdeliveryRefund,
            );
            settlements.push(settlement);
        }
    }
    Ok(FinancialEvent {
        ledger: book.ledger[start..].to_vec(),
        deliveries,
        settlements,
    })
}
