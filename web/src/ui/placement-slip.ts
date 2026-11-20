import { DAY_MINUTES, displayTime, parseTime } from './airtime';
import type { Programme } from '../protocol';
import { action, node, option } from './dom';

export function placementSlip(work: Programme, start: number, ads: number, ui: {
  stamp: (minute: number) => string;
  dateLabel: (day: number) => string;
  firstDay: number;
  lastDay: number;
  inspect: (start: number, minutes: number) => string | null;
  confirm: (start: number, ads: number) => string | null;
  retryFocus: () => void;
}) {
  const dialog = node('dialog', '', 'placement-slip');
  const form = node('form');
  const heading = node('h2', 'Airtime booking');
  heading.id = 'placement-slip-title';
  dialog.setAttribute('aria-labelledby', heading.id);
  const input = node('input');
  input.type = 'number'; input.required = true; input.min = '0'; input.max = '30'; input.step = '1';
  input.value = String(ads);
  const time = node('input');
  time.type = 'time'; time.required = true; time.value = displayTime(start);
  const day = node('select');
  for (let value = Math.min(ui.firstDay, Math.floor(start / DAY_MINUTES)); value <= ui.lastDay; value++) day.append(option(value, ui.dateLabel(value)));
  day.value = String(Math.floor(start / DAY_MINUTES));
  const date = node('label', 'Day '); date.append(day);
  const clock = node('label', 'Start time '); clock.append(time);
  const minute = () => Number(day.value) * DAY_MINUTES + parseTime(time.value);
  const label = node('label', 'Ad minutes '); label.append(input);
  const starts = node('p');
  const preview = node('p'); preview.setAttribute('aria-live', 'polite');
  const problem = node('p', '', 'placement-problem'); problem.setAttribute('role', 'status');
  const submit = node('button', 'Confirm placement');
  const refresh = () => {
    const length = work.duration_minutes + input.valueAsNumber;
    const valid = input.validity.valid && time.validity.valid;
    starts.textContent = time.validity.valid ? `Starts ${ui.stamp(minute())}` : '';
    preview.textContent = valid
      ? `${work.duration_minutes} min programme + ${input.value} min ads · ${length} min · ends ${ui.stamp(minute() + length)}` : '';
    problem.textContent = valid ? ui.inspect(minute(), length) ?? '' : 'Choose a start time and enter 0–30 whole ad minutes.';
    submit.disabled = !valid;
  };
  let saved = false;
  form.addEventListener('input', refresh);
  form.addEventListener('submit', event => {
    event.preventDefault();
    const failure = ui.confirm(minute(), input.valueAsNumber);
    if (failure !== null) { problem.textContent = failure; return; }
    saved = true;
    dialog.close();
  });
  form.append(heading, node('h3', work.name), starts, date, clock, label, preview, problem,
    submit, action('Cancel', () => dialog.close()));
  dialog.append(form);
  document.body.append(dialog);
  dialog.addEventListener('close', () => {
    dialog.remove();
    if (!saved) ui.retryFocus();
    else document.querySelector<HTMLElement>('[aria-label="Airing dossier"] .dossier-close')?.focus();
  }, { once: true });
  refresh();
  dialog.showModal();
}
