import init, { WebSession, api_version } from '../pkg/bts_web.js';
import type { Airing, Booking, BroadcastResult, Command, Programme, Report, Response } from './protocol';
import { initialiseWeeklyPlanner, WEEKDAYS } from './weekly-planner';
import { initialisePlannerDrag } from './planner-drag';
import { renderAdMarket, salesFor, minuteRevenue } from './ad-market';
import { initialiseRightsMarket } from './rights-market';
import { renderAudience } from './audience';
import { initialiseFilmDesk, preparing } from './film';
import { initialiseHiring } from './hiring';
import { initialiseNewsroom } from './newsroom';
import { initialiseProduction } from './production';
import './style.css';
import { element, select, button, node, option } from './ui/dom';
import { initialisePlayback } from './ui/playback';
import { assistantNote, surveyEvidence } from './ui/programme-assistant';
import { adMinutes, airStamp, bestOpening, DAY_MINUTES, displayTime, opening, parseTime, turnout } from './ui/airtime';
import { initialiseTapeShelf } from './ui/tape-shelf';
import { initialiseDesks } from './ui/desks';
import { initialiseCalendar } from './ui/planning-calendar';
import { matchesProductionType, programmeChart, programmeCredits } from './ui/programme-paper';
import { dossier, leaf, spread } from './ui/dossier';
import { officePapers } from './ui/office-dossier';
import { initialiseOfficeIdle } from './ui/office-idle';
import { initialiseOfficeMotion } from './ui/office-motion';
import { renderToday } from './ui/today-strip';
import { fundsMark } from './ui/funds';
import { createResourceHud } from './ui/resources';

const resourceHud=createResourceHud(element('station-resources'));
const status = element('status');
const error = element('error');
let session: WebSession;
let report: Report;
let weeklyUi: ReturnType<typeof initialiseWeeklyPlanner> | undefined;
let plannerDrag: ReturnType<typeof initialisePlannerDrag> | undefined;
let rightsUi: ReturnType<typeof initialiseRightsMarket> | undefined;
let filmUi: ReturnType<typeof initialiseFilmDesk> | undefined;
let hiringUi: ReturnType<typeof initialiseHiring> | undefined;
let newsroomUi: ReturnType<typeof initialiseNewsroom> | undefined;
let productionUi: ReturnType<typeof initialiseProduction> | undefined;
let selected: number | null = null;
let libraryProgramme = 1;
let pickedGap: number | null = null;
let plannerAnchor = 18 * 60;
let showingAiring = false;
let officeTapeOpen = false;
let officeTrayOpen = false;
let placementOverride: number | null = null;
const airingForecasts = new Map<number, { start: number; surveyed: number; viewers: number }>();
let activeDesk = 'office';
let calendar: ReturnType<typeof initialiseCalendar>;
const isDesk = () => activeDesk === 'planning' || activeDesk === 'office';
const openSeries = new Set<string>();
let viewedDay = 0;
let tapeShelf: ReturnType<typeof initialiseTapeShelf>;
let desks: ReturnType<typeof initialiseDesks>;
let officeMotion: ReturnType<typeof initialiseOfficeMotion>;
let moveFieldsFor: number | null = null;
let reviewSelection: number | null | undefined;
let playback: ReturnType<typeof initialisePlayback> | undefined;
const money = (text: string) => BigInt(text).toLocaleString('en'); // Formatting only.
const count = (value: number) => value.toLocaleString('en', { maximumFractionDigits: 2 });
const pad = (value: number) => String(value).padStart(2, '0');
const time = (tick: number) => `${pad(Math.floor((tick % 96) / 4))}:${pad((tick % 4) * 15)}`;
const dayName = (day: number) => WEEKDAYS[((report?.first_weekday ?? 5) + day) % 7];
const dayLabel = (day: number) => `${dayName(day)} · Day ${day + 1}`;
const stamp = (tick: number) => `Day ${Math.floor(tick / 96) + 1} · ${time(tick)}`;
const weekdayStamp = (tick: number) => `${dayName(Math.floor(tick / 96))} · ${stamp(tick)}`;
const lastDay = () => Math.ceil(report.end_tick / 96) - 1;
const exactStamp = (minute: number) => `${dayName(Math.floor(minute / DAY_MINUTES))} · ${airStamp(minute)}`;
const windowText = (a: Airing) => `${airStamp(a.start_minute)}–${Math.floor(a.start_minute / DAY_MINUTES) === Math.floor(a.end_minute / DAY_MINUTES)
  ? displayTime(a.end_minute) : airStamp(a.end_minute)}`;
const playerStation = () => report.stations.find(s => s.is_player)!.id;
const airingFor = (id: number | null) => report.schedule.find(a => a.airing_id === id);
const advertiserName = (id: number) => report.advertisers.find(a => a.id === id)?.name ?? String(id);
function table(caption: string, headers: (string | Node)[], rows: (string | Node)[][]): HTMLElement {
  const wrap = node('div', '', 'scroll'); const result = node('table');
  result.createCaption().textContent = caption;
  const head = result.createTHead().insertRow();
  headers.forEach(label => {
    const cell = node('th');
    if (typeof label === 'string') cell.textContent = label;
    else cell.append(label);
    head.append(cell);
  });
  const body = result.createTBody();
  for (const values of rows) { const row = body.insertRow(); values.forEach(text => { const cell = row.insertCell(); if (typeof text === 'string') cell.textContent = text; else cell.append(text); }); }
  wrap.append(result); return wrap;
}
function airingLink(id: number, label: string, target = 'planning'): HTMLButtonElement {
  const link = node('button', label, 'row-link'); link.type = 'button'; link.addEventListener('click', () => { element<HTMLDialogElement>('tape-record').close(); chooseAiring(id, target === 'planning'); if (target !== 'planning') { tapeShelf.close(); desks.navigate(target); } }); return link;
}
function closeDeskFile() {
  const wasAiring = showingAiring;
  if (wasAiring && activeDesk === 'planning') element('timeline-scroller').scrollIntoView({ block: 'nearest' });
  showingAiring = false;
  officeTapeOpen = false;
  if (activeDesk === 'office') officeTrayOpen = false;
  stashAiringFile();
  element('paper-on-desk').replaceChildren();
  syncOfficeFile();
  const focus = wasAiring && activeDesk === 'planning'
    ? document.querySelector<HTMLElement>(`.airing-block[data-airing="${selected}"]`) : cardFor(libraryProgramme);
  if (focus?.getClientRects().length) focus.focus({ preventScroll: true });
  else element(activeDesk === 'office' ? 'pull-tapes' : 'timeline-scroller').focus({ preventScroll: true });
  if (activeDesk === 'office') officeMotion.restore();
}
function syncOfficeFile() {
  const open = activeDesk === 'office' && (showingAiring || officeTapeOpen);
  const office = element('office');
  office.classList.toggle('file-open', open);
  office.classList.toggle('tray-open', officeTrayOpen);
  const occupied = activeDesk === 'office' && (open || officeTrayOpen);
  for (const id of ['office-job-links', 'open-notebook', 'office-crt']) element(id).inert = occupied;
  element('pull-tapes').setAttribute('aria-expanded', String(officeTrayOpen));
  element('put-tapes-away').hidden = !officeTrayOpen;
  element('office-workspace').hidden = activeDesk === 'office' && !open && !officeTrayOpen;
}
const minutesFor = (tape: Programme) => placementOverride ?? adMinutes(tape, report.ad_load_percent);
function chooseAiring(id: number, scroll = false): void {
  const airing = airingFor(id); if (!airing) return;
  selected = id; showingAiring = true;
  if (scroll) { if (activeDesk !== 'office') desks.navigate('planning'); viewedDay = Math.min(lastDay(), Math.floor(airing.start_tick / 96)); select('day').value = String(viewedDay); }
  renderTimeline(); renderCatalogue(); renderDetails();
  if (isDesk()) tapeShelf.close();
  if (activeDesk === 'planning') calendar.dayMode();
  if (scroll && activeDesk === 'planning' && Math.floor(airing.start_minute / DAY_MINUTES) === viewedDay) {
    jump(Math.max(0, airing.start_minute % DAY_MINUTES / 15 - 4));
  }
  if (isDesk()) element('details').closest('.dossier')?.querySelector<HTMLButtonElement>('.dossier-close')?.focus();
}
function played(programme: number) {
  return report.results.filter(result => !result.off_air && result.aired_programme === programme)
    .sort((a, b) => b.end_tick - a.end_tick || b.airing_id - a.airing_id);
}

function airingReceipts(id: number): bigint {
  return minuteRevenue(salesFor(report, id))
    + BigInt(report.contracts.find(contract => contract.airing_id === id)?.settlement?.payment ?? '0');
}

function tapeNote(tape: Programme, start: number, ads: number): string {
  const history = played(tape.id);
  const recentEvenings = history.filter(result => result.start_tick % 96 >= 72).slice(0, 2);
  const coveredCosts = recentEvenings.length === 2
    && recentEvenings.every(result => airingReceipts(result.airing_id) >= BigInt(result.broadcast_cost));
  return assistantNote(tape, report, freeMinutes(start), ads, start / 15, history.length, coveredCosts);
}

