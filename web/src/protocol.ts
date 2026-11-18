// Player protocol v8. Rust DTOs are authoritative; no developer-export types are imported.
export type Command =
  | { type: 'inspect' | 'reset' }
  | { type: 'pitch_film'; title: string; subject: Subject; ambition: number[] }
  | { type: 'write_film'; film_id: number; writer: number; focus: number | null }
  | { type: 'accept_script'; film_id: number }
  | { type: 'review_film' | 'book_film'; film_id: number; booking: FilmBooking }
  | { type: 'set_polling'; polling: Polling }
  | { type: 'spend_insight'; audience: number; amount: number }
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
  episode: number | null; production_type: 'Fiction' | 'Nonfiction'; qualities: Quality[]; awareness: number; cast_appeal: number;
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
  audiences: { audience: string; population: number; player_viewers: number; rival_viewers: number; not_watching: number; film_reception: string | null }[];
}
export type Polling = 'off' | 'daily' | 'every_three_days';
export interface Research {
  polling: Polling;
  next_tick: number | null;
  missed_poll: boolean;
  insight: number;
  opening_insight: number;
  spending: { tick: number; audience: number; spent: number; remaining: number }[];
  groups: {
    id: number;
    name: string;
    estimate: { tick: number; population: number; typical_age: number; tastes: Record<string, number>; familiarity_level: string; uncertainty: number };
  }[];
}
export interface Report {
  research: Research | null;
  film_desk: FilmDesk;
  film_review: FilmReview | null;
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
export type Quality = { name: string; level: string };
export type Response = { status: 'ok'; api_version: 8; report: Report } | { status: 'error'; api_version: 8; error: { code: string; message: string } };

export type Subject = 'science' | 'everyday_life' | 'entertainment';
export type ProductionTone = 'calm' | 'conversational' | 'lively';
export interface ProductionConcept {
  title: string; host: number; guests: [number, number]; subject: Subject; tone: ProductionTone; budget: string; record_at: number;
}
export interface ProductionInfo extends ProductionConcept {
  ambition: Quality[];
  id: number; fees: string; recording_end: number; ready_at: number;
  phase: 'draft' | 'booked' | 'recording' | 'post_production' | 'completed';
  programme: number | null; booked_at: number | null; studio:number|null; quest_hints: { request_id: number; matches_requirements: boolean; can_finish_in_time: boolean; achievement_tick: number }[]; exceptional_person: number | null; cooperation_sway: number;
}
export interface ProductionView {
  format: string; duration_minutes: number; recording_ticks: number; post_ticks: number;
  people: { id: number; name: string; role: 'host' | 'guest' | 'actor' | 'director' | 'writer' | 'anchor' | 'reporter'; fee: string; description: string; normal_until_tick: number; negotiable_from_tick: number | null; station_id:number|null; commitments: { production_id: number; title: string; start_tick: number; end_tick: number }[] }[];
  projects: ProductionInfo[];
  requests: { id: number; proposer: number; reward: number; achievement: 'recording_completed' | 'production_completed'; statement: string; time_limit_ticks: number; status: 'available' | 'open' | 'completed' | 'expired'; acceptance: { production_id: number | null; accepted_at: number; deadline_tick: number; fulfilled_at: number | null } | null }[];
  sway_balance: number; participation_cost: number; known_concerns: { people: [number, number]; note: string }[];
  sway_history: { tick: number; production_id: number; change: number; balance: number; kind: string; request_id: number | null; person_id: number | null }[];
}

export type FilmBooking = { cast: [number, number]; director: number; studio: number; start: number };
export type FilmReview = FilmBooking & {
  id: number; recording_end: number; ready: number; fees: string; rent: string; total: string; studio_shortfall: boolean;
};
export type FilmPerson = {
  id: number; name: string; job: 'writer' | 'actor' | 'director'; fee: string; revision_fee: string;
  specialty: string; available_until: number; commitments: [string, number, number][];
};
export type FilmCard = {
  id: number; title: string; subject: Subject; phase: 'pitch' | 'writing' | 'script_review' |
    'revising' | 'casting' | 'booked' | 'recording' | 'post' | 'complete';
  committed: string; writer: number | null; due: number | null; revised: boolean;
  script: { qualities: Quality[]; weaknesses: [number, string][]; studio_demand: string } | null;
  booking: FilmReview | null; programme: number | null;
};
export type FilmDesk = {
  projects: FilmCard[]; people: FilmPerson[];
  studios: { id: number; name: string; fee: string; summary: string; commitments: [string, number, number][] }[];
  roles: [string, string]; writing_ticks: number; revision_ticks: number;
};
