import type { Airing, Report } from '../protocol';
import { DAY_MINUTES, displayTime } from './airtime';
import { action, element, node } from './dom';

export function renderToday(report: Report, station: number, open: (airing: number) => void,
  bind: (lane: HTMLElement, start: number) => void) {
  const track = element('today-track');
  const start = Math.min(Math.floor(report.now_minute / DAY_MINUTES),
    Math.ceil(report.end_tick * 15 / DAY_MINUTES) - 1) * DAY_MINUTES;
  const end = start + DAY_MINUTES;
  const airings = report.schedule.filter(airing => airing.station === station
    && airing.start_minute < end && airing.end_minute > start);
  const next = report.schedule.find(airing => airing.airing_id === report.next_airing);
  const live = airings.find(airing => airing.airing_id === report.active_airing && airing.status === 'on_air');
  const describe = (airing: Airing) => `${displayTime(airing.start_minute)} · ${airing.programme_name}`;
  element('office-on-air').textContent = live ? `On air · ${live.programme_name}` : 'Off air';
  element('office-up-next').textContent = next ? `Up next · ${describe(next)}` : 'No next airing';
  const lane = node('div', '', 'today-lane');
  for (const airing of airings) {
    const aired = airing.end_minute <= report.now_minute;
    const block = aired ? action(describe(airing), () => open(airing.airing_id), 'today-block')
      : node('span', '', 'today-block');
    block.title = describe(airing);
    block.dataset.status = airing.status;
    block.style.left = `${100 * (Math.max(start, airing.start_minute) - start) / DAY_MINUTES}%`;
    block.style.width = `${100 * (Math.min(end, airing.end_minute) - Math.max(start, airing.start_minute)) / DAY_MINUTES}%`;
    lane.append(block);
  }
  const needle = node('span', '', 'today-needle');
  needle.style.left = `${100 * Math.min(DAY_MINUTES, report.now_minute - start) / DAY_MINUTES}%`;
  needle.title = `Current time ${displayTime(report.now_minute)}`;
  lane.append(needle);
  const scale = node('div', '', 'today-scale');
  for (const label of ['00', '06', '12', '18', '24']) scale.append(node('span', label));
  track.replaceChildren(lane, scale);
  bind(lane, start);
}
