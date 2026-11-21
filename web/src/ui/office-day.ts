import { element } from './dom';
import { DAY_MINUTES } from './airtime';
import { roomMotionEnabled } from './office-idle';

const evening = 18 * 60;
let previous = 0;
let hideExterior: number | undefined;

/** View of the simulated time. This never sends a clock or gameplay command. */
export function renderOfficeDay(now: number) {
  const office = element('office');
  const minute = now % DAY_MINUTES;
  office.dataset.shift = minute >= 8 * 60 && minute < evening ? 'open' : 'closed';
  office.dataset.staffEarly = String(minute >= 8 * 60 && minute < 17 * 60 + 45);
  office.dataset.staffLate = String(minute >= 8 * 60 + 30 && minute < evening);
  office.dataset.staffStudio = String(minute >= 9 * 60 && minute < 17 * 60 + 30);
  office.dataset.coffee = minute >= 8 * 60 && minute < 12 * 60 ? 'full' : 'empty';

  const closing = Math.floor(previous / DAY_MINUTES) === Math.floor(now / DAY_MINUTES)
    && previous % DAY_MINUTES < evening && minute >= evening && minute < evening + 15;
  previous = now;
  if (!closing || !element<HTMLInputElement>('overnight-transition').checked
    || document.body.dataset.stationView !== 'office' || office.matches('.file-open,.tray-open,.prop-travelling')
    || !roomMotionEnabled(office)) return;

  const exterior = office.querySelector<HTMLElement>('.station-exterior')!;
  window.clearTimeout(hideExterior);
  exterior.hidden = false;
  hideExterior = window.setTimeout(() => { exterior.hidden = true; }, 2200);
}
