import { DAY_MINUTES, displayTime, parseTime } from './airtime';
import { showDeskPicker } from './desk-picker';
import { action, node } from './dom';

type SlotFields = {
  label: string;
  day?: HTMLSelectElement;
  time?: HTMLInputElement | HTMLSelectElement;
  firstWeekday?: () => number;
  repeats?: boolean;
};

const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
const daysPerPage = 14;
const twoDigits = (value: number) => String(value).padStart(2, '0');

/** Enhance existing form fields. Typed values and normal form validation still work. */
export function bindDateClock(fields: SlotFields) {
  for (const control of [fields.day, fields.time]) {
    if (!control) continue;
    control.classList.add('slot-card-field');
    control.setAttribute('aria-haspopup', 'dialog');
    control.addEventListener('pointerdown', event => {
      if (!(event instanceof PointerEvent) || event.button !== 0 || control.disabled) return;
      event.preventDefault();
      openDateClock(control, fields);
    });
    control.addEventListener('keydown', event => {
      if (!(event instanceof KeyboardEvent) || (event.key !== 'Enter' && !(event.altKey && event.key === 'ArrowDown'))) return;
      event.preventDefault();
      openDateClock(control, fields);
    });
  }
}

function openDateClock(trigger: HTMLElement, { label, day, time, firstWeekday, repeats }: SlotFields) {
  const panel = node('dialog', '', 'desk-picker date-clock');
  panel.setAttribute('aria-label', label);
  const form = node('form');
  const close = action('Cancel', () => panel.close());
  const cover = node('header', '', 'casting-gallery-cover');
  cover.append(node('h3', label), close);
  const instruments = node('div', '', 'date-clock-instruments');
  let selectedDay = day?.value;
  let hour: HTMLInputElement | undefined;
  let minute: HTMLInputElement | undefined;
  if (day) instruments.append(calendarPage(day, value => { selectedDay = value; }, repeats ? 0 : firstWeekday?.() ?? 0, !!repeats));
  if (time) {
    const step = time instanceof HTMLSelectElement ? 15 : 1;
    const value = time instanceof HTMLSelectElement ? Number(time.value) * step : parseTime(time.value);
    const clock = node('div', '', 'flip-clock');
    hour = flipCounter(clock, 'Hours', 23, 1, delta => spin(delta * 60));
    minute = flipCounter(clock, 'Minutes', 59, step, spin);
    function spin(delta: number) {
      const raw = Number(hour!.value) * 60 + Number(minute!.value) + delta;
      const current = (raw % DAY_MINUTES + DAY_MINUTES) % DAY_MINUTES;
      turnCard(hour!, Math.floor(current / 60));
      turnCard(minute!, current % 60);
    }
    spin(Number.isFinite(value) ? value : 0);
    instruments.append(clock);
  }
  const apply = node('button', 'Use slot', 'use-selected-slot');
  form.append(cover, instruments, apply);
  form.addEventListener('submit', event => {
    event.preventDefault();
    if (day && selectedDay !== undefined) day.value = selectedDay;
    if (time && hour && minute) {
      const chosen = hour.valueAsNumber * 60 + minute.valueAsNumber;
      time.value = time instanceof HTMLSelectElement ? String(chosen / 15) : displayTime(chosen);
    }
    panel.close();
    trigger.dispatchEvent(new Event('input', { bubbles: true }));
    trigger.dispatchEvent(new Event('change', { bubbles: true }));
  });
  panel.append(form);
  showDeskPicker(trigger, panel, instruments.querySelector<HTMLElement>('[aria-pressed="true"], input') ?? apply);
}

function turnCard(input: HTMLInputElement, value: number) {
  const digits = twoDigits(value);
  if (input.value === digits) return;
  input.value = digits;
  if (input.isConnected && !matchMedia('(prefers-reduced-motion: reduce)').matches) {
    input.animate([{ transform: 'perspective(8rem) rotateX(-38deg)', opacity: .72 },
      { transform: 'none', opacity: 1 }], { duration: 150, easing: 'ease-out' });
  }
}

function calendarPage(day: HTMLSelectElement, choose: (value: string) => void, firstWeekday: number, repeats: boolean) {
  const calendar = node('div', '', 'slot-calendar');
  const options = Array.from(day.options).filter(option => !option.disabled && option.value !== '');
  const indexes = options.map(option => Number(option.value) + firstWeekday);
  let selection = day.value;
  const first = indexes[0] ?? 0;
  const last = indexes.at(-1) ?? 0;
  let start = Math.max(Math.floor(first / daysPerPage), Math.floor((Number(selection) + firstWeekday) / daysPerPage)) * daysPerPage;
  const dates = node('div', '', 'slot-dates');
  const caption = node('span', '', 'slot-date-caption');
  const previous = action('‹', () => { start -= daysPerPage; render(); });
  const next = action('›', () => { start += daysPerPage; render(); });
  previous.setAttribute('aria-label', 'Earlier dates');
  next.setAttribute('aria-label', 'Later dates');
  const month = node('header');
  month.append(previous, caption, next);
  calendar.append(month, dates);
  function render() {
    dates.replaceChildren(...days.map(name => node('span', name, 'slot-weekday')));
    const limit = start + (repeats ? days.length : daysPerPage);
    const picked = options.find(option => option.value === selection);
    caption.textContent = picked?.textContent ?? 'Choose a date';
    previous.disabled = start <= first;
    next.disabled = limit > last;
    if (repeats) { previous.hidden = true; next.hidden = true; }
    for (let index = start; index < limit; index++) {
      const option = options.find(item => Number(item.value) === index - firstWeekday);
      if (!option) { dates.append(node('span')); continue; }
      const pick = action(repeats ? days[index] : String(Number(option.value) + 1), () => {
        selection = option.value;
        choose(selection);
        caption.textContent = option.textContent;
        for (const button of dates.querySelectorAll('button')) button.setAttribute('aria-pressed', String(button === pick));
      });
      pick.setAttribute('aria-label', option.textContent);
      pick.setAttribute('aria-pressed', String(option.value === selection));
      dates.append(pick);
    }
  }
  render();
  return calendar;
}

function flipCounter(clock: HTMLElement, label: string, maximum: number, step: number, spin: (change: number) => void) {
  const wrap = node('label', '', 'flip-counter');
  const input = node('input'); input.type = 'number'; input.min = '0'; input.max = String(maximum);
  input.step = String(step); input.required = true; input.value = '00';
  input.setAttribute('aria-label', label);
  const earlier = action('−', () => spin(-step));
  const later = action('+', () => spin(step));
  earlier.setAttribute('aria-label', `Earlier ${label.toLowerCase()}`);
  later.setAttribute('aria-label', `Later ${label.toLowerCase()}`);
  const keys = node('span', '', 'flip-keys');
  keys.append(earlier, later);
  wrap.append(node('span', label), input, keys);
  clock.append(wrap);
  return input;
}
