import type { Command, Report, WeeklyPlan } from './protocol';
import { action, element, node, option } from './ui/dom';

export const WEEKDAYS = ['Monday','Tuesday','Wednesday','Thursday','Friday','Saturday','Sunday'];
const reasons: Record<string, string> = {
  overlap: 'Overlaps a scheduled airing', occurrence_overlap: 'Two requested dates overlap',
  invalid_time: 'Outside the remaining schedule', placement_overflow: 'Outside the scenario',
};

export function initialiseWeeklyPlanner(ui: {
  send: (command: Command) => void; stamp: (tick: number) => string;
  time: (tick: number) => string; dayLabel: (day: number) => string; showAiring: (id: number) => void;
}) {
  const form = node('form', '', 'weekly-options');
  const programme = node('select'), weekday = node('select'), time = node('select'), count = node('select'), ads = node('input');
  for (const [i, name] of WEEKDAYS.entries()) weekday.append(option(i, name));
  weekday.value = '5';
  for (let i = 0; i < 96; i++) time.append(option(i, ui.time(i)));
  time.value = '72';
  for (let i = 1; i <= 3; i++) count.append(option(i, String(i)));
  count.value = '3';
  ads.type = 'number'; ads.min = '0'; ads.max = '30'; ads.step = '1'; ads.value = '10'; ads.required = true;
  const label = (title: string, control: HTMLElement) => { const l = node('label', title); l.append(control); return l; };
  const review = element('weekly-review');
  let draft: WeeklyPlan | null = null;
  const title = node('h3', 'Review the dates');
  const ask = () => { if (draft) ui.send({ type: 'preview_weekly', plan: draft }); };
  const preview = node('button', 'Preview weekly slots');
  const lastResult = node('p', '', 'weekly-confirmed');
  form.append(label('Weekly programme', programme), label('Weekday', weekday), label('Regular airtime', time),
    label('Weeks', count), label('Default ad minutes', ads), preview);
  element('weekly-form').append(form, lastResult);
  form.addEventListener('change', () => review.replaceChildren());
  form.addEventListener('submit', event => {
    event.preventDefault();
    draft = { programme: Number(programme.value), weekday: Number(weekday.value),
      time: Number(time.value), count: Number(count.value), ad_minutes: ads.valueAsNumber, exceptions: [] };
    lastResult.textContent = '';
    ask();
  });
  function exception(index: number, target: number | null) {
    if (!draft) return;
    draft.exceptions = draft.exceptions.filter(item => item.occurrence !== index);
    draft.exceptions.push({ occurrence: index, start_tick: target });
    ask();
  }
  return {
    render(report: Report) {
      const selected = programme.value;
      if (Array.from(programme.options).map(item => Number(item.value)).join() !== report.catalogue.map(p => p.id).join()) {
        programme.replaceChildren(...report.catalogue.map(p => option(p.id, p.name)));
        if (report.catalogue.some(p => String(p.id) === selected)) programme.value = selected;
      }
      preview.disabled = report.now_tick >= report.end_tick || programme.options.length === 0;
      review.replaceChildren();
      const model = report.weekly_review;
      if (!model) return;
      review.append(title);
      const list = node('ol', '', 'weekly-dates');
      for (const row of model.rows) {
        const card = node('li', '', `weekly-date${row.issue ? ' weekly-conflict' : ''}`);
        card.append(node('strong', ui.stamp(row.regular_tick)),
          node('p', row.start_tick === null ? 'Skipped' :
            `${row.start_tick !== row.regular_tick ? `Moved to ${ui.stamp(row.start_tick)} · ` : ''}${row.runtime_minutes} min runtime · holds until ${row.end_tick === null ? 'outside the calendar' : ui.stamp(row.end_tick)}`));
        if (row.protected) card.append(node('p', 'Protected · keeps its existing time'));
        if (row.issue) card.append(node('p', reasons[row.issue] ?? 'This occurrence cannot be booked.'));
        for (const id of row.conflicts) {
          const other = report.schedule.find(a => a.airing_id === id);
          card.append(action(other ? `Conflict: ${other.programme_name}` : 'Show conflict', () => ui.showAiring(id)));
        }
        if (!model.edition && row.start_tick !== null) {
          const move = node('details');
          const day = node('select'), hour = node('select');
          const start = row.start_tick;
          for (let d = 0; d < Math.ceil(report.end_tick / 96); d++) day.append(option(d, ui.dayLabel(d)));
          for (let t = 0; t < 96; t++) hour.append(option(t, ui.time(t)));
          day.value = String(Math.floor(start / 96));
          hour.value = String(start % 96);
          move.append(node('summary', 'Move this occurrence'), label('New date', day), label('New time', hour),
            action('Update preview', () => exception(row.occurrence, Number(day.value) * 96 + Number(hour.value))));
          card.append(move, action('Skip this occurrence', () => exception(row.occurrence, null)));
        } else if (!model.edition) {
          card.append(action('Restore regular slot', () => exception(row.occurrence, row.regular_tick)));
        }
        list.append(card);
      }
      const confirm = action(model.edition ? 'Confirm future moves' : 'Confirm weekly schedule', () => ui.send({ type: 'confirm_weekly' }), 'weekly-primary');
      confirm.disabled = model.rows.some(row => row.issue !== null) || !model.rows.some(row => row.start_tick !== null && !row.protected);
      review.append(list, confirm);
    },
    confirmed(edited: boolean) {
      lastResult.textContent = edited ? 'Future drafts moved. Protected airings kept their time.' : 'Weekly slots added. Each airing can sell its own ads.';
    },
    reset() { lastResult.textContent = ''; draft = null; },
  };
}