function showTapeDetails(tape: Programme, inline = false): void {
  const history = played(tape.id);
  const total = history.reduce((sum, result) => sum + airingReceipts(result.airing_id), 0n);
  const costs = history.reduce((sum, result) => sum + BigInt(result.broadcast_cost), 0n);
  const film = report.film_desk.projects.find(project => project.programme === tape.id);
  const production = report.production.projects.find(project => project.programme === tape.id);
  const investment = film?.committed ?? production?.budget;
  const rights = report.rights_market.titles.find(title => title.programme === tape.id);
  const content = inline ? node('div', '', 'station-paper') : element('tape-record-content');
  if (!inline) element('tape-record-title').textContent = tape.name;
  const programme = node('section', '', 'programme-spread');
  const brief = node('div');
  brief.append(node('h3', 'Programme sheet'), node('p', rights?.evidence ?? `${tape.name} · ${tape.production_type}`),
    node('p', `${tape.duration_minutes} min · ${tape.genre} · ${money(tape.broadcast_cost)} Funds to air`), programmeChart(tape));
  programme.append(brief, programmeCredits(tape, report));
  const audience = node('section');
  const start = placementMinute();
  const ads = adMinutes(tape, report.ad_load_percent);
  audience.append(node('h3', 'Audience estimate'),
    node('p', `Slot under consideration · ${exactStamp(start)}`),
    node('p', tapeNote(tape, start, ads), 'assistant-note'));
  const groups = surveyEvidence(tape, report);
  if (groups.length) {
    const surveys = node('ul');
    for (const group of groups) surveys.append(node('li', `${group.name} · possible fit. ${group.estimate.watch_time_note ?? ''} Survey: ${weekdayStamp(group.estimate.tick)} · about ${Math.round(group.estimate.uncertainty * 100)}% uncertainty scale.`));
    audience.append(surveys);
  } else audience.append(node('p', 'The published surveys give us little evidence about who this tape will suit.'));
  if (rights) audience.append(node('p', `Rights desk assessment · about ${Math.round(rights.expected_viewers / 100) * 100} viewers. ${rights.uncertainty}. General estimate across the title, independent of this slot.`));
  else if (history.length) audience.append(node('p', `Last showing drew about ${Math.round(history[0].audiences.reduce((sum, row) => sum + row.player_viewers, 0) / 100) * 100} expected viewers. See the ledger for the slot and date.`));
  else audience.append(node('p', 'No numerical forecast is on file for this tape.'));
  const ledger = node('section');
  ledger.append(node('h3', 'Airing ledger'));
  const figures = node('div', '', 'review-figures');
  for (const [label, value] of [
    ['Times aired', String(history.length)],
    ['Last aired', history.length ? weekdayStamp(history[0].start_tick) : 'Never'],
    ['Ad receipts to date', `${money(String(total))} Funds`],
    ['Airing costs to date', `${money(String(costs))} Funds`],
    ['Production investment', investment === undefined ? 'Unavailable' : `${money(investment)} Funds`],
  ]) {
    const stat = node('div', '', 'finance-stat');
    stat.append(node('span', label), node('strong', value));
    figures.append(stat);
  }
  ledger.append(figures);
  if (history.length) {
    ledger.append(table('Last five airings · provisional expected viewers', ['Aired', 'Your viewers', 'Ad receipts', 'Airing cost'], history.slice(0, 5).map(result => [
      airingLink(result.airing_id, exactStamp(result.start_minute), 'results'),
      count(result.audiences.reduce((sum, audience) => sum + audience.player_viewers, 0)),
      money(String(airingReceipts(result.airing_id))), money(result.broadcast_cost),
    ])));
  } else ledger.append(node('p', 'This tape has no completed broadcasts on your channel.'));
  const office = inline && activeDesk === 'office';
  if (office) {
    const papers = officePapers(tape, report, history, {
    next: nextGap()?.start, start, stamp: exactStamp, recommendation: minute => tapeNote(tape, minute, ads),
    showResult: result => airingLink(result.airing_id, 'Read broadcast review', 'results'),
      receipts: total, costs, money,
    });
    const container = element('paper-on-desk');
    const selected = container.querySelector('[aria-pressed="true"]')?.textContent;
    container.replaceChildren(dossier('Tape', ['Brief', 'Reception', 'Members', 'Bottom line']
      .map((label, index) => ({ label, content: papers[index] })), { selected, close: closeDeskFile }));
    return;
  }
  const papers = [programme, audience, ledger];
  const tabs = inline ? node('div', '', 'desk-paper-tabs') : element('tape-paper-tabs');
  const labels = ['Programme', 'Audience estimate', 'Airing ledger'];
  const buttons = labels.map((label, index) => {
    const tab = node('button', label);
    tab.type = 'button';
    tab.addEventListener('click', () => switchPaper(index));
    return tab;
  });
  function switchPaper(index: number) {
    content.replaceChildren(papers[index]);
    buttons.forEach((tab, i) => tab.setAttribute('aria-pressed', String(i === index)));
  }
  tabs.replaceChildren(...buttons);
  switchPaper(0);
  if (inline) {
    const desk = element('paper-on-desk');
    const chosen = desk.querySelector('button[aria-pressed="true"]')?.textContent;
    const previousScroll = desk.querySelector('.station-paper')?.scrollTop ?? 0;
    switchPaper(Math.max(0, buttons.findIndex(tab => tab.textContent === chosen)));
    desk.replaceChildren(tabs, content);
    content.scrollTop = previousScroll;
  } else element<HTMLDialogElement>('tape-record').showModal();
}

const dayAnchor = () => viewedDay * DAY_MINUTES + plannerAnchor;
const nextGap = () => opening(report, Math.max(dayAnchor(), report.now_minute), (viewedDay + 1) * DAY_MINUTES);
const placementMinute = () => pickedGap ?? nextGap()?.start ?? dayAnchor();
function freeMinutes(start: number): number {
  const gap = report.gaps.find(g => start >= g.start_minute && start < g.end_minute);
  return gap ? gap.end_minute - start : 0;
}

