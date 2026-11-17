// Player protocol v2. Rust DTOs are authoritative; no developer-export types are imported.
export type Command =
  | { type: 'inspect' | 'reset' }
  | { type: 'place'; start_tick: number; programme: number; advertiser: number | null }
  | { type: 'move'; airing_id: number; start_tick: number }
  | { type: 'remove'; airing_id: number }
  | { type: 'set_advertiser'; airing_id: number; advertiser: number | null }
  | { type: 'request_offer'; airing_id: number }
  | { type: 'accept_offer'; airing_id: number; offer_id: string }
  | { type: 'advance'; to_tick: number };
export interface Programme {
  id: number; name: string; genre: string; kind: string; series_name: string | null;
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
  reporting: string; now_tick: number; end_tick: number; tick_minutes: number; next_airing: number | null; active_airing: number | null; cash: string;
  stations: { id: number; name: string; is_player: boolean }[];
  gaps: { start_tick: number; end_tick: number }[];
  finances: { opening_cash: string; costs_paid: string; revenue: string; ledger: { tick: number; airing_id: number | null; kind: string; amount: string }[] };
  catalogue: Programme[]; advertisers: { id: number; name: string }[]; schedule: Airing[];
  offer: { offer_id: string; airing_id: number; start_tick: number; end_tick: number; advertiser: number; deadline_tick: number; maximum_payment: string; targets: Commitment[] } | null;
  contracts: Booking[]; results: BroadcastResult[];
}
export type Response = { status: 'ok'; api_version: 2; report: Report } | { status: 'error'; api_version: 2; error: { code: string; message: string } };
