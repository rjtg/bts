// Player protocol v3. Rust DTOs are authoritative; no developer-export types are imported.
export type Command =
  | { type: 'inspect' | 'reset' }
  | { type: 'create_production'; concept: ProductionConcept }
  | { type: 'revise_production'; production_id: number; concept: ProductionConcept }
  | { type: 'discard_production'; production_id: number }
  | { type: 'book_production'; production_id: number; promise_request: number | null; call_favour: number | null }
  | { type: 'place'; start_tick: number; programme: number; advertiser: number | null }
  | { type: 'move'; airing_id: number; start_tick: number }
  | { type: 'remove'; airing_id: number }
  | { type: 'set_advertiser'; airing_id: number; advertiser: number | null }
  | { type: 'request_offer'; airing_id: number }
  | { type: 'accept_offer'; airing_id: number; offer_id: string }
  | { type: 'advance'; to_tick: number };
export interface Programme {
  id: number; production_id: number | null; name: string; genre: string; kind: string; series_name: string | null;
  episode: number | null; tone: number; awareness: number; cast_appeal: number;
  broadcast_cost: string; duration_minutes: number;
}
export interface Airing {
  airing_id: number; station: number; programme: number; start_tick: number; end_tick: number;
  proposed_advertiser: number | null; booked: boolean; status: 'planned' | 'on_air' | 'completed' | 'off_air'; editable: boolean;
}
export interface Commitment {
  audience: string; forecast_relevant_delivery: number; promised_units: string; allocated_payment: string;
}
export interface Settlement {
  payment: string; refund: string;
  targets: { audience: string; promised_units: string; credited_units: string; allocated_payment: string; payment: string; refund: string }[];
}
export interface Booking {
  airing_id: number; contract_id: string; advertiser: number; deadline_tick: number; maximum_payment: string;
  targets: Commitment[]; settlement: Settlement | null;
}
export interface BroadcastResult {
  airing_id: number; start_tick: number; end_tick: number; snapshot_tick: number;
  aired_programme: number | null; aired_advertiser: number | null; off_air: boolean; broadcast_cost: string; cash_after: string;
  audiences: { audience: string; population: number; player_viewers: number; rival_viewers: number; not_watching: number }[];
}
export interface Report {
  production: ProductionView;
  reporting: string; now_tick: number; end_tick: number; tick_minutes: number; next_airing: number | null; active_airing: number | null; cash: string;
  stations: { id: number; name: string; is_player: boolean }[];
  gaps: { start_tick: number; end_tick: number }[];
  finances: { opening_cash: string; costs_paid: string; revenue: string; production_committed: string; production_in_escrow: string; production_spent: string; ledger: { production_id: number | null; tick: number; airing_id: number | null; kind: string; amount: string }[] };
  catalogue: Programme[]; advertisers: { id: number; name: string }[]; schedule: Airing[];
  offer: { offer_id: string; airing_id: number; start_tick: number; end_tick: number; advertiser: number; deadline_tick: number; maximum_payment: string; targets: Commitment[] } | null;
  contracts: Booking[]; results: BroadcastResult[];
}
export type Response = { status: 'ok'; api_version: 3; report: Report } | { status: 'error'; api_version: 3; error: { code: string; message: string } };

export type Subject = 'science' | 'everyday_life' | 'entertainment';
export type ProductionTone = 'calm' | 'conversational' | 'lively';
export interface ProductionConcept {
  title: string; host: number; guests: [number, number]; subject: Subject; tone: ProductionTone; budget: string; record_at: number;
}
export interface ProductionInfo extends ProductionConcept {
  id: number; fees: string; recording_end: number; ready_at: number;
  phase: 'draft' | 'booked' | 'recording' | 'post_production' | 'completed';
  programme: number | null; booked_at: number | null; promised_request: number | null; called_favour: number | null;
}
export interface ProductionView {
  format: string; duration_minutes: number; recording_ticks: number; post_ticks: number;
  people: { id: number; name: string; role: 'host' | 'guest'; fee: string; description: string; normal_until_tick: number; favour_from_tick: number | null }[];
  projects: ProductionInfo[];
  request: { id: number; person: number; co_guest: number; statement: string; promise: { production_id: number; promised_at: number; fulfilled_at: number | null; earned_favour: number | null } | null } | null;
  favours: { id: number; owed_by: number; earned_from: number; earned_at: number; called_for: number | null; called_at: number | null }[];
}