function renderCatalogue(): void {
  const list = element('catalogue');
  const scroll = list.scrollLeft;
  list.replaceChildren();
  const search = element<HTMLInputElement>('library-search').value.trim().toLocaleLowerCase();
  const start = placementMinute();
  const next = nextGap();
  const fit = element<HTMLInputElement>('library-fit');
  const productionType = select('library-type').value;
  const maxRuntime = Number(select('library-runtime').value) || Infinity;
  const timeToFill = next ? next.end - next.start : 0;
  const room = (tape: Programme) => tape.duration_minutes + adMinutes(tape, report.ad_load_percent) <= timeToFill;
  fit.disabled = !next;
  if (!report.catalogue.some(p => p.id === libraryProgramme)) libraryProgramme = report.catalogue[0]?.id ?? 0;
  const groups = new Map<string, { box: HTMLDetailsElement; tapes: HTMLElement; summary: HTMLElement }>();
  const catalogue = [...report.catalogue].sort((a, b) => (a.series_name ?? a.name).localeCompare(b.series_name ?? b.name)
    || (a.episode ?? 0) - (b.episode ?? 0));
  const firstBySeries = new Map<string, Programme>();
  const multiple = new Set<string>();
  for (const tape of catalogue) {
    if (!tape.series_name) continue;
    if (firstBySeries.has(tape.series_name)) multiple.add(tape.series_name);
    else firstBySeries.set(tape.series_name, tape);
  }
  const estimates = new Map(catalogue.map(tape => [tape.id, next ? turnout(tape, report, next.start) : null]));
  const arranged = fit.checked && next ? [...catalogue].sort((a, b) =>
    (estimates.get(b.id) ?? -1) - (estimates.get(a.id) ?? -1)) : catalogue;
  for (const p of arranged) {
    const ads = adMinutes(p, report.ad_load_percent);
    if ((select('genre-filter').value !== 'all' && p.genre !== select('genre-filter').value)
      || !`${p.name} ${p.series_name ?? ''}`.toLocaleLowerCase().includes(search) || !matchesProductionType(p, productionType) || p.duration_minutes > maxRuntime || (fit.checked && next && !room(p))) continue;
    const card = node('button', '', `catalogue-card${libraryProgramme === p.id ? ' chosen' : ''}`);
    card.classList.toggle('fits-selected-gap', fit.checked && room(p));
    card.title = p.name; card.dataset.programme = String(p.id); card.setAttribute('aria-pressed', String(libraryProgramme === p.id));
    const label = node('span', '', 'cassette-label');
    const episode = p.episode ? `Episode ${p.episode}` : p.genre;
    const boxed = p.series_name !== null && multiple.has(p.series_name);
    const genericEpisode = boxed && /^Demo series \d+$/.test(p.series_name ?? '');
    const runtimeLabel = `${p.duration_minutes} min`;
    label.append(node('strong', genericEpisode ? episode : p.name),
      node('small', genericEpisode ? runtimeLabel : `${episode} · ${runtimeLabel}`));
    const spools = node('span', '', 'cassette-window'); spools.setAttribute('aria-hidden', 'true');
    const history = played(p.id);
    const last = history[0];
    const runtime = node('small', last
      ? `${history.length} aired · Last: ${stamp(last.start_tick)}` : 'Unaired', 'cassette-history');
    if (fit.checked && next && estimates.get(p.id) == null) label.append(node('small', 'Unranked', 'shelf-warning'));
    plannerDrag?.bindSource(card, () => ({ programme: p.id, airing: null, minutes: p.duration_minutes + (p.id === libraryProgramme ? minutesFor(p) : ads), adMinutes: p.id === libraryProgramme ? placementOverride : null }));
    card.setAttribute('aria-label', `${p.name} · ${episode} · ${p.duration_minutes} min · ${runtime.textContent}`);
    card.append(label, spools, runtime);
    card.addEventListener('click', () => { if (libraryProgramme !== p.id) placementOverride = null; libraryProgramme = p.id; officeTapeOpen = true; if (activeDesk === 'office') officeTrayOpen = true; showingAiring = false; renderCatalogue(); cardFor(p.id)?.focus(); previewSlot(); });
    if (!p.series_name || !multiple.has(p.series_name)) { list.append(card); continue; }
    const series = p.series_name;
    let group = groups.get(series);
    if (!group) {
      const box = node('details', '', 'series-box');
      const summary = node('summary');
      const tapes = node('div', '', 'series-tapes');
      box.open = Boolean(search) || openSeries.has(series);
      summary.addEventListener('click', event => {
        event.preventDefault();
        box.open = !box.open;
        if (box.open) openSeries.add(series); else openSeries.delete(series);
      });
      box.append(summary, tapes);
      list.append(box);
      group = { box, tapes, summary };
      groups.set(series, group);
    }
    card.dataset.episode = String(p.episode ?? 0);
    const following = Array.from(group.tapes.querySelectorAll<HTMLElement>('[data-episode]'))
      .find(other => Number(other.dataset.episode) > (p.episode ?? 0));
    group.tapes.insertBefore(card, following ?? null);
    const seriesTitle = /^Demo series \d+$/.test(series) ? firstBySeries.get(series)!.name.replace(/:\s*episode\s+\d+$/i, '') : series;
    group.summary.textContent = `${seriesTitle} · ${group.tapes.childElementCount} ${group.tapes.childElementCount === 1 ? 'episode' : 'episodes'}`;
  }
  list.scrollLeft = scroll;
  if (!list.childElementCount) list.append(node('p', 'No titles match these filters.', 'muted'));
  const chosen = report.catalogue.find(p=>p.id===libraryProgramme);
  element('place-form').hidden=!chosen;
  if(!chosen){element('place-note').textContent='Your library is empty.';return;}
  element('place-name').textContent = chosen.name;
  element('desk-runtime').textContent = `${chosen.episode ? `Episode ${chosen.episode} · ` : ''}${chosen.duration_minutes} min`;
  const presentation = element('desk-tape');
  if (presentation.dataset.tape !== String(chosen.id)) {
    presentation.dataset.tape = String(chosen.id);
    presentation.classList.remove('tape-pulled');
    requestAnimationFrame(() => presentation.classList.add('tape-pulled'));
  }
  const ads = minutesFor(chosen);
  const horizon = Math.max(dayAnchor(), report.now_minute);
  const best = bestOpening(chosen, report, horizon);
  const target = pickedGap ?? next?.start;
  const action = button('place');
  action.textContent = target === undefined ? 'No gap today' : `Place at ${displayTime(target)}`;
  action.disabled = target === undefined || target < report.now_minute || freeMinutes(target) < chosen.duration_minutes + ads;
  const bestAction = button('place-best');
  bestAction.textContent = best ? `Best slot · ${exactStamp(best.start)}` : 'Best slot unavailable';
  bestAction.disabled = best === null;
  bestAction.onclick = () => { if (best) placeTape(best.start); };
  element('regular-slot').hidden = !chosen.series_name;
  const ratio = element<HTMLInputElement>('ad-load');
  if (document.activeElement !== ratio) ratio.value = String(report.ad_load_percent);
  element('ad-load-label').textContent = `${ratio.value}% of programme runtime`;
  element('library-description').textContent = tapeNote(chosen, start, ads);
  button('open-tape-record').onclick = () => showTapeDetails(chosen);
  const minutes = element<HTMLInputElement>('placement-ad-minutes');
  if (document.activeElement !== minutes) minutes.value = String(ads);
  element('placement-runtime').textContent = `${chosen.duration_minutes + ads} min occupied`;
  element('place-note').textContent = `${chosen.duration_minutes} min programme + ${ads} min ads · ${chosen.duration_minutes + ads} min airtime · ${money(chosen.broadcast_cost)} Funds to air`;
  renderNotes();
  if (!showingAiring) {
    stashAiringFile();
    if (activeDesk !== 'office' || officeTapeOpen) showTapeDetails(chosen, true);
    else element('paper-on-desk').replaceChildren();
  }
  syncOfficeFile();
}
const cardFor = (id: number) => document.querySelector<HTMLElement>(`[data-programme="${id}"]`);
function placeTape(start: number) {
  send({ type: 'place_exact', programme: libraryProgramme, start_minute: start, ad_minutes: placementOverride });
}

function pickGap(start: number, suggest = false) {
  pickedGap = start;
  plannerAnchor = start % DAY_MINUTES;
  showingAiring = false;
  if (activeDesk === 'office') officeTrayOpen = true;
  if (suggest) element<HTMLInputElement>('library-fit').checked = true;
  renderTimeline(); renderCatalogue(); previewSlot();
  tapeShelf.open();
}

function renderNotes() {
  const notes = element('desk-notes');
  notes.replaceChildren();
  const gap = nextGap();
  if (gap) {
    const length = gap.end - gap.start;
    const message = node('button', `Next gap · ${displayTime(gap.start)} · ${length} min`, 'sticky-note');
    message.type = 'button';
    message.addEventListener('click', () => pickGap(gap.start, true));
    notes.append(message);
    if (length <= 5) {
      const ident = node('button', 'Air a 1-minute ident', 'sticky-note');
      ident.type = 'button';
      ident.addEventListener('click', () => send({ type: 'place_filler', start_minute: gap.start, teaser_for: null }));
      notes.append(ident);
      const after = report.schedule.find(a => a.station === playerStation() && !a.filler && a.start_minute >= gap.end);
      if (after) {
        const teaser = node('button', `Tease ${after.programme_name}`, 'sticky-note');
        teaser.type = 'button';
        teaser.addEventListener('click', () => send({ type: 'place_filler', start_minute: gap.start, teaser_for: after.airing_id }));
        notes.append(teaser);
      }
    }
  }
  const unsold = report.schedule.find(a => a.station === playerStation() && a.status === 'planned'
    && Math.floor(a.start_minute / DAY_MINUTES) === viewedDay && a.ad_minutes > a.sold_minutes);
  if (unsold) notes.append(airingLink(unsold.airing_id,
    `${displayTime(unsold.start_minute)} · ${unsold.ad_minutes - unsold.sold_minutes} ad min unsold`));
  const stale = report.schedule.find(a => a.status === 'planned' && a.filler && a.teaser_for !== null && !a.teaser_current);
  if (stale) notes.append(airingLink(stale.airing_id, `${displayTime(stale.start_minute)} · Teaser needs review`));
}

