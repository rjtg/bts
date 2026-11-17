import init, { WebSession, api_version } from '../pkg/bts_web.js';
import type { Command, Report, Response } from './protocol';
import './style.css';

function element<T extends HTMLElement>(id: string): T {
  const result = document.getElementById(id);
  if (!result) throw new Error(`Missing UI element: ${id}`);
  return result as T;
}
const status = element('status');
const error = element('error');
const slot = element<HTMLSelectElement>('slot');
const programme = element<HTMLSelectElement>('programme');
const advertiser = element<HTMLSelectElement>('advertiser');
let session: WebSession;
let report: Report;
const money = (value: string) => BigInt(value).toLocaleString('en'); // Formatting only; never accounting.
const count = (value: number) => value.toLocaleString('en', { maximumFractionDigits: 2 });
function option(value: string, text: string): HTMLOptionElement {
  const result = document.createElement('option'); result.value = value; result.textContent = text; return result;
}
function paragraph(text: string): HTMLParagraphElement {
  const result = document.createElement('p'); result.textContent = text; return result;
}
function table(caption: string, headers: string[], rows: string[][]): HTMLElement {
  const wrap = document.createElement('div'); wrap.className = 'scroll';
  const result = document.createElement('table');
  result.createCaption().textContent = caption;
  const head = result.createTHead().insertRow();
  for (const text of headers) { const th = document.createElement('th'); th.textContent = text; head.append(th); }
  const body = result.createTBody();
  for (const values of rows) { const row = body.insertRow(); for (const text of values) row.insertCell().textContent = text; }
  wrap.append(result); return wrap;
}
function catalogueInfo(): void {
  const p = report.catalogue.find(p => p.id === Number(programme.value));
  element('programme-info').textContent = p ? `${p.genre} · ${p.kind}${p.episode ? ` ${p.episode}` : ''} · tone ${p.tone} · awareness ${p.awareness} · cast appeal ${p.cast_appeal} · cost ${money(p.broadcast_cost)} credits. Execution is not known.` : '';
}
function selectPlan(): void {
  const plan = report.schedule[Number(slot.value)];
  programme.value = String(plan?.programme ?? report.catalogue[0].id);
  advertiser.value = plan?.proposed_advertiser === null || !plan ? '' : String(plan.proposed_advertiser);
  catalogueInfo();
}
function render(next: Report): void {
  const previousSlot = report?.next_slot;
  report = next;
  element('cash').textContent = money(report.cash);
  element('reporting').textContent = report.reporting;
  element('position').textContent = report.next_slot === null ? 'Seven days complete. Reset to run again.' : `Next: day ${report.schedule[report.next_slot].day}, ${report.schedule[report.next_slot].hour}:00.`;
  slot.replaceChildren(...report.schedule.filter(s => !s.completed).map(s => option(String(s.slot), `Day ${s.day} · ${s.hour}:00${s.locked ? ' · booked' : ''}`)));
  const selectedSlot = previousSlot === report.next_slot ? slot.dataset.selected : null;
  slot.value = selectedSlot && report.schedule[Number(selectedSlot)] && !report.schedule[Number(selectedSlot)].completed ? selectedSlot : String(report.next_slot ?? '');
  programme.replaceChildren(...report.catalogue.map(p => option(String(p.id), p.name)));
  advertiser.replaceChildren(option('', 'No advertisement'), ...report.advertisers.map(a => option(String(a.id), a.name)));
  selectPlan();
  const name = (id: number | null) => report.catalogue.find(p => p.id === id)?.name ?? 'Unplanned';
  const brand = (id: number) => report.advertisers.find(a => a.id === id)?.name ?? String(id);
  const schedule = element<HTMLTableSectionElement>('schedule'); schedule.replaceChildren();
  for (const s of report.schedule) {
    const row = schedule.insertRow();
    for (const text of [`${s.day} / ${s.hour}:00`, name(s.programme), name(s.rival_programme), s.completed ? 'Completed' : s.locked ? 'Booked; locked' : 'Editable']) row.insertCell().textContent = text;
  }
  const offer = element('offer'); offer.replaceChildren();
  if (report.offer) {
    offer.append(paragraph(`${brand(report.offer.advertiser)} offers up to ${money(report.offer.maximum_payment)} credits. Delivery deadline: simulated hour ${report.offer.deadline_hour}. Each audience settles separately; underdelivery refunds the unpaid allocation.`));
    offer.append(table('Frozen terms if accepted', ['Audience', 'Forecast relevant impressions', 'Promised micro-units', 'Allocated credits'], report.offer.targets.map(t => [t.audience, count(t.forecast_relevant_delivery), t.promised_units, money(t.allocated_payment)])));
  } else offer.append(paragraph('No current offer.'));
  element<HTMLButtonElement>('accept').disabled = !report.offer;
  element<HTMLButtonElement>('advance').disabled = report.next_slot === null;
  element<HTMLButtonElement>('quote').disabled = report.next_slot === null || report.schedule[report.next_slot].locked;
  element<HTMLButtonElement>('plan').disabled = report.next_slot === null;
  const contracts = element('contracts'); contracts.replaceChildren();
  for (const c of report.contracts) {
    contracts.append(paragraph(`Contract ${c.contract_id} · ${brand(c.advertiser)} · maximum ${money(c.maximum_payment)} credits · ${c.settlement ? `Settled: paid ${money(c.settlement.payment)}, refunded ${money(c.settlement.refund)} credits.` : 'Reserved in escrow.'}`));
    if (c.settlement) contracts.append(table('Settlement by audience', ['Audience', 'Promised units', 'Credited units', 'Paid credits', 'Refunded credits'], c.settlement.targets.map(t => [t.audience, t.promised_units, t.credited_units, money(t.payment), money(t.refund)])));
  }
  const results = element('results'); results.replaceChildren();
  if (!report.results.length) results.append(paragraph('No broadcasts yet.'));
  for (const r of report.results) {
    results.append(paragraph(`Slot ${r.slot + 1}: ${r.off_air ? 'Off air: insufficient broadcast funds.' : name(r.aired_programme)} · cost ${money(r.broadcast_cost)} · cash after ${money(r.cash_after)} credits.`));
    results.append(table('Provisional exact expected counts', ['Audience', 'Your station', 'Rival', 'Not watching'], r.audiences.map(a => [a.audience, count(a.player_viewers), count(a.rival_viewers), count(a.not_watching)])));
  }
}
function send(command: Command): void {
  try {
    const response: Response = JSON.parse(session.dispatch(JSON.stringify({ api_version: 1, command })));
    if (response.status === 'error') { error.textContent = `${response.error.code}: ${response.error.message}`; error.hidden = false; return; }
    error.hidden = true; render(response.report);
  } catch (failure) { error.textContent = `Browser adapter failure: ${String(failure)}. Reset or reload to retry.`; error.hidden = false; }
}
async function start(): Promise<void> {
  try {
    await init();
    if (api_version() !== 1) throw new Error('Unsupported adapter version');
    session = new WebSession();
    send({ type: 'inspect' });
    programme.addEventListener('change', catalogueInfo);
    slot.addEventListener('change', () => { slot.dataset.selected = slot.value; selectPlan(); });
    element('plan').addEventListener('click', () => send({ type: 'plan', slot: Number(slot.value), programme: Number(programme.value), advertiser: advertiser.value === '' ? null : Number(advertiser.value) }));
    element('quote').addEventListener('click', () => send({ type: 'request_offer' }));
    element('accept').addEventListener('click', () => { if (report.offer) send({ type: 'accept_offer', offer_id: report.offer.offer_id }); });
    element('advance').addEventListener('click', () => send({ type: 'advance' }));
    element('reset').addEventListener('click', () => { delete slot.dataset.selected; send({ type: 'reset' }); });
    element('controls').hidden = false; status.textContent = 'Ready · Rust / WebAssembly · player protocol v1';
  } catch (failure) {
    status.textContent = 'WebAssembly initialization failed.';
    error.textContent = `${String(failure)}. Reload to retry; use the documented build and HTTP development server.`; error.hidden = false;
  }
}
void start();
