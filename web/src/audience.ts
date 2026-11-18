import type { Command, Polling, Report } from './protocol';
import { element, node, option } from './ui/dom';

const tiers: [Polling, string][] = [
  ['off', 'Off'], ['daily', 'Daily · 60 credits per poll'],
  ['every_three_days', 'Every 3 days · 90 credits per poll'],
];
const affinity = (value: number) => value >= .67 ? 'High' : value >= .34 ? 'Medium' : 'Low';

export function renderAudience(report: Report, stamp: (tick: number) => string, send: (command: Command) => void) {
  const root = element('audience-research');
  const research = report.research;
  if (!research) return;
  root.replaceChildren(node('p', `Insight available: ${research.insight}. Spending improves the next poll for that group.`));
  for (const group of research.groups) {
    const card = node('article', '', 'production-card');
    const snapshot = group.estimate;
    const ageHours = Math.floor((report.now_tick - snapshot.tick) / 4);
    const size = Math.round(snapshot.population / 100) * 100;
    const age = Math.round(snapshot.typical_age / 5) * 5;
    card.append(
      node('h3', group.name),
      node('p', `Estimated size: about ${size.toLocaleString()} · Typical age: about ${age}`),
      node('p', ['Comedy', 'Drama', 'Science'].map((genre, i) => `${genre}: ${affinity(snapshot.tastes[i])}`).join(' · ')),
      node('p', `${stamp(snapshot.tick)} · ${ageHours}h old · Familiarity at poll: ${snapshot.familiarity_level} · Uncertainty: ${snapshot.uncertainty >= .22 ? 'High' : snapshot.uncertainty >= .12 ? 'Moderate' : 'Low'}`),
    );
    if (group.missed_poll) card.append(node('p', 'The last poll was skipped because cash was too low.'));
    if (group.next_tick !== null && group.next_tick <= report.end_tick) card.append(node('p', `Next poll: ${stamp(group.next_tick)}`));
    const controls = node('div', '', 'form-row');
    const label = node('label', 'Polling frequency');
    const select = node('select');
    for (const [value, title] of tiers) select.append(option(value, title));
    select.value = group.polling;
    select.addEventListener('change', () => {
      const polling = select.value as Polling;
      select.value = group.polling;
      send({ type: 'set_polling', audience: group.id, polling });
      root.querySelector<HTMLSelectElement>(`#polling-${group.id}`)?.focus();
    });
    select.id = `polling-${group.id}`;
    label.append(select);
    const form = node('form', '', 'form-row');
    const spendLabel = node('label', 'Insight to spend');
    const input = node('input');
    input.type = 'number';
    input.min = '1';
    input.max = String(research.insight);
    input.step = '1';
    input.value = '1';
    input.required = true;
    const spend = node('button', 'Improve next estimate');
    spend.disabled = input.disabled = research.insight === 0;
    spendLabel.append(input);
    form.append(spendLabel, spend);
    form.addEventListener('submit', event => {
      event.preventDefault();
      send({ type: 'spend_insight', audience: group.id, amount: input.valueAsNumber });
      root.querySelector<HTMLSelectElement>(`#polling-${group.id}`)?.focus();
    });
    controls.append(label, form);
    card.append(controls);
    root.append(card);
  }
}