function previewSlot(): void {
  const work = report.catalogue.find(p => p.id === libraryProgramme);
  if (!work || pickedGap === null) return;
  const start = pickedGap;
  const ads = minutesFor(work);
  const preview = plannerDrag?.inspect({ programme: work.id, airing: null, minutes: work.duration_minutes + ads }, start);
  document.querySelector('.shelf-slot-preview')?.remove();
  if (!preview || Math.floor(start / DAY_MINUTES) !== viewedDay) return;
  const lane = document.querySelector<HTMLElement>(`[data-station="${playerStation()}"]`);
  const marker = node('span', '', `drag-ghost shelf-slot-preview${preview.issue ? ' conflict' : ''}`);
  const available = freeMinutes(start);
  marker.append(node('strong', preview.issue ? 'Doesn’t fit' : work.name),
    node('small', available ? `${displayTime(start)} · ${available} MIN GAP` : displayTime(start)));
  Object.assign(marker.style, position(start, preview.end));
  marker.setAttribute('aria-hidden', 'true');
  lane?.append(marker);
}
function position(start: number, end: number): { left: string; width: string } {
  const dayStart = viewedDay * DAY_MINUTES;
  const offset = Math.max(start, dayStart) - dayStart;
  const length = Math.min(end, dayStart + DAY_MINUTES) - Math.max(start, dayStart);
  return { left: `calc(${offset / 15} * var(--tick))`, width: `max(1px, calc(${length / 15} * var(--tick)))` };
}
function renderTimeline(): void {
  button('previous-day').disabled = viewedDay === 0; button('next-day').disabled = viewedDay === lastDay();
  const timeline = element('timeline'); timeline.replaceChildren();
  const ruler = node('div', '', 'timeline-row ruler'); ruler.append(node('div', '15 MIN / TICK', 'lane-label'));
  const track = node('div', '', 'lane-track');
  for (let hour = 0; hour <= 24; hour++) { const mark = node('span', `${pad(hour)}:00`, 'hour-mark'); mark.style.left = hour === 24 ? 'calc(var(--day-width) - 48px)' : `calc(${hour * 4} * var(--tick))`; track.append(mark); }
  ruler.append(track); timeline.append(ruler);
  const dayStart = viewedDay * DAY_MINUTES;
  for (const station of report.stations) {
    const row = node('div', '', 'timeline-row');
    const label = node('div', station.is_player ? 'Your channel' : station.name, 'lane-label'); label.append(node('small', station.is_player ? 'INTERNAL DRAFT' : 'FIXED COMPETITOR')); row.append(label);
    const lane = node('div', '', 'lane-track'); lane.dataset.station = String(station.id);
    if (station.is_player) {
      plannerDrag?.bindLane(lane, dayStart);
      for (let tick = 0; tick < 96; tick++) {
        const minute = dayStart + tick * 15;
        const cell = node('button', '', 'timeline-cell');
        cell.dataset.tick = String(minute / 15);
        Object.assign(cell.style, position(minute, minute + 15));
        cell.disabled = minute + 15 <= report.now_minute;
        cell.setAttribute('aria-pressed', String(minute === pickedGap));
        cell.title = `Plan at ${airStamp(minute)}`;
        cell.setAttribute('aria-label', `Choose placement ${airStamp(minute)}`);
        cell.addEventListener('click', () => pickGap(Math.max(report.now_minute, minute)));
        lane.append(cell);
      }
      function reserveBlock(title: string, start: number, end: number | null, open: () => void) {
        if (end === null || end <= dayStart || start >= dayStart + DAY_MINUTES) return;
        const block = node('button', `${title} · In production`, 'airing-block booked');
        Object.assign(block.style, position(start, end));
        block.title = `${title} · ${airStamp(start)} · Airtime held for production`;
        block.addEventListener('click', () => { desks.navigate('production'); open(); });
        lane.append(block);
      }
      for (const run of report.production.discussion_runs) {
        for (const edition of run.editions) reserveBlock(run.title, edition.planned_air_minute, edition.reserved_end_minute,
          () => productionUi?.showProject(edition.production_id));
      }
      if (report.newsroom) {
        for (const [index, edition] of report.newsroom.editions.entries()) {
          reserveBlock(report.newsroom.title, edition.air_minute, edition.reserved_end_minute, () => {
            element<HTMLDetailsElement>('newsroom-board').open = true;
            element(`news-edition-${index + 1}`).scrollIntoView({ block: 'center' });
          });
        }
      }
      for (const gap of report.gaps.filter(g => g.start_minute < dayStart + DAY_MINUTES && g.end_minute > dayStart)) {
        const marker = node('span', 'OFF AIR / GAP', 'gap-label');
        Object.assign(marker.style, position(gap.start_minute, gap.end_minute));
        lane.append(marker);
      }
    }
    for (const airing of report.schedule.filter(a => a.station === station.id && a.start_minute < dayStart + DAY_MINUTES && a.end_minute > dayStart)) {
      const p = { name: airing.programme_name, duration_minutes: airing.duration_minutes };
      const block = node('button', '', `airing-block ${airing.status}${!station.is_player ? ' rival' : ''}${airing.booked ? ' booked' : ''}${airing.airing_id === selected ? ' selected' : ''}`);
      block.dataset.airing = String(airing.airing_id); block.dataset.start = String(airing.start_tick); block.dataset.end = String(airing.end_tick);
      Object.assign(block.style, position(airing.start_minute, airing.end_minute));
      block.dataset.startMinute = String(airing.start_minute);
      block.dataset.endMinute = String(airing.end_minute);
      const runtime = p.duration_minutes + airing.ad_minutes;
      block.title = `#${airing.airing_id} · ${p.name} · ${windowText(airing)} · ${airing.status} · ${runtime} min airtime`;
      block.setAttribute('aria-label', block.title); block.setAttribute('aria-pressed', String(selected === airing.airing_id));
      block.append(node('span', p.name, 'block-title'), node('span', `${displayTime(airing.start_minute)}–${airing.end_minute === dayStart + DAY_MINUTES ? '24:00' : displayTime(airing.end_minute)}`, 'block-time'), node('span', `#${airing.airing_id} · ${airing.booked ? 'BOOKED' : airing.status.replace('_', ' ').toUpperCase()}`, 'block-meta'));
      if (airing.editable) plannerDrag?.bindSource(block, () => ({ programme: airing.programme, airing: airing.airing_id, minutes: p.duration_minutes + airing.ad_minutes }));
      block.addEventListener('click', () => chooseAiring(airing.airing_id)); lane.append(block);
    }

    row.append(lane); timeline.append(row);
  }
  if (report.now_minute >= dayStart && report.now_minute < dayStart + DAY_MINUTES) {
    const line = node('div', '', 'current-time-marker'); line.style.left = `calc(var(--lane-label) + ${(report.now_minute - dayStart) / 15} * var(--tick))`;
    line.setAttribute('role','img'); line.setAttribute('aria-label', `Current simulated time: ${airStamp(report.now_minute)}`);
    line.append(node('span', `NOW ${displayTime(report.now_minute)}`)); timeline.append(line);
  }
  const agenda = element('day-agenda');
  const entries = report.schedule.filter(a => a.start_minute < dayStart + DAY_MINUTES && a.end_minute > dayStart)
    .sort((a, b) => a.start_tick - b.start_tick || a.station - b.station);
  agenda.replaceChildren(node('h3', dayLabel(viewedDay)), node('p', 'Tap an airing to open its details.'));
  for (const a of entries) {
    const station = a.station === playerStation() ? 'Your channel' : 'Rival';
    const label = `${displayTime(a.start_minute)} · ${station} · ${a.programme_name}${a.airing_id === selected ? ' · Selected' : ''}`;
    agenda.append(airingLink(a.airing_id, label));
  }
  if (!entries.length) agenda.append(node('p', 'No airings on this day.'));
  const gridLink = node('a', 'Open parallel timeline', 'quiet agenda-grid-link');
  gridLink.href = '#timeline-scroller';
  agenda.append(gridLink);
  element('day-gaps').textContent = `${report.gaps.filter(g => g.start_minute < dayStart + DAY_MINUTES && g.end_minute > dayStart).length} unfilled intervals on this day`;
}
function contractPanel(booking: Booking): HTMLElement {
  const panel = node('div', '', 'contract-panel');
  panel.append(node('h4', `Contract ${booking.contract_id} · airing #${booking.airing_id}`));
  panel.append(node('p', `${advertiserName(booking.advertiser)} · maximum ${money(booking.maximum_payment)} Funds · deadline ${stamp(booking.deadline_tick)}.`));
  panel.append(table('Frozen audience commitments · provisional forecast', ['Audience', 'Forecast relevant impressions', 'Promised micro-units', 'Allocated Funds'], booking.targets.map(t => [t.audience, count(t.forecast_relevant_delivery), t.promised_units, money(t.allocated_payment)])));
  if (booking.settlement) {
    const s = booking.settlement; panel.append(node('p', `Settled: paid ${money(s.payment)}, refunded ${money(s.refund)} Funds.`));
    panel.append(table('Settlement by audience', ['Audience', 'Credited micro-units', 'Paid Funds', 'Refunded Funds'], s.targets.map(t => [t.audience, t.credited_units, money(t.payment), money(t.refund)])));
  } else panel.append(node('p', 'Reserved in advertiser escrow. Settlement is due at completion, even if the programme goes off air.'));
  return panel;
}
function resultPanel(result: BroadcastResult): HTMLElement {
  const panel = node('div', '', 'contract-panel'); panel.append(node('h4', `Broadcast result · airing #${result.airing_id}`));
  panel.append(node('p', `${result.off_air ? 'OFF AIR — programme could not be funded. No programme or ad exposure.' : 'Completed.'} Cost ${money(result.broadcast_cost)} · cash after completion ${money(result.cash_after)} Funds.`));
  panel.append(table(`Provisional expected counts sampled at ${airStamp(result.start_minute)} · not measured ratings`, ['Audience', 'Your channel', 'Rival', 'Not watching'], result.audiences.map(a => [a.audience, count(a.player_viewers), count(a.rival_viewers), count(a.not_watching)])));
  const minuteSales = salesFor(report, result.airing_id);
  const legacy = report.contracts.find(booking => booking.airing_id === result.airing_id);
  const revenue = airingReceipts(result.airing_id);
  const contribution = revenue - BigInt(result.broadcast_cost);
  const figures = node('div', '', 'review-figures');
  for (const [label, value] of [['Ad receipts', revenue], ['Airing cost', BigInt(result.broadcast_cost)], ['Airing margin', contribution]] as const) {
    const stat = node('div', '', 'finance-stat');
    stat.append(node('span', label), node('strong', `${money(String(value))} Funds`));
    figures.append(stat);
  }
  panel.prepend(figures);
  if (minuteSales.length) {
    const sold = minuteSales.reduce((total, sale) => total + sale.minutes, 0);
    const aired = minuteSales.reduce((total, sale) => total + (sale.settlement?.aired_minutes ?? 0), 0);
    panel.append(node('p', `Ad minutes · ${sold} sold · ${aired} aired · ${money(String(minuteRevenue(minuteSales)))} Funds paid.`));
    panel.append(table('Minute sale settlement', ['Advertiser', 'Minutes aired', 'Paid', 'Returned'], minuteSales.map(sale => [
      advertiserName(sale.advertiser) + (sale.automatic ? ' · Autosell' : ''), String(sale.settlement?.aired_minutes ?? 0), money(sale.settlement?.payment ?? '0'), money(sale.settlement?.refund ?? '0'),
    ])));
  }
  const scheduled = report.schedule.find(airing => airing.airing_id === result.airing_id);
  const leadIn = airingFor(scheduled?.lead_in_airing_id ?? null);
  if (leadIn) {
    const note = node('p', 'Recent lead-in · ', 'small');
    note.append(airingLink(leadIn.airing_id, leadIn.programme_name, 'results'));
    panel.append(note);
  }
  if (scheduled?.prior_slot_airing) panel.append(node('p', 'This run has aired in this slot before.', 'small'));
  const film = report.film_desk.projects.find(project => project.programme === scheduled?.programme);
  const discussion = report.production.projects.find(project => project.programme === scheduled?.programme);
  const bulletin = report.newsroom?.editions.find(edition => edition.programme === scheduled?.programme);
  if (bulletin) panel.append(node('p', `News production · ${money(bulletin.cost)} Funds.`));
  if (legacy) panel.append(contractPanel(legacy));
  if (discussion) panel.append(node('p', `Production investment: ${money(discussion.budget)} Funds.`));
  if (film) {
    const earned = String(revenue);
    panel.append(node('p', `Film investment: ${money(film.committed)} Funds · This airing’s ad revenue: ${money(earned)} Funds.`));
    const reception = result.audiences.filter(row => row.film_reception);
    if (reception.length) panel.append(table('Provisional audience reception', ['Audience', 'Reception'], reception.map(row => [row.audience, row.film_reception!])));
  }
  return panel;
}
function restoreAdPaper() {
  element('ad-content-home').after(element('ad-content'));
}
function stashAiringFile() {
  restoreAdPaper();
  element('airing-file-home').append(element('details'), element('airing-paper-content'));
  element('planner-airing-mount').replaceChildren();
}
function displayAiringFile(airing: Airing, result: BroadcastResult | undefined) {
  if (!isDesk() || !showingAiring) return;
  const file = element<HTMLDetailsElement>('details');
  const papers = element(activeDesk === 'planning' ? 'planner-airing-mount' : 'paper-on-desk');
  const sheet = element('airing-paper-content');
  if (!papers.contains(file)) {
    const left = leaf();
    const right = leaf();
    left.append(file);
    right.append(sheet);
    const other = element(papers.id === 'planner-airing-mount' ? 'paper-on-desk' : 'planner-airing-mount');
    if (other.querySelector('[data-file="airing"]')) other.replaceChildren();
    papers.replaceChildren(dossier('Airing', [{ label: 'Airing', content: spread(left, right) }],
      { close: closeDeskFile }));
  }
  file.open = true;
  element('details-title').textContent = result ? 'Airing ledger' : 'Airtime and booking';
  const owned = airing.station === playerStation();
  const tape = owned ? report.catalogue.find(work => work.id === airing.programme) : undefined;
  const estimates = report.research?.groups ?? [];
  if (tape && estimates.length && airing.status === 'planned' && !result && airing.start_minute >= report.now_minute) {
    const viewers = turnout(tape, report, airing.start_minute);
    if (viewers !== null) airingForecasts.set(airing.airing_id,
      { start: airing.start_minute, surveyed: Math.max(...estimates.map(group => group.estimate.tick)), viewers });
  }
  const estimate = airingForecasts.get(airing.airing_id);
  const forecast = node('div', '', 'airing-forecast');
  if (estimate?.start === airing.start_minute) forecast.append(
    node('h3', result ? 'Pre-air outlook' : 'Audience outlook'),
    node('p', `About ${Math.round(estimate.viewers / 100) * 100} potential viewers · survey ${weekdayStamp(estimate.surveyed)}`),
    node('p', 'Survey screening estimate. Competition and repeat viewing can change turnout.', 'paper-footnote'));
  const leadIn = airingFor(airing.lead_in_airing_id);
  if (leadIn && !result) {
    const note = node('p', 'Possible lead-in · ', 'paper-footnote');
    note.append(airingLink(leadIn.airing_id, leadIn.programme_name));
    forecast.append(note);
  }
  if (result) {
    restoreAdPaper();
    sheet.replaceChildren(forecast, resultPanel(result));
  } else if (owned && !airing.filler) {
    sheet.replaceChildren(forecast, element('ad-content'));
  } else {
    restoreAdPaper();
    sheet.replaceChildren(forecast);
  }
  syncOfficeFile();
}
function renderDetails(): void {
  const a = airingFor(selected);
  const completed = report.results.find(r => r.airing_id === selected);
  const result = element('selected-result'); result.replaceChildren();
  const index = element<HTMLDetailsElement>('results-index');
  const reviewed = completed?.airing_id ?? null;
  if (reviewSelection !== reviewed) index.open = !completed;
  reviewSelection = reviewed;
  const reviewAiring = button('review-airing'), future = button('review-future');
  reviewAiring.hidden = !completed; future.hidden = true;
  if (completed && a) {
    result.append(node('h3', a.programme_name), node('p', exactStamp(a.start_minute)), resultPanel(completed));
    const next = report.schedule.filter(item => item.station === a.station && item.programme === a.programme
      && item.status === 'planned' && item.start_tick > a.start_tick)
      .sort((left, right) => left.start_tick - right.start_tick)[0];
    reviewAiring.onclick = () => chooseAiring(a.airing_id, true);
    if (next) {
      future.hidden = false; future.textContent = `Next showing · ${exactStamp(next.start_minute)}`;
      future.onclick = () => chooseAiring(next.airing_id, true);
    }
  }
  element('ad-empty').hidden = !!a && a.station === playerStation() && !a.filler;
  element('ad-content').hidden = !element('ad-empty').hidden;
  element('selection-empty').hidden = !!a; element('selection-content').hidden = !a;
  if (!a) return;
  const p = { name: a.programme_name, genre: a.genre, duration_minutes: a.duration_minutes };
  const own = a.station === playerStation();
  element('selected-genre').textContent = p.genre.slice(0, 2).toUpperCase();
  element('selected-id').textContent = `AIRING #${a.airing_id} · ${own ? 'YOUR CHANNEL' : 'COMPETITOR'}`;
  element('selected-name').textContent = p.name; element('selected-window').textContent = `${windowText(a)} · ${p.duration_minutes} min`;
  const airtime = p.duration_minutes + a.ad_minutes;
  const reserved = a.end_minute - a.start_minute;
  element('selected-runtime').textContent = airtime !== reserved || a.ad_minutes
    ? `${airtime} min airtime` : '';
  element('selected-status').textContent = `${a.status.replace('_', ' ').toUpperCase()}${a.booked ? ' · BOOKED' : own && a.editable ? ' · INTERNAL DRAFT' : ''}`;
  const owned = own ? report.catalogue.find(work => work.id === a.programme) : null;
  element('known-details').textContent = owned ? `${owned.genre} · ${owned.kind}${owned.episode ? ` ${owned.episode}` : ''} · ${owned.production_type}. ${owned.qualities.map(q => `${q.name} · ${q.level}`).join(", ")}. Cost per airing: ${money(owned.broadcast_cost)} Funds. Execution is unknown.` : `${p.genre} · ${own ? 'previously owned programme' : 'competitor programme'}. This work is outside your library. Its execution is unknown.`;
  if (a.filler) {
    const target = airingFor(a.teaser_for);
    element('known-details').textContent = a.teaser_for === null ? 'A one-minute station ident.'
      : a.teaser_current && target ? `Teases ${target.programme_name} at ${exactStamp(target.start_minute)}.`
      : 'The advertised slot changed. This minute plays as a station ident.';
  }
  if(owned && owned.production_id!==null)element('known-details').append(document.createTextNode(` Your original interview/discussion, production #${owned.production_id}.`));
  element<HTMLFieldSetElement>('edit-controls').disabled = !a.editable;
  if (moveFieldsFor !== a.airing_id) {
    select('move-day').value = String(Math.floor(a.start_tick / 96)); element<HTMLInputElement>('move-time').value = displayTime(a.start_minute);
    element<HTMLInputElement>('weekly-edit-time').value = displayTime(a.start_minute);
    moveFieldsFor = a.airing_id;
  }
  element('lock-note').textContent = !own ? 'The competitor schedule is read-only.' : a.sold_minutes > 0 ? 'Sold minutes protect this programme and start time. Adjust the ad block on this file.' : a.booked ? 'Booking protected: programme, start, duration and advertiser are locked.' : !a.editable ? 'This airing has already started. Past and on-air broadcasts cannot be edited.' : 'Moving keeps this airing ID. Removal leaves an explicit off-air gap.';
  select('advertiser').replaceChildren(option('', 'No advertisement'), ...report.advertisers.map(b => option(b.id, b.name)));
  select('advertiser').value = a.proposed_advertiser === null ? '' : String(a.proposed_advertiser);
  select('advertiser').disabled = !a.editable; button('set-advertiser').disabled = !a.editable;
  button('quote').disabled = !a.editable || a.proposed_advertiser === null;
  button('quote').textContent = `Request offer for #${a.airing_id}`;
  button('move').textContent = `Move this airing #${a.airing_id}`;
  element('weekly-edit-controls').hidden = a.weekly_id === null || !own;
  if (a.prior_slot_airing) element('selected-status').append(document.createTextNode(' · RETURNING TO REGULAR SLOT'));
  if (a.weekly_id !== null) element('selected-status').append(document.createTextNode(' · WEEKLY SLOT')); button('remove').textContent = `Remove #${a.airing_id}`;
  element('open-ad-sales').hidden = !own || a.filler || !!completed;
  element('open-review').hidden = !completed;
  element('ad-review').hidden = !completed;
  element('ad-title').textContent = p.name;
  element('ad-window').textContent = `Airing #${a.airing_id} · ${exactStamp(a.start_minute)} · ${p.duration_minutes} min programme`;
  renderAdMarket(a, report, send, advertiserName, money);
  element('legacy-ad-heading').hidden = a.ad_minutes > 0 || !own || a.filler;
  const legacyControls = element('advertiser').closest<HTMLElement>('.ad-controls')!;
  legacyControls.hidden = a.ad_minutes > 0 || !own || a.filler;
  const legacyHelp = legacyControls.nextElementSibling as HTMLElement;
  legacyHelp.hidden = legacyControls.hidden;
  const offer = element('offer'); offer.replaceChildren();
  if (report.offer?.airing_id === a.airing_id) {
    const q = report.offer;
    offer.className = 'contract-panel'; offer.append(node('h4', `Offer for airing #${q.airing_id} · ${airStamp(q.start_minute)}`));
    offer.append(node('p', `${advertiserName(q.advertiser)} offers up to ${money(q.maximum_payment)} Funds. Forecast from published audience estimates; each audience settles separately at ${stamp(q.deadline_tick)}.`));
    offer.append(table('Terms if accepted', ['Audience', 'Forecast relevant impressions', 'Promised micro-units', 'Allocated Funds'], q.targets.map(t => [t.audience, count(t.forecast_relevant_delivery), t.promised_units, money(t.allocated_payment)])));
    const accept = node('button', `Accept offer for airing #${q.airing_id}`); accept.id = 'accept'; accept.addEventListener('click', () => send({ type: 'accept_offer', airing_id: q.airing_id, offer_id: q.offer_id })); offer.append(accept);
  } else {
    offer.className = '';
    if (report.offer) { const note = node('p', 'A pending offer belongs to another airing: ', 'muted'); note.append(airingLink(report.offer.airing_id, `#${report.offer.airing_id}`)); offer.append(note); }
  }
  const contracts = element('selected-contract'); contracts.replaceChildren();
  const booking = report.contracts.find(c => c.airing_id === a.airing_id); if (booking) contracts.append(contractPanel(booking));
  button('advance-selected').textContent = `Run through selected airing #${a.airing_id} · to ${airStamp(a.end_minute)}`;
  button('advance-selected').disabled = !own || a.end_minute <= report.now_minute;
  displayAiringFile(a, completed);
}
function renderReports(): void {
  const results = element('results-list'); results.replaceChildren();
  if (!report.results.length) results.append(node('p', 'No completed broadcasts yet. Results arrive at each airing’s scheduled completion.', 'muted'));
  else results.append(table('Completed airings · select a row for audience counts and contract terms', ['Airing / programme', 'Broadcast window', 'Outcome', 'Cost', 'Cash after'], report.results.map(r => {
    const a = airingFor(r.airing_id)!;
    return [airingLink(r.airing_id, `#${r.airing_id} · ${a.programme_name}`, 'results'), windowText(a), r.off_air ? 'Off air' : 'Completed', money(r.broadcast_cost), money(r.cash_after)];
  })));
  const summary = element('finance-summary'); summary.replaceChildren();
  for (const [label, value] of [['Opening cash', report.finances.opening_cash], ['Broadcast costs', report.finances.costs_paid], ['Production committed', report.finances.production_committed], ['Ad receipts', report.finances.revenue], ['Rights purchases', report.finances.rights_spent], ['Rights resale', report.finances.resale_receipts], ['Current cash', report.cash]]) { const stat = node('div', '', 'finance-stat'); stat.append(node('span', label), node('strong', money(value))); summary.append(stat); }
  summary.append(node('p', `Production budget remaining in escrow: ${money(report.finances.production_in_escrow)} · spent on recording: ${money(report.finances.production_spent)}. Reservation reduces cash once; spending transfers those reserved funds.`, 'production-reserve-note'));
  const ledger = element('ledger'); ledger.replaceChildren();
  const amountHeading = node('span');
  amountHeading.append(fundsMark(), document.createTextNode(' Funds'));
  if (report.finances.ledger.length) ledger.append(table('Station ledger · exact integer Funds', ['Time', 'Airing / production', 'Transaction', amountHeading], report.finances.ledger.map(l => {
    const kinds: Record<string, string> = { broadcast_cost: 'Broadcast cost', audience_poll: 'Audience poll', advertising_payment: 'Advertising receipt', production_reserve: 'Production budget reserved (cash)', production_spend: 'Recording expense (from escrow)', rights_purchase: 'Rights purchase', rights_resale: 'Rights resale' };
    let link: string | HTMLButtonElement = l.airing_id === null ? '—' : airingLink(l.airing_id, `Airing #${l.airing_id}`);
    if (l.rights_title !== null) link = report.rights_market.titles.find(title => title.programme === l.rights_title)?.name ?? 'Programme rights';
    if (l.production_id !== null) { const id = l.production_id; link = node('button', `Production #${id}`, 'row-link'); link.addEventListener('click', () => {
      const film = report.film_desk.projects.find(f => f.id === id);
      desks.navigate(film ? (preparing(film) ? 'film-desk' : 'film-production') : 'production');
      if (film) filmUi?.showProject(id); else productionUi?.showProject(id);
    }); }
    return [stamp(l.tick), link, kinds[l.kind] ?? l.kind, money(l.amount)];
  })));
  else ledger.append(node('p', 'No station transactions yet. Advertiser escrow is separate from your station cash.', 'muted'));
}
function render(next: Report): void {
  report = next;
  for (const id of ['day', 'move-day']) {
    const list = select(id);
    if (list.options.length !== lastDay() + 1) {
      const value = list.value || '0';
      list.replaceChildren(...Array.from({ length: lastDay() + 1 }, (_, i) => option(i, dayLabel(i))));
      list.value = value;
    }
  }
  if (selected !== null && !airingFor(selected)) { selected = null; showingAiring = false; }
  element('clock').textContent = report.now_minute === report.end_tick * 15 ? 'Calendar complete' : exactStamp(report.now_minute);
  resourceHud.render({cash:money(report.cash),sway:String(report.production.sway_balance),insight:String(report.research?.insight ?? 0)});
  if (report.now_tick >= report.end_tick) playback?.pause();
  playback?.render();
  const nextAiring = airingFor(report.next_airing); const active = airingFor(report.active_airing);
  element('next-airing').textContent = nextAiring ? `#${nextAiring.airing_id} · ${nextAiring.programme_name} · ${airStamp(nextAiring.start_minute)}` : 'Nothing else planned on your channel';
  element('on-air').textContent = active ? `Currently ${active.status === 'off_air' ? 'off air' : 'on air'}: #${active.airing_id} · until ${stamp(active.end_tick)}` : 'Your channel is currently off air.';
  element('on-air').classList.toggle('live', active?.status === 'on_air');
  button('advance-next').disabled = !active && !nextAiring;
  button('advance-next').textContent = active ? `${active.status === 'off_air' ? 'Advance past off-air' : 'Finish on-air'} #${active.airing_id}` : nextAiring ? `Run next airing #${nextAiring.airing_id}` : 'No next airing';
  for (const id of ['advance-tick', 'advance-day', 'place']) button(id).disabled = report.now_tick >= report.end_tick;
  button('previous-day').disabled = viewedDay === 0; button('next-day').disabled = viewedDay === lastDay();
  element('reporting').textContent = report.reporting;
  calendar.followClock();
  calendar.render();
  renderOffice();
  renderAudience(report, stamp, send); renderTimeline(); renderCatalogue(); renderDetails(); renderReports(); productionUi?.render(report); newsroomUi?.render(report); hiringUi?.render(report); filmUi?.render(report); rightsUi?.render(report); weeklyUi?.render(report);

}
function send(command: Command, automatic = false): void {
  if (!automatic) playback?.pause();
  try {
    const previousIds = report?.schedule.map(a => a.airing_id) ?? [];
    const response: Response = JSON.parse(session.dispatch(JSON.stringify({ api_version: 8, command })));
    if (response.status === 'error') { playback?.pause(); error.textContent = `${response.error.code}: ${response.error.message}`; error.hidden = false; return; }
    error.hidden = true; productionUi?.commandSucceeded(command);
    const weeklyConfirmed = command.type === 'confirm_weekly';
    if (weeklyConfirmed) weeklyUi?.confirmed(Boolean(report?.weekly_review?.edition));
    if (command.type === 'move_exact' || command.type === 'reset' || weeklyConfirmed) moveFieldsFor = null;
    if (command.type === 'place_exact' || command.type === 'place_filler') {
      const created = response.report.schedule.find(a => !previousIds.includes(a.airing_id));
      if (created) {
        placementOverride = null; pickedGap = null; tapeShelf.close();
        selected = created.airing_id; showingAiring = true;
        viewedDay = Math.min(lastDay(), Math.floor(created.start_minute / DAY_MINUTES));
        plannerAnchor = created.start_minute % DAY_MINUTES;
        select('day').value = String(viewedDay);
      }
    }
    if (command.type === 'reset') {
      airingForecasts.clear(); officeTapeOpen = false; officeTrayOpen = false; placementOverride = null;
      calendar.reset();
      pickedGap = null; plannerAnchor = 1080; showingAiring = false; weeklyUi?.reset(); playback?.reset();
      libraryProgramme = 1; select('genre-filter').value = 'all'; selected = null; viewedDay = 0; select('day').value = '0';
    }
    render(response.report);
    if (command.type === 'accept_script') {
      desks.navigate('film-production');
      filmUi?.showProject(command.film_id);
    }
    if (command.type === 'set_ad_load') previewSlot();
    if ((command.type === 'place_exact' || command.type === 'place_filler') && selected !== null) {
      element('placement-preview').hidden = true;
      document.querySelector(`[data-airing="${selected}"]`)?.classList.add('tape-seated');
    }
    if (weeklyConfirmed) {
      const created = report.schedule.find(a => !previousIds.includes(a.airing_id));
      if (created) chooseAiring(created.airing_id, true);
      else desks.navigate('planning');
      status.textContent = 'Weekly schedule saved.';
    }
  } catch (failure) { playback?.pause(); error.textContent = `Browser adapter failure: ${String(failure)}. Reset or reload to retry.`; error.hidden = false; }
}
function selectedCommand(action: (a: Airing) => Command): void { const a = airingFor(selected); if (a) send(action(a)); }
function jump(tickInDay: number): void {
  const tickWidth = parseFloat(getComputedStyle(element('timeline')).getPropertyValue('--tick'));
  element('timeline-scroller').scrollLeft = tickInDay * tickWidth;
}
function renderOffice() {
  renderToday(report, playerStation(), id => {
    if (!isDesk()) desks.navigate('office');
    chooseAiring(id);
  }, (lane, start) => plannerDrag?.bindLane(lane, start, true));
  const dayStart = Math.min(lastDay(), Math.floor(report.now_minute / DAY_MINUTES)) * DAY_MINUTES;
  const next = opening(report, Math.max(report.now_minute, dayStart + 18 * 60), dayStart + DAY_MINUTES);
  const excerpt = next ? `Next gap · ${displayTime(next.start)}` : 'No evening gap today';
  element('notebook-cover').textContent = excerpt;
  const pages = element('notebook-pages');
  const schedule = leaf('On the desk today');
  if (next) {
    const gap = node('button', `${excerpt} · ${next.end - next.start} min`);
    gap.addEventListener('click', () => {
      element<HTMLDialogElement>('desk-notebook').close();
      desks.navigate('office');
      setPlanningDay(Math.floor(next.start / DAY_MINUTES));
      pickGap(next.start);
      element('library-search').focus();
    });
    schedule.append(gap);
  } else schedule.append(node('p', excerpt));
  const unsold = report.schedule.find(airing => airing.station === playerStation() && airing.status === 'planned'
    && airing.ad_minutes > airing.sold_minutes && airing.start_minute < dayStart + DAY_MINUTES);
  if (unsold) {
    const note = node('button', `${displayTime(unsold.start_minute)} · ${unsold.ad_minutes - unsold.sold_minutes} ad min unsold`);
    note.addEventListener('click', () => {
      element<HTMLDialogElement>('desk-notebook').close();
      desks.navigate('office');
      chooseAiring(unsold.airing_id);
    });
    schedule.append(note);
  }
  schedule.append(node('h4', 'Season goals'));
  for (const goal of report.station_goals) {
    const line = node('p');
    line.append(node(goal.progress >= goal.target ? 's' : 'span', goal.label),
      node('small', `${Math.min(goal.progress, goal.target)} / ${goal.target}`));
    schedule.append(line);
  }
  const quests = leaf('Sway requests');
  const accepted = report.production.requests.filter(request => request.acceptance !== null);
  if (!accepted.length) quests.append(node('p', 'No accepted requests.'));
  for (const request of accepted) {
    const line = node('p');
    const done = request.status === 'completed';
    line.append(node(done ? 's' : 'span', request.statement), node('small', done ? 'Completed'
      : `${request.status === 'expired' ? 'Expired' : 'Due'} · ${weekdayStamp(request.acceptance!.deadline_tick)}`));
    quests.append(line);
  }
  const openRequests = node('a', 'Open request board'); openRequests.href = '#production';
  openRequests.addEventListener('click', () => element<HTMLDialogElement>('desk-notebook').close());
  quests.append(openRequests);
  pages.replaceChildren(spread(schedule, quests));
}

