import type { Report } from '../protocol';
import { DAY_MINUTES, displayTime, opening } from './airtime';
import { action, button, element, node, select } from './dom';

type Mode = 'day' | 'week' | 'month';
const periodLength = { day: 1, week: 7, month: 30 };

export function initialiseCalendar(ui: {
  report: () => Report;
  day: () => number;
  selectDay: (day: number, jumpToClock?: boolean) => void;
  dayLabel: (day: number) => string;
}) {
  let mode: Mode = 'day';
  let follows = true;
  const controls = Array.from(element('calendar-modes').querySelectorAll<HTMLButtonElement>('button'));
  const root = element('calendar-summary');
  const lastDay = () => Math.ceil(ui.report().end_tick / 96) - 1;
  const today = () => Math.min(lastDay(), Math.floor(ui.report().now_minute / DAY_MINUTES));

  function render() {
    const report = ui.report();
    const day = ui.day();
    element('planning').hidden = mode !== 'day';
    element('planning-workspace').hidden = mode !== 'day';
    button('open-library').hidden = mode !== 'day';
    root.hidden = mode === 'day';
    controls.forEach(control => control.setAttribute('aria-pressed', String(control.dataset.mode === mode)));
    element('calendar-position').textContent = follows ? 'Following the clock' : 'Browsing the calendar';
    if (mode === 'day') return;
    const span = periodLength[mode];
    const first = Math.floor(day / span) * span;
    const last = first + span - 1;
    const ownId = report.stations.find(station => station.is_player)?.id;
    const airings = report.schedule.filter(airing => airing.station === ownId);
    const heading = node('h1', `${mode === 'week' ? 'Week' : 'Month'} ${Math.floor(day / span) + 1}`);
    const header = node('div', '', 'calendar-summary-heading');
    const previous = action('Previous period', () => { follows = false; ui.selectDay(first - span); render(); });
    const next = action('Next period', () => { follows = false; ui.selectDay(last + 1); render(); });
    previous.disabled = first === 0;
    next.disabled = last >= lastDay();
    header.append(heading, previous, next);
    const dates = node('div', '', 'calendar-dates');
    for (let date = first; date <= last; date++) {
      const card = node('article', '', 'calendar-cell');
      if (date > lastDay()) {
        card.append(node('h2', ui.dayLabel(date)), node('p', 'Outside playable window'));
      } else {
        const start = date * DAY_MINUTES, end = start + DAY_MINUTES;
        const plan = airings.filter(airing => airing.start_minute < end && airing.end_minute > start && airing.status !== 'off_air');
        const used = plan.reduce((sum, airing) => sum + Math.min(end, airing.end_minute) - Math.max(start, airing.start_minute), 0);
        const unsold = plan.filter(airing => airing.start_minute >= start && airing.status === 'planned')
          .reduce((sum, airing) => sum + airing.ad_minutes - airing.sold_minutes, 0);
        const gap = opening(report, start, end);
        const title = node('h2');
        title.append(action(ui.dayLabel(date), () => dayMode(date)));
        card.append(title, node('p', gap ? `First gap ${displayTime(gap.start)}` : date < today() ? 'Day complete' : 'No remaining gap'),
          node('p', `${(used / DAY_MINUTES * 100).toFixed(1)}% occupied`),
          node('p', `${unsold} ad min unsold`));
      }
      dates.append(card);
    }
    const runs = new Map<number, (typeof airings)[number]>();
    for (const airing of airings) {
      if (airing.weekly_id === null || airing.start_minute < first * DAY_MINUTES || airing.start_minute >= (last + 1) * DAY_MINUTES) continue;
      if ((runs.get(airing.weekly_id)?.start_minute ?? -1) < airing.start_minute) runs.set(airing.weekly_id, airing);
    }
    const recurring = node('section', '', 'calendar-recurring');
    recurring.append(node('h2', 'Last scheduled airing in each run'));
    for (const airing of runs.values()) {
      const episode = report.catalogue.find(tape => tape.id === airing.programme)?.episode;
      recurring.append(node('p', `${airing.programme_name}${episode === null || episode === undefined ? '' : ` · Episode ${episode}`} · ${ui.dayLabel(Math.floor(airing.start_minute / DAY_MINUTES))} · ${displayTime(airing.start_minute)}`));
    }
    if (!runs.size) recurring.append(node('p', 'No regular runs in this period.'));
    root.replaceChildren(header, dates, recurring);
  }

  function dayMode(day?: number) {
    mode = 'day';
    if (day !== undefined) { follows = false; ui.selectDay(day); }
    if (follows && ui.day() !== today()) ui.selectDay(today());
    render();
  }
  controls.forEach(control => control.addEventListener('click', () => {
    mode = control.dataset.mode as Mode;
    if (follows && ui.day() !== today()) ui.selectDay(today());
    render();
  }));
  button('calendar-today').addEventListener('click', () => {
    follows = true;
    ui.selectDay(today(), mode === 'day');
    render();
  });

  return {
    hold() { follows = false; render(); },
    reset() { follows = true; mode = 'day'; },
    followClock() {
      if (!follows) return;
      const date = today();
      if (date !== ui.day()) {
        select('day').value = String(date);
        ui.selectDay(date);
      }
    },
    dayMode,
    render,
  };
}
