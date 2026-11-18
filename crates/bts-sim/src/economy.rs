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
use crate::planning::production::ProductionId;
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
pub struct Demand<T: Clock = SimHour> {
    pub brand: BrandId,
    pub at: T,
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
    pub effectiveness: Option<ModelEvaluation>,
    pub relevant_delivery: f64,
}
/// Quote-time value, separate from promised/actual relevant delivery.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AudienceValuation {
    pub audience: AudienceId,
    pub relevant_delivery: f64,
    pub deficit: f64,
    pub value_weighted_delivery: f64,
    pub uncapped_value: f64,
    pub contractible: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeliveryForecast<T: Clock = SimHour> {
    pub aired: Slot<T>,
    pub off_air: Vec<OffAir>,
    pub audiences: Vec<ForecastAudience>,
    pub relevant_delivery: f64,
}
/// Inspectable quote; acceptance recomputes and compares every field.
#[derive(Clone, Debug, PartialEq)]
pub struct Offer<T: Clock = SimHour> {
    pub brand: BrandId,
    pub station: StationId,
    pub deadline: T,
    pub demand: Demand<T>,
    pub forecast: DeliveryForecast<T>,
    pub value_weighted_delivery: f64,
    pub valuation: Vec<AudienceValuation>,
    pub promised_delivery: u64,
    pub rate_per_thousand: Money,
    pub maximum_offer: Money,
    pub uncapped_price: f64,
    pub payment: Money,
    proposal: Slot<T>,
    revision: u64,
}
impl<T: Clock> Offer<T> {
    pub fn proposal(&self) -> &Slot<T> {
        &self.proposal
    }
}