function setPlanningDay(day: number, jumpToClock = false) {
  viewedDay = Math.max(0, Math.min(lastDay(), day));
  select('day').value = String(viewedDay);
  pickedGap = null;
  if (jumpToClock) plannerAnchor = report.now_minute % DAY_MINUTES;
  renderTimeline(); renderCatalogue(); renderOffice();
  if (jumpToClock) jump(Math.max(0, plannerAnchor / 15 - 4));
}
function dayChanged(): void {
  playback?.pause();
  setPlanningDay(Number(select('day').value));
  calendar.hold();
}

async function start(): Promise<void> {
  try {
    await init(); if (api_version() !== 8) throw new Error('Unsupported player protocol version');
    tapeShelf = initialiseTapeShelf();
    button('close-tape-record').addEventListener('click', () => element<HTMLDialogElement>('tape-record').close());
    calendar = initialiseCalendar({ report: () => report, day: () => viewedDay, selectDay: setPlanningDay, dayLabel });
    officeMotion = initialiseOfficeMotion();
    initialiseOfficeIdle();
    select('room-motion').addEventListener('change', event => {
      element('office').dataset.ambientMotion = (event.currentTarget as HTMLSelectElement).value;
    });
    desks = initialiseDesks(tapeShelf.close, desk => {
      activeDesk = desk;
      document.body.dataset.stationView = desk;
      if (desk === 'film-desk' || desk === 'film-production') {
        element(desk === 'film-desk' ? 'prep-mount' : 'film-mount').append(element('film-work'));
        filmUi?.setMode(desk === 'film-desk' ? 'preparation' : 'production');
        if (report) filmUi?.render(report);
      }
      if (isDesk()) {
        const mount = element(desk === 'office' ? 'office-workspace' : 'planning-workspace');
        mount.append(element('station-workspace'));
        tapeShelf.setOffice(desk === 'office');
        if (report) { renderCatalogue(); renderDetails(); renderOffice(); }
      } else {
        tapeShelf.close();
        restoreAdPaper();
      }
      if (desk === 'office') requestAnimationFrame(() => { if (activeDesk === 'office') officeMotion.restore(); });
    }, () => officeMotion.launched);
    const transport = document.querySelector<HTMLElement>('main > .transport')!;
    const positionProgramming = () => {
      const bottom = transport.getBoundingClientRect().bottom + window.scrollY;
      document.documentElement.style.setProperty('--programming-top', `${Math.ceil(bottom) + 4}px`);
    };
    const observer = new ResizeObserver(positionProgramming);
    for (const frame of [transport, document.querySelector('.masthead')!, document.querySelector('.rail')!]) observer.observe(frame);
    element('office-crt').addEventListener('click', () => calendar.dayMode());
    button('pull-tapes').addEventListener('click', () => {
      if (officeTrayOpen) { closeDeskFile(); return; }
      officeTrayOpen = true;
      showingAiring = false;
      setPlanningDay(Math.min(lastDay(), Math.floor(report.now_minute / DAY_MINUTES)));
      tapeShelf.open();
      element('library-search').focus();
    });
    button('put-tapes-away').addEventListener('click', closeDeskFile);
    const notebook = element<HTMLDialogElement>('desk-notebook');
    button('open-notebook').addEventListener('click', () => notebook.showModal());
    notebook.addEventListener('close', () => {
      if (activeDesk === 'office' && !officeTrayOpen) officeMotion.restore();
    });
    const showProgramme = (id: number) => {
      desks.navigate('planning'); tapeShelf.open();
      officeTapeOpen = true; officeTrayOpen = true; placementOverride = null;
      element<HTMLInputElement>('library-search').value = '';
      element<HTMLInputElement>('library-fit').checked = false;
      const series = report.catalogue.find(p => p.id === id)?.series_name;
      if (series) openSeries.add(series);
      libraryProgramme = id; select('genre-filter').value = 'all'; select('library-type').value = '';
      showingAiring = false;
      renderCatalogue(); element('library-title').scrollIntoView({ behavior: 'smooth', block: 'center' });
      document.querySelector<HTMLElement>(`[data-programme="${id}"]`)?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    };
    hiringUi = initialiseHiring({ send, stamp, money, showProject: id => productionUi?.showProject(id) });
    newsroomUi = initialiseNewsroom({ report: () => report, send, stamp: exactStamp, money, showAiring: id => chooseAiring(id, true) });
    element('prep-newsroom').addEventListener('click', event => {
      event.preventDefault();
      desks.navigate('production');
      const board = element<HTMLDetailsElement>('newsroom-board');
      board.open = true;
      board.querySelector('summary')?.focus();
    });
    productionUi = initialiseProduction({ report: () => report, send, stamp, money, showProgramme, showAiring: id => chooseAiring(id, true) });
    filmUi = initialiseFilmDesk({ send, stamp, money, showProgramme,
      openProject: film => {
        desks.navigate(preparing(film) ? 'film-desk' : 'film-production');
        filmUi?.showProject(film.id);
      },
    });
    if (activeDesk === 'film-production') filmUi.setMode('production');
    rightsUi = initialiseRightsMarket({ send, stamp, money, showProgramme, showAiring: id => chooseAiring(id, true) });
    weeklyUi = initialiseWeeklyPlanner({ send, stamp: exactStamp, dayLabel, showAiring: id => chooseAiring(id, true) });
    plannerDrag = initialisePlannerDrag({ report: () => report, player: playerStation, send, stamp: airStamp, time: displayTime, pause: () => playback?.pause() });
    plannerDrag.bindSource(element('desk-tape'), () => {
      const work = report.catalogue.find(tape => tape.id === libraryProgramme);
      const ads = work ? minutesFor(work) : 0;
      return work ? { programme: work.id, airing: null, minutes: work.duration_minutes + ads, adMinutes: placementOverride } : null;
    });
    session = new WebSession(); send({ type: 'inspect' });
    playback = initialisePlayback({
      state: () => report,
      tick: () => send({ type: 'advance', to_tick: Math.min(report.end_tick, report.now_tick + 1) }, true),
      onPlay: () => { error.hidden = true; },
    });
    element('place-form').addEventListener('submit', event => {
      event.preventDefault();
      if (!button('place').disabled) placeTape(placementMinute());
    });
    element('regular-slot').addEventListener('click', () => {
      const work = report.catalogue.find(tape => tape.id === libraryProgramme);
      if (work) weeklyUi?.choose(work.id, placementMinute() % DAY_MINUTES, adMinutes(work, report.ad_load_percent));
    });
    const ratio = element<HTMLInputElement>('ad-load');
    ratio.addEventListener('input', () => { element('ad-load-label').textContent = `${ratio.value}% of programme runtime`; });
    ratio.addEventListener('change', () => { placementOverride = null; send({ type: 'set_ad_load', percent: ratio.valueAsNumber }); });
    element<HTMLInputElement>('placement-ad-minutes').addEventListener('change', event => {
      const field = event.currentTarget as HTMLInputElement;
      placementOverride = field.validity.valid && Number.isInteger(field.valueAsNumber) ? field.valueAsNumber : null;
      renderCatalogue(); previewSlot();
    });
    element('open-ad-sales').addEventListener('click', () => {
      const market = element('ad-minute-market');
      market.tabIndex = -1;
      market.focus({ preventScroll: true });
      market.scrollIntoView({ block: 'nearest' });
    });
    document.addEventListener('keydown', event => {
      if (event.key !== 'Escape' || document.querySelector('dialog[open]')) return;
      if (isDesk() && (showingAiring || (activeDesk === 'office' && (officeTapeOpen || officeTrayOpen)))) {
        closeDeskFile();
      } else if (activeDesk !== 'office' && document.body.classList.contains('prop-workspace')) desks.navigate('office');
    });
    element('move').addEventListener('click', () => {
      const a = airingFor(selected);
      const [hour, minute] = element<HTMLInputElement>('move-time').value.split(':').map(Number);
      if (!a || !Number.isInteger(hour) || !Number.isInteger(minute)) return;
      const start = Number(select('move-day').value) * DAY_MINUTES + hour * 60 + minute;
      plannerDrag?.inspect({ programme: a.programme, airing: a.airing_id, minutes: a.duration_minutes + a.ad_minutes }, start);
      send({ type: 'move_exact', airing_id: a.airing_id, start_minute: start });
    });
    element('move-future').addEventListener('click', () => {
      selectedCommand(a => ({ type: 'preview_weekly_edit', airing_id: a.airing_id, minute_of_day: parseTime(element<HTMLInputElement>('weekly-edit-time').value) }));
      desks.navigate('weekly-planner');
      element('weekly-review').scrollIntoView({ block: 'nearest' });
    });
    element('remove').addEventListener('click', () => selectedCommand(a => ({ type: 'remove', airing_id: a.airing_id })));
    element('set-advertiser').addEventListener('click', () => selectedCommand(a => ({ type: 'set_advertiser', airing_id: a.airing_id, advertiser: select('advertiser').value === '' ? null : Number(select('advertiser').value) })));
    element('quote').addEventListener('click', () => selectedCommand(a => ({ type: 'request_offer', airing_id: a.airing_id })));
    element('advance-selected').addEventListener('click', () => selectedCommand(a => ({ type: 'advance_exact', to_minute: a.end_minute })));
    element('advance-tick').addEventListener('click', () => send({ type: 'advance', to_tick: report.now_tick + 1 }));
    element('advance-next').addEventListener('click', () => { const a = airingFor(report.active_airing) ?? airingFor(report.next_airing); if (a) send({ type: 'advance_exact', to_minute: a.end_minute }); });
    element('advance-day').addEventListener('click', () => send({ type: 'advance', to_tick: Math.min(report.end_tick, (Math.floor(report.now_tick / 96) + 1) * 96) }));
    element('reset').addEventListener('click', () => send({ type: 'reset' }));
    for (const id of ['genre-filter','library-fit','library-search','library-runtime','library-type']) element(id).addEventListener('input', renderCatalogue);
    select('day').addEventListener('change', dayChanged);
    element('previous-day').addEventListener('click', () => { select('day').value = String(viewedDay - 1); dayChanged(); });
    element('next-day').addEventListener('click', () => { select('day').value = String(viewedDay + 1); dayChanged(); });
    element('jump-evening').addEventListener('click', () => { plannerAnchor = 1080; pickedGap = null; renderTimeline(); renderCatalogue(); jump(64); });
    element('jump-midnight').addEventListener('click', () => { plannerAnchor = 0; pickedGap = null; renderTimeline(); renderCatalogue(); jump(0); });
    element('timeline-scroller').addEventListener('keydown', event => {
      if (!(event.target instanceof Element) || !['ArrowLeft', 'ArrowRight'].includes(event.key) || !event.shiftKey) return;
      event.preventDefault();
      const direction = event.key === 'ArrowRight' ? 1 : -1;
      const start = Math.min((viewedDay + 1) * DAY_MINUTES - 1,
        Math.max(viewedDay * DAY_MINUTES, report.now_minute, placementMinute() + direction));
      pickGap(start);
      element('timeline-scroller').focus({ preventScroll: true });
    });
    element('jump-now').addEventListener('click', () => { select('day').value = String(Math.min(lastDay(), Math.floor(report.now_tick / 96))); plannerAnchor = report.now_minute % DAY_MINUTES; dayChanged(); jump(Math.max(0, report.now_tick % 96 - 4)); });
    element('controls').hidden = false; document.body.classList.add('ready'); status.textContent = 'WASM READY · Station online';
    requestAnimationFrame(() => jump(64));
  } catch (failure) { status.textContent = 'WebAssembly initialization failed.'; error.textContent = `${String(failure)}. Reload to retry using the documented HTTP development server.`; error.hidden = false; }
}
void start();
