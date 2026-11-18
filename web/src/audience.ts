import type { Command, Polling, Report } from './protocol';
import { element, node } from './ui/dom';
import { rotaryDial } from './ui/dial';
import { starRating } from './ui/stars';

const portraits: Partial<Record<number, string>> = {
  1: 'young-viewers',
  2: 'households',
  3: 'specialists',
};

const tiers: { id: Polling; icon: string; title: string; cost: string }[] = [
  { id: 'off', icon: 'Ⅱ', title: 'Off', cost: 'No charge' },
  { id: 'daily', icon: '▥', title: 'Daily', cost: '60 credits per survey' },
  { id: 'every_three_days', icon: '▦', title: 'Every 3 days', cost: '90 credits per survey' },
];

function symbol(glyph: string): HTMLElement {
  const icon = node('span', glyph, 'research-icon');
  icon.setAttribute('aria-hidden', 'true');
  return icon;
}

export function renderAudience(report: Report, stamp: (tick: number) => string, send: (command: Command) => void) {
  const root = element('audience-research');
  const research = report.research;
  if (!research) return;
  const desk = node('section', '', 'production-card research-desk');
  const heading = node('h3', 'Audience surveys');
  heading.prepend(symbol('▤'));
  const stock = node('p', `Insight available: ${research.insight}`, 'insight-stock');
  stock.prepend(symbol('◈'));
  const choices = node('div', '', 'survey-choices');
  choices.setAttribute('role', 'group');
  choices.setAttribute('aria-label', 'Polling frequency for all audiences');
  for (const tier of tiers) {
    const button = node('button', '', 'survey-choice');
    button.dataset.polling = tier.id;
    button.setAttribute('aria-pressed', String(research.polling === tier.id));
    button.append(symbol(tier.icon), node('strong', tier.title), node('small', tier.cost));
    button.addEventListener('click', () => {
      send({ type: 'set_polling', polling: tier.id });
      root.querySelector<HTMLButtonElement>(`[data-polling="${tier.id}"]`)?.focus();
    });
    choices.append(button);
  }
  desk.append(heading, stock, choices);
  if (research.missed_poll) desk.append(node('p', 'The last survey was skipped because cash was too low.'));
  if (research.next_tick !== null && research.next_tick <= report.end_tick) {
    desk.append(node('p', `Next survey for all groups · ${stamp(research.next_tick)}`));
  }
  const draftAmounts = new Map(Array.from(
    root.querySelectorAll<HTMLInputElement>('.rotary-dial input'),
    input => [input.id, input.valueAsNumber],
  ));
  root.replaceChildren(desk);
  for (const group of research.groups) {
    const card = node('article', '', 'production-card audience-card');
    const header = node('header', '', 'audience-card-header');
    const dossier = node('div', '', 'audience-dossier');
    const portrait = portraits[group.id];
    if (portrait) {
      const image = node('img', '', 'audience-portrait');
      image.src = `${import.meta.env.BASE_URL}audiences/${portrait}.webp`;
      image.alt = '';
      header.append(image);
    }
    const snapshot = group.estimate;
    const ageHours = Math.floor((report.now_tick - snapshot.tick) / 4);
    const size = Math.round(snapshot.population / 100) * 100;
    const age = Math.round(snapshot.typical_age / 5) * 5;
    dossier.append(
      node('h3', group.name),
      node('p', `Estimated size: about ${size.toLocaleString()} · Typical age: about ${age}`),
      node('p', `${stamp(snapshot.tick)} · ${ageHours}h old · Familiarity at poll: ${snapshot.familiarity_level} · Uncertainty: ${snapshot.uncertainty >= .22 ? 'High' : snapshot.uncertainty >= .12 ? 'Moderate' : 'Low'}`),
    );
    header.append(dossier);
    card.append(header);
    const tastes = node('ul', '', 'audience-tastes');
    for (const [quality, halfStars] of Object.entries(snapshot.tastes)) {
      const row = node('li');
      const label = node('span', quality);
      if (quality === 'Playfulness') label.title = 'Fewer stars indicate a preference for a more serious register.';
      row.append(label, starRating(halfStars));
      tastes.append(row);
    }
    card.append(node('h4', 'What they like'), tastes);
    const form = node('form', '', 'investigate-panel');
    const input = node('input');
    const id = `insight-${group.id}`;
    input.id = id;
    input.type = 'range';
    input.min = '0';
    input.max = String(research.insight);
    input.step = '1';
    input.value = String(Math.min(draftAmounts.get(id) ?? 0, research.insight));
    input.disabled = research.insight === 0;
    input.setAttribute('aria-label', 'Insight to spend');
    const title = node('h4', 'Investigate');
    const cost = node('output', '', 'investigate-cost');
    cost.setAttribute('for', id);
    const spend = node('button', 'Spend Insight', 'investigate-spend');
    const refresh = () => {
      const amount = input.valueAsNumber;
      cost.textContent = `${amount} Insight · ${research.insight - amount} left`;
      spend.disabled = amount === 0;
      spend.setAttribute('aria-label', `Spend ${amount} Insight`);
    };
    input.addEventListener('input', refresh);
    refresh();
    const brief = node('div', '', 'investigate-brief');
    brief.append(title, node('p', 'Get a clearer read on this group in the next survey.'), cost, spend);
    form.append(rotaryDial(input), brief);
    form.addEventListener('submit', event => {
      event.preventDefault();
      if (spend.disabled) return;
      const amount = input.valueAsNumber;
      input.value = '0';
      send({ type: 'spend_insight', audience: group.id, amount });
      const next = root.querySelector<HTMLInputElement>(`#${id}`);
      if (next?.disabled) root.querySelector<HTMLButtonElement>('[data-polling][aria-pressed="true"]')?.focus();
      else next?.focus();
    });
    card.append(form);
    root.append(card);
  }
}
