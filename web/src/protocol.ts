// Player protocol v5. Rust DTOs are authoritative; no developer-export types are imported.
export type Command =
  | { type: 'inspect' | 'reset' }
  | { type: 'accept_request'; request_id: number }
  | { type: 'create_production'; concept: ProductionConcept }
  | { type: 'revise_production'; production_id: number; concept: ProductionConcept }
  | { type: 'discard_production'; production_id: number }
  | { type: 'review_production'; production_id: number; exceptional_person: number | null; cooperation_sway: number }
  | { type: 'book_production'; production_id: number; exceptional_person: number | null; cooperation_sway: number }
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
  airing_id: number; station: number; programme: number; programme_name: string; genre:string; duration_minutes:number; start_tick: number; end_tick: number;
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
  booking_review: { production_id: number; exceptional_person: number | null; cooperation_sway: number; studio:number; total_sway: number; sway_remaining: number; budget: string; cash_remaining: string } | null;
  reporting: string; now_tick: number; end_tick: number; tick_minutes: number; next_airing: number | null; active_airing: number | null; cash: string;
  stations: { id: number; name: string; is_player: boolean; staff:string[]; owned_studios:number|null }[];
  gaps: { start_tick: number; end_tick: number }[];
  finances: { opening_cash: string; costs_paid: string; revenue: string; production_committed: string; production_in_escrow: string; production_spent: string; ledger: { production_id: number | null; tick: number; airing_id: number | null; kind: string; amount: string }[] };
  catalogue: Programme[]; advertisers: { id: number; name: string }[]; schedule: Airing[];
  offer: { offer_id: string; airing_id: number; start_tick: number; end_tick: number; advertiser: number; deadline_tick: number; maximum_payment: string; targets: Commitment[] } | null;
  contracts: Booking[]; results: BroadcastResult[];
}
export type Response = { status: 'ok'; api_version: 5; report: Report } | { status: 'error'; api_version: 5; error: { code: string; message: string } };

export type Subject = 'science' | 'everyday_life' | 'entertainment';
export type ProductionTone = 'calm' | 'conversational' | 'lively';
export interface ProductionConcept {
  title: string; host: number; guests: [number, number]; subject: Subject; tone: ProductionTone; budget: string; record_at: number;
}
export interface ProductionInfo extends ProductionConcept {
  id: number; fees: string; recording_end: number; ready_at: number;
  phase: 'draft' | 'booked' | 'recording' | 'post_production' | 'completed';
  programme: number | null; booked_at: number | null; studio:number|null; quest_hints: { request_id: number; matches_requirements: boolean; can_finish_in_time: boolean; achievement_tick: number }[]; exceptional_person: number | null; cooperation_sway: number;
}
export interface ProductionView {
  format: string; duration_minutes: number; recording_ticks: number; post_ticks: number;
  people: { id: number; name: string; role: 'host' | 'guest'; fee: string; description: string; normal_until_tick: number; negotiable_from_tick: number | null; station_id:number|null; commitments: { production_id: number; title: string; start_tick: number; end_tick: number }[] }[];
  projects: ProductionInfo[];
  requests: { id: number; proposer: number; reward: number; achievement: 'recording_completed' | 'production_completed'; statement: string; time_limit_ticks: number; status: 'available' | 'open' | 'completed' | 'expired'; acceptance: { production_id: number | null; accepted_at: number; deadline_tick: number; fulfilled_at: number | null } | null }[];
  sway_balance: number; participation_cost: number; known_concerns: { people: [number, number]; note: string }[];
  sway_history: { tick: number; production_id: number; change: number; balance: number; kind: string; request_id: number | null; person_id: number | null }[];
}