/// Integer contract delivery resolution; one unit is one millionth of a relevant impression.
pub const DELIVERY_UNITS_PER_IMPRESSION: u64 = 1_000_000;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AudienceCommitment {
    pub audience: AudienceId,
    pub forecast_delivery: f64,
    pub priority: f64,
    pub promised_units: u64,
    pub allocated_payment: Money,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AudienceSettlement {
    pub audience: AudienceId,
    pub promised_units: u64,
    pub credited_units: u64,
    pub allocated_payment: Money,
    pub payment: Money,
    pub refund: Money,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActualAudience {
    pub audience: AudienceId,
    pub priority: f64,
    pub viewers: f64,
    pub effectiveness: f64,
    pub relevant_delivery: f64,
    pub delivered_units: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ActualDelivery {
    pub audiences: Vec<ActualAudience>,
    pub relevant_delivery: f64,
    pub credited_units: u64,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Settlement<T: Clock = SimHour> {
    pub contract: ContractId,
    pub at: T,
    pub promised_delivery: u64,
    pub credited_delivery: u64,
    pub audiences: Vec<AudienceSettlement>,
    pub payment: Money,
    pub refund: Money,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ContractStatus<T: Clock = SimHour> {
    Reserved,
    Settled(Settlement<T>),
}
#[derive(Clone, Debug, PartialEq)]
pub struct Contract<T: Clock = SimHour> {
    pub id: ContractId,
    pub offer: Offer<T>,
    pub commitments: Vec<AudienceCommitment>,
    pub actual: ActualDelivery,
    pub delivered_at: Option<T>,
    pub status: ContractStatus<T>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Account {
    Brand(BrandId),
    Escrow(ContractId),
    Station(StationId),
    BroadcastCosts,
    ResearchCosts,
    ProductionEscrow(ProductionId),
    ProductionCosts,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum LedgerReason {
    Reserve,
    DeliveryPayment,
    UnderdeliveryRefund,
    BroadcastCost,
    ProductionReserve,
    ProductionSpend,
    AudiencePoll,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LedgerEntry<T: Clock = SimHour> {
    pub at: T,
    pub from: Account,
    pub to: Account,
    pub amount: Money,
    pub reason: LedgerReason,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FinancialEvent<T: Clock = SimHour> {
    pub ledger: Vec<LedgerEntry<T>>,
    pub deliveries: Vec<(ContractId, ActualDelivery)>,
    pub settlements: Vec<Settlement<T>>,
}
impl<T: Clock> Default for FinancialEvent<T> {
    fn default() -> Self {
        Self {
            ledger: vec![],
            deliveries: vec![],
            settlements: vec![],
        }
    }
}
/// Internal balances and obligations. No serialization: this is not a save format.
#[derive(Clone, Debug, PartialEq)]
pub struct EconomyState<T: Clock = SimHour> {
    budgets: BTreeMap<BrandId, Money>,
    cash: BTreeMap<StationId, Money>,
    cost_sink: Money,
    research_costs: Money,
    production_escrows: BTreeMap<ProductionId, Money>,
    production_costs: Money,
    contracts: BTreeMap<ContractId, Contract<T>>,
    sold: BTreeMap<(T, StationId), ContractId>,
    ledger: Vec<LedgerEntry<T>>,
    revision: u64,
    next_contract: u64,
}
impl<T: Clock> EconomyState<T> {
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
            research_costs: Money(0),
            production_escrows: BTreeMap::new(),
            production_costs: Money(0),
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
    pub fn production_escrow(&self) -> u128 {
        self.production_escrows
            .values()
            .map(|m| u128::from(m.0))
            .sum()
    }
    pub fn production_costs(&self) -> Money {
        self.production_costs
    }
    pub fn ledger(&self) -> &[LedgerEntry<T>] {
        &self.ledger
    }
    pub fn contracts(&self) -> impl Iterator<Item = &Contract<T>> {
        self.contracts.values()
    }
    pub fn contract(&self, id: ContractId) -> Option<&Contract<T>> {
        self.contracts.get(&id)
    }
    pub fn escrow(&self) -> u128 {
        self.contracts
            .values()
            .filter(|c| matches!(c.status, ContractStatus::Reserved))
            .map(|c| u128::from(c.offer.payment.0))
            .sum()
    }
    /// Available budgets + station cash + ad/production escrow + broadcast/production sinks.
    pub fn total_money(&self) -> u128 {
        self.budgets
            .values()
            .chain(self.cash.values())
            .map(|m| u128::from(m.0))
            .sum::<u128>()
            + self.escrow()
            + u128::from(self.cost_sink.0)
            + u128::from(self.research_costs.0)
            + self.production_escrow()
            + u128::from(self.production_costs.0)
    }
    pub(crate) fn reserve_production(
        &mut self,
        at: T,
        station: StationId,
        id: ProductionId,
        amount: Money,
    ) -> Result<(), Error> {
        amount.validate()?;
        if amount.0 == 0 || self.production_escrows.contains_key(&id) {
            return Err(Error::Invalid(
                "invalid or duplicate production reserve".into(),
            ));
        }
        let cash = self
            .cash
            .get(&station)
            .copied()
            .ok_or_else(|| Error::Invalid("unknown production station".into()))?;
        let mut next = self.clone();
        next.cash.insert(station, cash.subtract(amount)?);
        next.production_escrows.insert(id, amount);
        next.transfer(
            at,
            Account::Station(station),
            Account::ProductionEscrow(id),
            amount,
            LedgerReason::ProductionReserve,
        );
        next.advance_revision()?;
        *self = next;
        Ok(())
    }
    pub(crate) fn spend_production(&mut self, at: T, id: ProductionId) -> Result<(), Error> {
        let amount = self
            .production_escrows
            .get(&id)
            .copied()
            .ok_or_else(|| Error::Invalid("unknown production escrow".into()))?;
        if amount.0 == 0 {
            return Err(Error::Invalid("production budget already spent".into()));
        }
        let mut next = self.clone();
        next.production_escrows.insert(id, Money(0));
        next.production_costs = next.production_costs.add(amount)?;
        next.transfer(
            at,
            Account::ProductionEscrow(id),
            Account::ProductionCosts,
            amount,
            LedgerReason::ProductionSpend,
        );
        next.advance_revision()?;
        *self = next;
        Ok(())
    }
    pub(crate) fn charge_poll(
        &mut self,
        at: T,
        station: StationId,
        price: Money,
    ) -> Result<(), Error> {
        let remaining = self.cash[&station].subtract(price)?;
        let costs = self.research_costs.add(price)?;
        self.cash.insert(station, remaining);
        self.research_costs = costs;
        self.transfer(
            at,
            Account::Station(station),
            Account::ResearchCosts,
            price,
            LedgerReason::AudiencePoll,
        );
        Ok(())
    }
    pub(crate) fn advance_revision(&mut self) -> Result<(), Error> {
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| Error::Invalid("state revision overflow".into()))?;
        Ok(())
    }
    fn transfer(&mut self, at: T, from: Account, to: Account, amount: Money, reason: LedgerReason) {
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
fn micro_delivery(value: f64) -> Result<u64, Error> {
    whole_delivery(value * DELIVERY_UNITS_PER_IMPRESSION as f64)
}

fn commitments<T: Clock>(offer: &Offer<T>) -> Result<Vec<AudienceCommitment>, Error> {
    let mut rows = offer
        .forecast
        .audiences
        .iter()
        .map(|f| {
            Ok(AudienceCommitment {
                audience: f.audience,
                forecast_delivery: f.relevant_delivery,
                priority: f.priority,
                promised_units: micro_delivery(f.relevant_delivery)?,
                allocated_payment: Money(0),
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    rows.sort_by_key(|r| r.audience);
    let weights = rows
        .iter()
        .map(|r| {
            if r.promised_units == 0 {
                0.0
            } else {
                offer
                    .valuation
                    .iter()
                    .find(|v| v.audience == r.audience)
                    .map(|v| v.uncapped_value)
                    .unwrap_or(0.0)
            }
        })
        .collect::<Vec<_>>();
    let eligible = weights
        .iter()
        .enumerate()
        .filter(|(_, w)| **w > 0.0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut suffix = vec![0.0; eligible.len() + 1];
    for j in (0..eligible.len()).rev() {
        suffix[j] = weights[eligible[j]] + suffix[j + 1];
    }
    let mut remaining = offer.payment.0;
    for (j, i) in eligible.iter().copied().enumerate() {
        let amount = if j + 1 == eligible.len() {
            remaining
        } else {
            ((remaining as f64 * (weights[i] / suffix[j]).clamp(0.0, 1.0)).floor() as u64)
                .min(remaining)
        };
        rows[i].allocated_payment = Money(amount);
        remaining -= amount;
    }
    if remaining != 0 {
        return Err(Error::Invalid("no contractible value for payment".into()));
    }
    Ok(rows)
}

impl<T: Clock> Engine<T> {
    /// Current actual hype pressure; finite budget is shown separately.
    pub fn demand(&self, brand: BrandId) -> Result<Demand<T>, Error> {
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
        proposal: &Slot<T>,
        station: StationId,
        brand: BrandId,
        deadline: T,
    ) -> Result<Offer<T>, Error> {
        self.quote_cadence(proposal, station, brand, deadline, None)
    }
    pub(crate) fn quote_cadence(
        &self,
        proposal: &Slot<T>,
        station: StationId,
        brand: BrandId,
        deadline: T,
        cadence: Option<(&BTreeSet<StationId>, usize)>,
    ) -> Result<Offer<T>, Error> {
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
        let (_, event) = match cadence {
            Some((starts, period)) => self.evaluate_cadence(proposal, Some(starts), period)?,
            None => self.evaluate_slot(proposal)?,
        };
        let audiences = demand
            .targets
            .iter()
            .map(|target| {
                let ad = event
                    .hype
                    .iter()
                    .find(|h| h.brand == brand && h.audience == target.audience)
                    .and_then(|h| h.advertisements.iter().find(|a| a.station == station));
                let viewers = ad.map(|a| a.viewers).unwrap_or(0.0);
                let relevant_delivery = ad
                    .map(|a| a.viewers * target.priority * a.effectiveness.trace.output)
                    .unwrap_or(0.0);
                ForecastAudience {
                    audience: target.audience,
                    priority: target.priority,
                    viewers,
                    effectiveness: ad.map(|a| a.effectiveness.clone()),
                    relevant_delivery,
                }
            })
            .collect::<Vec<_>>();
        let relevant_delivery = audiences.iter().map(|a| a.relevant_delivery).sum::<f64>();
        let valuation = audiences
            .iter()
            .zip(&demand.targets)
            .map(|(delivery, target)| {
                let value_weighted_delivery = delivery.relevant_delivery * target.deficit;
                let contractible = micro_delivery(delivery.relevant_delivery)? > 0;
                Ok(AudienceValuation {
                    contractible,
                    audience: target.audience,
                    relevant_delivery: delivery.relevant_delivery,
                    deficit: target.deficit,
                    value_weighted_delivery,
                    uncapped_value: if contractible {
                        advertiser.rate_per_thousand.0 as f64 / 1000.0 * value_weighted_delivery
                    } else {
                        0.0
                    },
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let value_weighted_delivery = valuation
            .iter()
            .map(|v| v.value_weighted_delivery)
            .sum::<f64>();
        let promised_delivery = whole_delivery(relevant_delivery)?;
        let uncapped_price = valuation.iter().map(|v| v.uncapped_value).sum::<f64>();
        bounded(uncapped_price, 0.0, f64::MAX, "offer price arithmetic")?;
        let cap = advertiser.maximum_offer.0.min(demand.available_budget.0);
        let payment = Money(uncapped_price.min(cap as f64).floor() as u64);
        Ok(Offer {
            brand,
            station,
            deadline,
            demand,
            forecast: DeliveryForecast {
                aired: event.aired,
                off_air: event.off_air,
                audiences,
                relevant_delivery,
            },
            value_weighted_delivery,
            valuation,
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
    pub fn accept_offer(&mut self, offer: &Offer<T>) -> Result<ContractId, Error> {
        if &self.quote(&offer.proposal, offer.station, offer.brand, offer.deadline)? != offer {
            return Err(Error::Invalid("stale or modified offer".into()));
        }
        self.reserve_verified_offer(offer)
    }
    pub(crate) fn reserve_verified_offer(&mut self, offer: &Offer<T>) -> Result<ContractId, Error> {
        if offer.payment.0 == 0 {
            return Err(Error::Invalid("offer has no payable delivery".into()));
        }
        let commitments = commitments(offer)?;
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
                commitments,
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
pub(crate) fn validate_inventory<T: Clock>(
    book: &EconomyState<T>,
    action: &Slot<T>,
) -> Result<(), Error> {
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
pub(crate) fn apply_slot<T: Clock>(
    world: &World,
    book: &mut EconomyState<T>,
    event: &SlotEvent<T>,
) -> Result<FinancialEvent<T>, Error> {
    validate_inventory(book, &event.action)?;
    let action = &event.aired;
    let start = book.ledger.len();
    for station in &world.stations {
        if let Some(broadcast) = action
            .broadcasts
            .iter()
            .find(|b| b.station == station.id && event.starting.contains(&b.station))
        {
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
                .commitments
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
                    Ok(ActualAudience {
                        audience: target.audience,
                        priority: target.priority,
                        viewers,
                        effectiveness,
                        relevant_delivery: viewers * target.priority * effectiveness,
                        delivered_units: micro_delivery(viewers * target.priority * effectiveness)?,
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?;
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
            let audience_settlements = contract
                .commitments
                .iter()
                .map(|commitment| {
                    let units = contract
                        .actual
                        .audiences
                        .iter()
                        .find(|a| a.audience == commitment.audience)
                        .map(|a| a.delivered_units)
                        .unwrap_or(0);
                    let paid = if commitment.allocated_payment.0 == 0 {
                        0
                    } else {
                        (u128::from(commitment.allocated_payment.0)
                            * u128::from(units.min(commitment.promised_units))
                            / u128::from(commitment.promised_units)) as u64
                    };
                    AudienceSettlement {
                        audience: commitment.audience,
                        promised_units: commitment.promised_units,
                        credited_units: units,
                        allocated_payment: commitment.allocated_payment,
                        payment: Money(paid),
                        refund: Money(commitment.allocated_payment.0 - paid),
                    }
                })
                .collect::<Vec<_>>();
            let payment = Money(audience_settlements.iter().map(|a| a.payment.0).sum());
            let refund = contract.offer.payment.subtract(payment)?;
            let station = contract.offer.station;
            let brand = contract.offer.brand;
            let settlement = Settlement {
                contract: id,
                at: action.at,
                promised_delivery: promised,
                credited_delivery: credited,
                audiences: audience_settlements,
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
