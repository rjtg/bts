import type { BroadcastResult, Programme, Report } from '../protocol';
import { audienceOpportunities, bestOpening, DAY_MINUTES, displayTime, turnout } from './airtime';
import { leaf as sheet, spread } from './dossier';
import { node, option } from './dom';
import { programmeChart, programmeCredits, radar } from './programme-paper';

export function officePapers(tape: Programme, report: Report, history: BroadcastResult[], details: {
  next: number | undefined;
  start: number;
  stamp: (minute: number) => string;
  recommendation: (start: number) => string;
  showResult: (result: BroadcastResult) => HTMLButtonElement;
  receipts: bigint;
  costs: bigint;
  money: (value: string) => string;
}) {
  const { next, stamp, money } = details;
  const best = bestOpening(tape, report, details.start)?.start;
  function estimate(minute: number | undefined) {
    const index = minute === undefined ? null : turnout(tape, report, minute);
    return index === null ? 'No estimate on file' : `About ${Math.round(index / 100) * 100} potential viewers`;
  }
  const profile = sheet('On the tape');
  profile.append(programmeChart(tape));
  const audience = sheet('Audience outlook');
  if (next !== undefined) audience.append(node('h4', `Next gap · ${stamp(next)}`), node('p', estimate(next)));
  else audience.append(node('p', 'No remaining gap on this day.'));
  if (best !== undefined) audience.append(node('h4', `Suggested · ${stamp(best)}`), node('p', estimate(best)));
  audience.append(node('p', 'Survey-based potential. Competition and repeat viewing can change turnout.', 'paper-footnote'),
    node('p', details.recommendation(next ?? details.start), 'handwritten'));

  const previous = sheet('Last transmission');
  const regular = report.schedule.find(airing => airing.programme === tape.id
    && airing.status === 'planned' && airing.prior_slot_airing);
  const last = history[0];
  if (last) {
    const labels = last.audiences.map(group => group.audience);
    const values = last.audiences.map(group => group.population > 0 ? group.player_viewers / group.population : 0);
    previous.append(node('p', stamp(last.start_minute)), radar(labels, values, 'Last airing share of each audience group'),
      node('p', 'Share of each group who tuned in · provisional counts', 'paper-footnote'));
    for (const [index, group] of last.audiences.entries()) {
      previous.append(node('p', `${group.audience} · ${Math.round(values[index] * 100)}% ${group.film_reception ? `· ${group.film_reception}` : ''}`));
    }
    previous.append(details.showResult(last));
  } else previous.append(node('p', 'We haven’t aired this tape yet.', 'handwritten'));

  const expected = sheet('Before it airs');
  const withLeadIn = report.schedule.find(airing => airing.programme === tape.id
    && airing.status === 'planned' && airing.lead_in_airing_id !== null);
  const leadIn = report.schedule.find(airing => airing.airing_id === withLeadIn?.lead_in_airing_id);
  if (leadIn) expected.append(node('p', `Possible lead-in · ${leadIn.programme_name}`, 'small'));
  if (regular) expected.append(node('p', 'One planned airing returns to a slot this run has used before.', 'small'));
  const forecast = node('div');
  const time = node('input'); time.type = 'time'; time.step = '60';
  const date = node('select');
  for (let day = Math.floor(report.now_minute / DAY_MINUTES); day < Math.ceil(report.end_tick * 15 / DAY_MINUTES); day++) {
    date.append(option(String(day), `Day ${day + 1}`));
  }
  const dateLabel = node('label', 'Date '); dateLabel.append(date);
  const timeLabel = node('label', 'Time '); timeLabel.append(time);
  const picker = node('div', '', 'reception-picker'); picker.append(dateLabel, timeLabel);
  function show(minute: number) {
    const groups = audienceOpportunities(tape, report, minute);
    const names = report.research?.groups.map(group => group.name) ?? [];
    const strength = names.map(name => groups.find(group => group.name === name)?.share);
    forecast.replaceChildren(node('p', stamp(minute)), node('p', estimate(minute)));
    if (groups.length) forecast.append(radar(names, strength, 'Survey-based audience opportunity by group'),
      node('p', 'Survey interest index by group. Uncharted groups have limited evidence.', 'paper-footnote'));
  }
  function choose(minute: number) {
    date.value = String(Math.floor(minute / DAY_MINUTES));
    time.value = displayTime(minute);
    show(minute);
  }
  picker.addEventListener('change', () => {
    if (!time.value || !date.value) return;
    const [hours, minutes] = time.value.split(':').map(Number);
    const minute = Number(date.value) * DAY_MINUTES + hours * 60 + minutes;
    if (minute >= report.now_minute && minute < report.end_tick * 15) show(minute);
    else forecast.replaceChildren(node('p', 'Choose a time remaining in this scenario.'));
  });
  const custom = node('details', '', 'try-airtime');
  custom.append(node('summary', 'Try another time'), picker);
  choose(next ?? best ?? report.now_minute);
  if (next === undefined) forecast.prepend(node('p', 'No gap left today.'));
  expected.append(forecast);
  if (best !== undefined && best !== next) expected.append(node('p', `Try ${stamp(best)}.`, 'handwritten'));
  expected.append(custom);

  const members = sheet('Production credits');
  members.append(programmeCredits(tape, report, stamp));
  const creditNotes = sheet('Cast notes');
  creditNotes.append(node('p', 'Select a name to open their public file.', 'handwritten'));
  const funds = sheet('Funds');
  const film = report.film_desk.projects.find(project => project.programme === tape.id);
  const discussion = report.production.projects.find(project => project.programme === tape.id);
  const rights = report.finances.ledger.filter(entry => entry.rights_title === tape.id);
  const rightsTotal = (kind: string) => rights.filter(entry => entry.kind === kind)
    .reduce((sum, entry) => sum + BigInt(entry.amount), 0n);
  const productionCost = film?.committed ?? discussion?.budget;
  const fundsText = (value: bigint) => `${money(String(value))} Funds`;
  funds.append(node('p', `Ad receipts · ${fundsText(details.receipts)}`), node('p', `Airing costs · ${fundsText(details.costs)}`));
  if (productionCost !== undefined) funds.append(node('p', `Production · ${fundsText(BigInt(productionCost))}`));
  if (rights.length) funds.append(node('p', `Rights paid · ${fundsText(rightsTotal('rights_purchase'))}`),
    node('p', `Resale receipts · ${fundsText(rightsTotal('rights_resale'))}`));
  const resources = sheet('Other resources');
  const insight = report.rights_market.spending.filter(entry => entry.programme === tape.id).reduce((sum, entry) => sum + entry.amount, 0);
  const sway = report.production.sway_history.filter(entry => discussion && entry.production_id === discussion.id);
  const spent = sway.reduce((sum, entry) => sum - Math.min(0, entry.change), 0);
  const earned = sway.reduce((sum, entry) => sum + Math.max(0, entry.change), 0);
  resources.append(node('p', `Insight spent · ${insight}`), node('p', `Sway spent · ${spent}`),
    node('p', `Sway earned · ${earned}`), node('p', 'Only transactions linked to this tape appear here.', 'paper-footnote'));
  return [spread(profile, audience), spread(previous, expected), spread(members, creditNotes), spread(funds, resources)];
}
