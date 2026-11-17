// Browser protocol v1. Rust DTOs in bts-web/src/protocol.rs are authoritative.
// The runtime schema is exercised against native and Wasm responses in the parity test.
export type Command =
  | { type: 'inspect' | 'request_offer' | 'advance' | 'reset' }
  | { type: 'plan'; slot: number; programme: number; advertiser: number | null }
  | { type: 'accept_offer'; offer_id: string };
export interface Programme {
  id: number; name: string; genre: string; kind: string; series_name: string | null;
  episode: number | null; tone: number; awareness: number; cast_appeal: number; broadcast_cost: string;
}
export interface Commitment {
  audience: string; forecast_relevant_delivery: number; promised_units: string; allocated_payment: string;
}
export interface Settlement {
  payment: string; refund: string;
  targets: { audience: string; promised_units: string; credited_units: string; allocated_payment: string; payment: string; refund: string }[];
}
export interface Report {
  reporting: string; next_slot: number | null; cash: string;
  catalogue: Programme[];
  advertisers: { id: number; name: string }[];
  schedule: { slot: number; day: number; hour: number; programme: number | null; proposed_advertiser: number | null; rival_programme: number; locked: boolean; completed: boolean }[];
  offer: { offer_id: string; advertiser: number; slot: number; deadline_hour: number; maximum_payment: string; targets: Commitment[] } | null;
  contracts: { contract_id: string; advertiser: number; deadline_hour: number; maximum_payment: string; targets: Commitment[]; settlement: Settlement | null }[];
  results: { slot: number; at_hour: number; aired_programme: number | null; aired_advertiser: number | null; off_air: boolean; broadcast_cost: string; cash_after: string;
    audiences: { audience: string; population: number; player_viewers: number; rival_viewers: number; not_watching: number }[] }[];
}
export type Response = { status: 'ok'; api_version: 1; report: Report } | { status: 'error'; api_version: 1; error: { code: string; message: string } };
