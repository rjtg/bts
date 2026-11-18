import init, { WebSession, api_version } from '../pkg/bts_web.js';
import type { Airing, Booking, BroadcastResult, Command, Programme, Report, Response } from './protocol';
import { renderAudience } from './audience';
import { initialiseFilmDesk } from './film';
import { initialiseProduction } from './production';
import './style.css';
import { element, select, button, node, option } from './ui/dom';
import { initialisePlayback } from './ui/playback';
import { createResourceHud } from './ui/resources';

const resourceHud=createResourceHud(element('station-resources'));
const status = element('status');
const error = element('error');
let session: WebSession;
let report: Report;
let filmUi: ReturnType<typeof initialiseFilmDesk> | undefined;
let productionUi: ReturnType<typeof initialiseProduction> | undefined;
let selected: number | null = null;
let libraryProgramme = 1;
let viewedDay = 0;
let playback: ReturnType<typeof initialisePlayback> | undefined;
const money = (text: string) => BigInt(text).toLocaleString('en'); // Formatting only.
const count = (value: number) => value.toLocaleString('en', { maximumFractionDigits: 2 });
const pad = (value: number) => String(value).padStart(2, '0');
const time = (tick: number) => `${pad(Math.floor((tick % 96) / 4))}:${pad((tick % 4) * 15)}`;
const stamp = (tick: number) => `Day ${Math.floor(tick / 96) + 1} · ${time(tick)}`;
const windowText = (a: Airing) => `${stamp(a.start_tick)}–${Math.floor(a.start_tick / 96) === Math.floor(a.end_tick / 96) ? time(a.end_tick) : stamp(a.end_tick)}`;
const playerStation = () => report.stations.find(s => s.is_player)!.id;
const programmeFor = (id: number) => report.catalogue.find(p => p.id === id)!;
const publicProgrammeFor = (id:number) => {
  const owned=report.catalogue.find(p=>p.id===id);if(owned)return {name:owned.name,genre:owned.genre,duration_minutes:owned.duration_minutes};
  const scheduled=report.schedule.find(a=>a.programme===id)!;return {name:scheduled.programme_name,genre:scheduled.genre,duration_minutes:scheduled.duration_minutes};
};
const airingFor = (id: number | null) => report.schedule.find(a => a.airing_id === id);
const advertiserName = (id: number) => report.advertisers.find(a => a.id === id)?.name ?? String(id);
function table(caption: string, headers: string[], rows: (string | Node)[][]): HTMLElement {
  const wrap = node('div', '', 'scroll'); const result = node('table');
  result.createCaption().textContent = caption;
  const head = result.createTHead().insertRow();
  headers.forEach(text => head.append(node('th', text)));
  const body = result.createTBody();
  for (const values of rows) { const row = body.insertRow(); values.forEach(text => { const cell = row.insertCell(); if (typeof text === 'string') cell.textContent = text; else cell.append(text); }); }
  wrap.append(result); return wrap;
}
function airingLink(id: number, label: string): HTMLButtonElement {
  const link = node('button', label, 'row-link'); link.addEventListener('click', () => chooseAiring(id, true)); return link;
}
function chooseAiring(id: number, scroll = false): void {
  const airing = airingFor(id); if (!airing) return;
  selected = id;
  if (scroll) { viewedDay = Math.min(6, Math.floor(airing.start_tick / 96)); select('day').value = String(viewedDay); }
  renderTimeline(); renderDetails();
  if (scroll) element('details').scrollIntoView({ behavior: 'smooth', block: 'nearest' });
}
function renderCatalogue(): void {
  const list = element('catalogue'); list.replaceChildren();
  for (const p of report.catalogue.filter(p => select('genre-filter').value === 'all' || p.genre === select('genre-filter').value)) {
    const card = node('button', '', `catalogue-card${libraryProgramme === p.id ? ' chosen' : ''}`);
    card.dataset.programme = String(p.id); card.setAttribute('aria-pressed', String(libraryProgramme === p.id));
    const badge = node('span', p.genre.slice(0, 2).toUpperCase(), `genre-badge ${p.genre.toLowerCase()}`); badge.setAttribute('aria-hidden', 'true');
    const title = node('span'); title.append(node('strong', p.name), node('small', p.production_id === null ? `${p.genre} · ${p.kind}${p.episode ? ` ${p.episode}` : ''}` : `Your original · discussion · #${p.production_id}`));
    const runtime = node('span', `${p.duration_minutes} min`, 'runtime'); runtime.append(node('small', `${money(p.broadcast_cost)} cr`));
    card.append(badge, title, runtime); card.addEventListener('click', () => { libraryProgramme = p.id; renderCatalogue(); }); list.append(card);
  }
  if(!report.catalogue.some(p=>p.id===libraryProgramme))libraryProgramme=report.catalogue[0]?.id??0;
  const chosen = report.catalogue.find(p=>p.id===libraryProgramme);
  element('place-form').hidden=!chosen;
  if(!chosen){element('place-note').textContent='Your library is empty.';return;}
  element('place-name').textContent = chosen.name;
  element('place-note').textContent = `${chosen.duration_minutes} minutes · ${money(chosen.broadcast_cost)} credits per airing. Gaps are off air at no cost.`;
}
function position(start: number, end: number): { left: string; width: string } {
  const dayStart = viewedDay * 96;
  const offset = Math.max(start, dayStart) - dayStart;
  const length = Math.min(end, dayStart + 96) - Math.max(start, dayStart);
  return { left: `calc(${offset} * var(--tick) + 2px)`, width: `calc(${length} * var(--tick) - 4px)` };
}
function renderTimeline(): void {
  button('previous-day').disabled = viewedDay === 0; button('next-day').disabled = viewedDay === 6;
  const timeline = element('timeline'); timeline.replaceChildren();
  const ruler = node('div', '', 'timeline-row ruler'); ruler.append(node('div', '15 MIN / TICK', 'lane-label'));
  const track = node('div', '', 'lane-track');
  for (let hour = 0; hour <= 24; hour++) { const mark = node('span', `${pad(hour)}:00`, 'hour-mark'); mark.style.left = hour === 24 ? 'calc(var(--day-width) - 48px)' : `calc(${hour * 4} * var(--tick))`; track.append(mark); }
  ruler.append(track); timeline.append(ruler);
  const dayStart = viewedDay * 96;
  for (const station of report.stations) {
    const row = node('div', '', 'timeline-row');
    const label = node('div', station.is_player ? 'Your channel' : station.name, 'lane-label'); label.append(node('small', station.is_player ? 'INTERNAL DRAFT' : 'FIXED COMPETITOR')); row.append(label);
    const lane = node('div', '', 'lane-track'); lane.dataset.station = String(station.id);
    if (station.is_player) {
      for (let tick = 0; tick < 96; tick++) {
        const cell = node('button', '', 'timeline-cell'); cell.style.left = `calc(${tick} * var(--tick))`; cell.disabled = dayStart + tick < report.now_tick;
        cell.title = `Place at ${stamp(dayStart + tick)}`; cell.setAttribute('aria-label', `Choose placement ${stamp(dayStart + tick)}`);
        cell.addEventListener('click', () => { select('place-day').value = String(viewedDay); select('place-time').value = String(tick); element('place-form').scrollIntoView({ behavior: 'smooth', block: 'nearest' }); }); lane.append(cell);
      }
      for (const gap of report.gaps.filter(g => g.start_tick < dayStart + 96 && g.end_tick > dayStart)) {
        const marker = node('span', 'OFF AIR / GAP', 'gap-label'); Object.assign(marker.style, position(gap.start_tick, gap.end_tick)); lane.append(marker);
      }
    }
    for (const airing of report.schedule.filter(a => a.station === station.id && a.start_tick < dayStart + 96 && a.end_tick > dayStart)) {
      const p = publicProgrammeFor(airing.programme);
      const block = node('button', '', `airing-block ${airing.status}${!station.is_player ? ' rival' : ''}${airing.booked ? ' booked' : ''}${airing.airing_id === selected ? ' selected' : ''}`);
      block.dataset.airing = String(airing.airing_id); block.dataset.start = String(airing.start_tick); block.dataset.end = String(airing.end_tick);
      Object.assign(block.style, position(airing.start_tick, airing.end_tick));
      block.title = `#${airing.airing_id} · ${p.name} · ${windowText(airing)} · ${airing.status}`;
      block.setAttribute('aria-label', block.title); block.setAttribute('aria-pressed', String(selected === airing.airing_id));
      block.append(node('span', p.name, 'block-title'), node('span', `${time(airing.start_tick)}–${airing.end_tick === dayStart + 96 ? '24:00' : time(airing.end_tick)}`, 'block-time'), node('span', `#${airing.airing_id} · ${airing.booked ? 'BOOKED' : airing.status.replace('_', ' ').toUpperCase()}`, 'block-meta'));
      block.addEventListener('click', () => chooseAiring(airing.airing_id)); lane.append(block);
    }

    row.append(lane); timeline.append(row);
  }
  if (report.now_tick >= dayStart && report.now_tick < dayStart + 96) {
    const line = node('div', '', 'current-time-marker'); line.style.left = `calc(var(--lane-label) + ${report.now_tick-dayStart} * var(--tick))`;
    line.setAttribute('role','img'); line.setAttribute('aria-label', `Current simulated time: ${stamp(report.now_tick)}`);
    line.append(node('span', `NOW ${time(report.now_tick)}`)); timeline.append(line);
  }
  element('day-gaps').textContent = `${report.gaps.filter(g => g.start_tick < dayStart + 96 && g.end_tick > dayStart).length} unfilled intervals on this day`;
}
function contractPanel(booking: Booking): HTMLElement {
  const panel = node('div', '', 'contract-panel');
  panel.append(node('h4', `Contract ${booking.contract_id} · airing #${booking.airing_id}`));
  panel.append(node('p', `${advertiserName(booking.advertiser)} · maximum ${money(booking.maximum_payment)} credits · deadline ${stamp(booking.deadline_tick)}.`));
  panel.append(table('Frozen audience commitments · provisional forecast', ['Audience', 'Forecast relevant impressions', 'Promised micro-units', 'Allocated credits'], booking.targets.map(t => [t.audience, count(t.forecast_relevant_delivery), t.promised_units, money(t.allocated_payment)])));
  if (booking.settlement) {
    const s = booking.settlement; panel.append(node('p', `Settled: paid ${money(s.payment)}, refunded ${money(s.refund)} credits.`));
    panel.append(table('Settlement by audience', ['Audience', 'Credited micro-units', 'Paid credits', 'Refunded credits'], s.targets.map(t => [t.audience, t.credited_units, money(t.payment), money(t.refund)])));
  } else panel.append(node('p', 'Reserved in advertiser escrow. Settlement is due at completion, even if the programme goes off air.'));
  return panel;
}
function resultPanel(result: BroadcastResult): HTMLElement {
  const panel = node('div', '', 'contract-panel'); panel.append(node('h4', `Broadcast result · airing #${result.airing_id}`));
  panel.append(node('p', `${result.off_air ? 'OFF AIR — programme could not be funded. No programme or ad exposure.' : 'Completed.'} Cost ${money(result.broadcast_cost)} · cash after completion ${money(result.cash_after)} credits.`));
  panel.append(table(`Provisional expected counts sampled at ${stamp(result.snapshot_tick)} · not measured ratings`, ['Audience', 'Your channel', 'Rival', 'Not watching'], result.audiences.map(a => [a.audience, count(a.player_viewers), count(a.rival_viewers), count(a.not_watching)])));
  const scheduled = report.schedule.find(airing => airing.airing_id === result.airing_id);
  const film = report.film_desk.projects.find(project => project.programme === scheduled?.programme);
  if (film) {
    const earned = report.contracts.find(booking => booking.airing_id === result.airing_id)?.settlement?.payment ?? '0';
    panel.append(node('p', `Film investment: ${money(film.committed)} credits · This airing’s ad revenue: ${money(earned)} credits.`));
    const reception = result.audiences.filter(row => row.film_reception);
    if (reception.length) panel.append(table('Provisional audience reception', ['Audience', 'Reception'], reception.map(row => [row.audience, row.film_reception!])));
  }
  return panel;
}
function renderDetails(): void {
  const a = airingFor(selected);
  element('selection-empty').hidden = !!a; element('selection-content').hidden = !a;
  if (!a) return;
  const p = publicProgrammeFor(a.programme); const own = a.station === playerStation();
  element('selected-genre').textContent = p.genre.slice(0, 2).toUpperCase();
  element('selected-id').textContent = `AIRING #${a.airing_id} · ${own ? 'YOUR CHANNEL' : 'COMPETITOR'}`;
  element('selected-name').textContent = p.name; element('selected-window').textContent = `${windowText(a)} · ${p.duration_minutes} min`;
  element('selected-status').textContent = `${a.status.replace('_', ' ').toUpperCase()}${a.booked ? ' · BOOKED' : own && a.editable ? ' · INTERNAL DRAFT' : ''}`;
  const owned = own ? programmeFor(a.programme) : null;
  element('known-details').textContent = owned ? `${owned.genre} · ${owned.kind}${owned.episode ? ` ${owned.episode}` : ''} · ${owned.production_type}. ${owned.qualities.map(q => `${q.name} · ${q.level}`).join(", ")}. Cost per airing: ${money(owned.broadcast_cost)} credits. Execution is unknown.` : `${p.genre} · competitor programme. This work is not in your library. Its execution and costs are unknown.`;
  if(owned && owned.production_id!==null)element('known-details').append(document.createTextNode(` Your original interview/discussion, production #${owned.production_id}.`));
  element<HTMLFieldSetElement>('edit-controls').disabled = !a.editable;
  select('move-day').value = String(Math.floor(a.start_tick / 96)); select('move-time').value = String(a.start_tick % 96);
  element('lock-note').textContent = !own ? 'The competitor schedule is read-only.' : a.booked ? 'Booking protected: programme, start, duration and advertiser are locked.' : !a.editable ? 'This airing has already started. Past and on-air broadcasts cannot be edited.' : 'Moving keeps this airing ID. Removal leaves an explicit off-air gap.';
  select('advertiser').replaceChildren(option('', 'No advertisement'), ...report.advertisers.map(b => option(b.id, b.name)));
  select('advertiser').value = a.proposed_advertiser === null ? '' : String(a.proposed_advertiser);
  select('advertiser').disabled = !a.editable; button('set-advertiser').disabled = !a.editable;
  button('quote').disabled = !a.editable || a.proposed_advertiser === null;
  button('quote').textContent = `Request offer for #${a.airing_id}`;
  button('move').textContent = `Move #${a.airing_id}`; button('remove').textContent = `Remove #${a.airing_id}`;
  const offer = element('offer'); offer.replaceChildren();
  if (report.offer?.airing_id === a.airing_id) {
    const q = report.offer;
    offer.className = 'contract-panel'; offer.append(node('h4', `Offer for airing #${q.airing_id} · ${stamp(q.start_tick)}`));
    offer.append(node('p', `${advertiserName(q.advertiser)} offers up to ${money(q.maximum_payment)} credits. Forecast from published audience estimates; each audience settles separately at ${stamp(q.deadline_tick)}.`));
    offer.append(table('Terms if accepted', ['Audience', 'Forecast relevant impressions', 'Promised micro-units', 'Allocated credits'], q.targets.map(t => [t.audience, count(t.forecast_relevant_delivery), t.promised_units, money(t.allocated_payment)])));
    const accept = node('button', `Accept offer for airing #${q.airing_id}`); accept.id = 'accept'; accept.addEventListener('click', () => send({ type: 'accept_offer', airing_id: q.airing_id, offer_id: q.offer_id })); offer.append(accept);
  } else {
    offer.className = '';
    if (report.offer) { const note = node('p', 'A pending offer belongs to another airing: ', 'muted'); note.append(airingLink(report.offer.airing_id, `#${report.offer.airing_id}`)); offer.append(note); }
  }
  const contracts = element('selected-contract'); contracts.replaceChildren();
  const booking = report.contracts.find(c => c.airing_id === a.airing_id); if (booking) contracts.append(contractPanel(booking));
  const result = element('selected-result'); result.replaceChildren();
  const completed = report.results.find(r => r.airing_id === a.airing_id); if (completed) result.append(resultPanel(completed));
  else if (a.status === 'on_air' || a.status === 'off_air') result.append(node('p', 'Results are available at scheduled completion.', 'muted'));
  button('advance-selected').textContent = `Run through selected airing #${a.airing_id} · to ${stamp(a.end_tick)}`;
  button('advance-selected').disabled = !own || a.end_tick <= report.now_tick;
}
function renderReports(): void {
  const results = element('results-list'); results.replaceChildren();
  if (!report.results.length) results.append(node('p', 'No completed broadcasts yet. Results arrive at each airing’s scheduled completion.', 'muted'));
  else results.append(table('Completed airings · select a row for audience counts and contract terms', ['Airing / programme', 'Broadcast window', 'Outcome', 'Cost', 'Cash after'], report.results.map(r => {
    const a = airingFor(r.airing_id)!;
    return [airingLink(r.airing_id, `#${r.airing_id} · ${programmeFor(a.programme).name}`), windowText(a), r.off_air ? 'Off air' : 'Completed', money(r.broadcast_cost), money(r.cash_after)];
  })));
  const summary = element('finance-summary'); summary.replaceChildren();
  for (const [label, value] of [['Opening cash', report.finances.opening_cash], ['Broadcast costs', report.finances.costs_paid], ['Production committed', report.finances.production_committed], ['Ad receipts', report.finances.revenue], ['Current cash', report.cash]]) { const stat = node('div', '', 'finance-stat'); stat.append(node('span', label), node('strong', money(value))); summary.append(stat); }
  summary.append(node('p', `Production budget remaining in escrow: ${money(report.finances.production_in_escrow)} · spent on recording: ${money(report.finances.production_spent)}. Reservation reduces cash once; spending transfers those reserved funds.`, 'production-reserve-note'));
  const ledger = element('ledger'); ledger.replaceChildren();
  if (report.finances.ledger.length) ledger.append(table('Station ledger · exact integer credits', ['Time', 'Airing / production', 'Transaction', 'Credits'], report.finances.ledger.map(l => {
    const kinds: Record<string, string> = { broadcast_cost: 'Broadcast cost', audience_poll: 'Audience poll', advertising_payment: 'Advertising receipt', production_reserve: 'Production budget reserved (cash)', production_spend: 'Recording expense (from escrow)' };
    let link: string | HTMLButtonElement = l.airing_id === null ? '—' : airingLink(l.airing_id, `Airing #${l.airing_id}`);
    if (l.production_id !== null) { const id = l.production_id; link = node('button', `Production #${id}`, 'row-link'); link.addEventListener('click', () => report.film_desk.projects.some(f => f.id === id) ? filmUi?.showProject(id) : productionUi?.showProject(id)); }
    return [stamp(l.tick), link, kinds[l.kind] ?? l.kind, money(l.amount)];
  })));
  else ledger.append(node('p', 'No station transactions yet. Advertiser escrow is separate from your station cash.', 'muted'));
}
function render(next: Report): void {
  report = next;
  if (selected !== null && !airingFor(selected)) selected = null;
  element('clock').textContent = report.now_tick === report.end_tick ? 'Seven days complete' : stamp(report.now_tick);
  resourceHud.render({cash:money(report.cash),sway:String(report.production.sway_balance)});
  if (report.now_tick >= report.end_tick) playback?.pause();
  if (playback?.isPlaying()) { viewedDay = Math.min(6, Math.floor(report.now_tick / 96)); select('day').value = String(viewedDay); }
  playback?.render();
  const nextAiring = airingFor(report.next_airing); const active = airingFor(report.active_airing);
  element('next-airing').textContent = nextAiring ? `#${nextAiring.airing_id} · ${programmeFor(nextAiring.programme).name} · ${stamp(nextAiring.start_tick)}` : 'Nothing else planned on your channel';
  element('on-air').textContent = active ? `Currently ${active.status === 'off_air' ? 'off air' : 'on air'}: #${active.airing_id} · until ${stamp(active.end_tick)}` : 'Your channel is currently off air.';
  button('advance-next').disabled = !active && !nextAiring;
  button('advance-next').textContent = active ? `${active.status === 'off_air' ? 'Advance past off-air' : 'Finish on-air'} #${active.airing_id}` : nextAiring ? `Run next airing #${nextAiring.airing_id}` : 'No next airing';
  for (const id of ['advance-tick', 'advance-day', 'place']) button(id).disabled = report.now_tick >= report.end_tick;
  button('previous-day').disabled = viewedDay === 0; button('next-day').disabled = viewedDay === 6;
  element('reporting').textContent = report.reporting;
  renderAudience(report, stamp, send); renderTimeline(); renderCatalogue(); renderDetails(); renderReports(); productionUi?.render(report); filmUi?.render(report);
  if (playback?.isPlaying()) {
    const scroller = element('timeline-scroller'); const tickWidth = parseFloat(getComputedStyle(document.documentElement).getPropertyValue('--tick'));
    const x = (report.now_tick % 96) * tickWidth;
    if (x < scroller.scrollLeft || x > scroller.scrollLeft + scroller.clientWidth - 180) jump(Math.max(0, report.now_tick % 96 - 4));
  }
}
function send(command: Command, automatic = false): void {
  if (!automatic) playback?.pause();
  try {
    const previousIds = report?.schedule.map(a => a.airing_id) ?? [];
    const response: Response = JSON.parse(session.dispatch(JSON.stringify({ api_version: 8, command })));
    if (response.status === 'error') { playback?.pause(); error.textContent = `${response.error.code}: ${response.error.message}`; error.hidden = false; return; }
    error.hidden = true; productionUi?.commandSucceeded(command);
    if (command.type === 'place') {
      const created = response.report.schedule.find(a => !previousIds.includes(a.airing_id));
      if (created) { selected = created.airing_id; viewedDay = Math.floor(created.start_tick / 96); select('day').value = String(viewedDay); select('place-day').value = String(Math.min(6, Math.floor(created.end_tick / 96))); select('place-time').value = String(created.end_tick % 96); }
    }
    if (command.type === 'reset') { playback?.reset(); libraryProgramme=1; select('genre-filter').value='all'; selected = null; viewedDay = 0; select('day').value = '0'; select('place-day').value = '0'; select('place-time').value = '72'; }
    render(response.report);
  } catch (failure) { playback?.pause(); error.textContent = `Browser adapter failure: ${String(failure)}. Reset or reload to retry.`; error.hidden = false; }
}
function selectedCommand(action: (a: Airing) => Command): void { const a = airingFor(selected); if (a) send(action(a)); }
function jump(tickInDay: number): void {
  const tickWidth = parseFloat(getComputedStyle(document.documentElement).getPropertyValue('--tick'));
  element('timeline-scroller').scrollLeft = tickInDay * tickWidth;
}
function dayChanged(): void { playback?.pause(); viewedDay = Number(select('day').value); button('previous-day').disabled = viewedDay === 0; button('next-day').disabled = viewedDay === 6; renderTimeline(); }
async function start(): Promise<void> {
  try {
    await init(); if (api_version() !== 8) throw new Error('Unsupported player protocol version');
    for (const id of ['day', 'place-day', 'move-day']) select(id).replaceChildren(...Array.from({ length: 7 }, (_, i) => option(i, `Day ${i + 1}`)));
    for (const id of ['place-time', 'move-time']) select(id).replaceChildren(...Array.from({ length: 96 }, (_, i) => option(i, time(i))));
    select('place-time').value = '72';
    const showProgramme = (id: number) => {
      libraryProgramme = id; select('genre-filter').value = 'all';
      const planned = Number(select('place-day').value) * 96 + Number(select('place-time').value);
      if (planned < report.now_tick) { select('place-day').value = String(Math.min(6, Math.floor(report.now_tick / 96))); select('place-time').value = String(report.now_tick % 96); }
      renderCatalogue(); element('library-title').scrollIntoView({ behavior: 'smooth', block: 'center' });
      document.querySelector<HTMLElement>(`[data-programme="${id}"]`)?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    };
    productionUi = initialiseProduction({ report: () => report, send, stamp, money, showProgramme });
    filmUi = initialiseFilmDesk({ send, stamp, money, showProgramme });
    session = new WebSession(); send({ type: 'inspect' });
    playback = initialisePlayback({ state:()=>report, tick:()=>send({type:'advance',to_tick:Math.min(report.end_tick,report.now_tick+1)},true), onPlay:()=>{viewedDay=Math.min(6,Math.floor(report.now_tick/96));select('day').value=String(viewedDay);renderTimeline();jump(Math.max(0,report.now_tick%96-4));} });
    element('place-form').addEventListener('submit', event => { event.preventDefault(); if(!report.catalogue.some(p=>p.id===libraryProgramme))return; send({ type: 'place', programme: libraryProgramme, start_tick: Number(select('place-day').value) * 96 + Number(select('place-time').value), advertiser: null }); });
    element('move').addEventListener('click', () => selectedCommand(a => ({ type: 'move', airing_id: a.airing_id, start_tick: Number(select('move-day').value) * 96 + Number(select('move-time').value) })));
    element('remove').addEventListener('click', () => selectedCommand(a => ({ type: 'remove', airing_id: a.airing_id })));
    element('set-advertiser').addEventListener('click', () => selectedCommand(a => ({ type: 'set_advertiser', airing_id: a.airing_id, advertiser: select('advertiser').value === '' ? null : Number(select('advertiser').value) })));
    element('quote').addEventListener('click', () => selectedCommand(a => ({ type: 'request_offer', airing_id: a.airing_id })));
    element('advance-selected').addEventListener('click', () => selectedCommand(a => ({ type: 'advance', to_tick: a.end_tick })));
    element('advance-tick').addEventListener('click', () => send({ type: 'advance', to_tick: report.now_tick + 1 }));
    element('advance-next').addEventListener('click', () => { const a = airingFor(report.active_airing) ?? airingFor(report.next_airing); if (a) send({ type: 'advance', to_tick: a.end_tick }); });
    element('advance-day').addEventListener('click', () => send({ type: 'advance', to_tick: Math.min(report.end_tick, (Math.floor(report.now_tick / 96) + 1) * 96) }));
    element('reset').addEventListener('click', () => send({ type: 'reset' }));
    element('genre-filter').addEventListener('change', renderCatalogue);
    select('day').addEventListener('change', dayChanged);
    element('previous-day').addEventListener('click', () => { select('day').value = String(viewedDay - 1); dayChanged(); });
    element('next-day').addEventListener('click', () => { select('day').value = String(viewedDay + 1); dayChanged(); });
    element('jump-evening').addEventListener('click', () => jump(64)); element('jump-midnight').addEventListener('click', () => jump(0));
    element('jump-now').addEventListener('click', () => { select('day').value = String(Math.min(6, Math.floor(report.now_tick / 96))); dayChanged(); jump(Math.max(0, report.now_tick % 96 - 4)); });
    document.querySelectorAll<HTMLAnchorElement>('.rail a').forEach(link => link.addEventListener('click', () => { document.querySelectorAll('.rail a').forEach(item => item.classList.toggle('active', item === link)); }));
    element('controls').hidden = false; document.body.classList.add('ready'); status.textContent = 'RUST / WASM READY · PLAYER PROTOCOL 5 · ALL PLANS REMAIN INTERNAL DRAFTS';
    requestAnimationFrame(() => jump(64));
  } catch (failure) { status.textContent = 'WebAssembly initialization failed.'; error.textContent = `${String(failure)}. Reload to retry using the documented HTTP development server.`; error.hidden = false; }
}
void start();
