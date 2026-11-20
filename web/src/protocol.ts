// Player protocol v8. Rust DTOs are authoritative; no developer-export types are imported.
export type Command =
  | { type: 'place_exact'; programme: number; start_minute: number; ad_minutes: number | null }
  | { type: 'move_exact'; airing_id: number; start_minute: number }
  | { type: 'advance_exact'; to_minute: number }
  | { type: 'set_ad_load'; percent: number }
  | { type: 'place_filler'; start_minute: number; teaser_for: number | null }
  | { type: 'inspect' | 'reset' | 'confirm_weekly' }
  | { type: 'preview_weekly'; plan: WeeklyPlan }
  | { type: 'preview_weekly_edit'; airing_id: number; minute_of_day: number }
  | { type: 'buy_rights' | 'accept_resale'; programme: number; token: string }
  | { type: 'assess_title'; programme: number; insight: number }
  | { type: 'list_resale'; programme: number }
  | { type: 'resize_ad_block'; airing_id: number; minutes: number }
  | { type: 'set_autosell'; airing_id: number; policy: AutosellPolicy | null }
  | { type: 'request_minute_offers'; airing_id: number }
  | { type: 'accept_minute_offer'; airing_id: number; advertiser: number; token: string }
  | { type: 'pitch_film'; title: string; subject: Subject; ambition: number[] }
  | { type: 'write_film'; film_id: number; writer: number; focus: number | null }
  | { type: 'accept_script'; film_id: number }
  | { type: 'review_film' | 'book_film'; film_id: number; booking: FilmBooking }
  | { type: 'set_polling'; polling: Polling }
  | { type: 'spend_insight'; audience: number; amount: number }
  | { type: 'accept_request'; request_id: number }
  | { type: 'create_discussion_run'; brief: ProductionConcept; first_air_minute: number; ad_minutes: number }
  | { type: 'book_discussion_edition'; production_id: number; studio: number }
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
  prior_slot_airing: boolean;
  lead_in_airing_id: number | null;
  autosell_eligible: boolean; autosell: AutosellPolicy | null;
  airing_id: number; station: number; programme: number; programme_name: string; genre:string; duration_minutes:number; start_tick: number; end_tick: number; start_minute: number; end_minute: number;
  filler: boolean; teaser_for: number | null; teaser_current: boolean; ad_minutes: number; sold_minutes: number; weekly_id: number | null; proposed_advertiser: number | null; booked: boolean; status: 'planned' | 'on_air' | 'completed' | 'off_air'; editable: boolean;
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
  airing_id: number; start_tick: number; end_tick: number; start_minute: number; end_minute: number; snapshot_tick: number;
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
    estimate: { tick: number; population: number; typical_age: number; tastes: Record<string, number>; familiarity_level: string; uncertainty: number; watch_time_note: string | null };
  }[];
}
export interface Report {
  brand_news: BrandNews[];
  station_goals: { label: string; progress: number; target: number }[];
  research: Research | null;
  film_desk: FilmDesk;
  rights_market: RightsMarket;
  film_review: FilmReview | null;
  production: ProductionView;
  booking_review: { production_id: number; exceptional_person: number | null; cooperation_sway: number; studio:number; total_sway: number; sway_remaining: number; budget: string; cash_remaining: string } | null;
  reporting: string; now_tick: number; now_minute: number; ad_load_percent: number; end_tick: number; first_weekday: number; weekly_review: WeeklyReview | null; tick_minutes: number; next_airing: number | null; active_airing: number | null; cash: string;
  stations: { id: number; name: string; is_player: boolean; staff:string[]; owned_studios:number|null }[];
  gaps: { start_tick: number; end_tick: number; start_minute: number; end_minute: number }[];
  finances: { opening_cash: string; costs_paid: string; revenue: string; rights_spent: string; resale_receipts: string; production_committed: string; production_in_escrow: string; production_spent: string; ledger: { rights_title: number | null; production_id: number | null; tick: number; airing_id: number | null; kind: string; amount: string }[] };
  catalogue: Programme[]; advertisers: { id: number; name: string }[]; schedule: Airing[];
  offer: { offer_id: string; airing_id: number; start_tick: number; end_tick: number; start_minute: number; end_minute: number; advertiser: number; deadline_tick: number; maximum_payment: string; targets: Commitment[] } | null;
  minute_offers: { airing_id: number; token: string; offers: {
    advertiser: number; minutes: number; price: string; expected_target_viewers: number;
  }[] } | null;
  minute_sales: MinuteBooking[];
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
  discussion_runs: { series: number; title: string; host: number; editions: DiscussionEdition[] }[];
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

export type MinuteBooking = {
  automatic: boolean;
  airing_id: number; advertiser: number; minutes: number; price: string;
  settlement: { aired_minutes: number; payment: string; refund: string; viewer_minutes: number } | null;
};

export type MarketOffer = { token: string; price: string; deadline: number };
export type MarketTitle = {
  programme: number; name: string; status: 'listed' | 'expired' | 'owned'; owner: number | null;
  evidence: string; art: string; duration: number; broadcast_cost: string;
  acquisition: MarketOffer | null; resale: MarketOffer | null; blocker: number | null;
  rival_airing: number | null; assessment_due: number | null; assessment_at: number | null;
  qualities: Quality[]; expected_viewers: number; uncertainty: string; groups: [string, number][];
};
export type RightsMarket = {
  titles: MarketTitle[];
  news: { programme: number; tick: number; kind: string; station: number | null }[];
  assessment_ticks: number;
  spending: { programme: number; tick: number; amount: number; balance: number }[];
};

export type WeeklyPlan = {
  programme: number; weekday: number; minute_of_day: number; count: number; ad_minutes: number;
  exceptions: { occurrence: number; start_minute: number | null }[];
};
export type WeeklyReview = {
  edition: boolean;
  rows: { occurrence: number; airing: number | null; regular_minute: number;
    start_minute: number | null; end_minute: number | null; runtime_minutes: number;
    conflicts: number[]; issue: string | null; protected: boolean }[];
};

export interface DiscussionEdition {
  production_id: number;
  planned_air_minute: number;
  reserved_end_minute: number | null;
  ad_minutes: number;
  airing_id: number | null;
  missed: boolean;
}

export interface AutosellPolicy { floor: string; max_minutes: number }

export interface BrandNews { advertiser: number; tick: number; direction: 'more_competitive' | 'more_cautious' }
