import type { Programme, Report } from '../protocol';
import { surveyEvidence } from './programme-assistant';

export const DAY_MINUTES = 1440;
export const displayTime = (minute: number) => {
  const local = minute % DAY_MINUTES;
  return `${String(Math.floor(local / 60)).padStart(2, '0')}:${String(local % 60).padStart(2, '0')}`;
};
export const parseTime = (value: string) => {
  const [hour, minute] = value.split(':').map(Number);
  return hour * 60 + minute;
};
export const airStamp = (minute: number) => `Day ${Math.floor(minute / DAY_MINUTES) + 1} · ${displayTime(minute)}`;
export const adMinutes = (tape: Programme, percent: number) =>
  Math.min(30, Math.round(tape.duration_minutes * percent / 100));

export function opening(report: Report, anchor: number, limit: number) {
  for (const gap of report.gaps) {
    const start = Math.max(anchor, gap.start_minute, report.now_minute);
    const end = Math.min(limit, gap.end_minute);
    if (start < end) return { start, end };
  }
  return null;
}

/** A comparative estimate from published survey tastes and dated times, in estimated audience opportunity units. */
export function audienceOpportunities(tape: Programme, report: Report, minute: number) {
  const weekday = (report.first_weekday + Math.floor(minute / DAY_MINUTES)) % 7;
  const hour = Math.floor(minute % DAY_MINUTES / 60);
  const band = hour < 12 ? 'mornings' : hour < 18 ? 'afternoons' : 'evenings';
  const watchTime = `${weekday >= 5 ? 'Weekend' : 'Weekday'} ${band}`;
  const groups = surveyEvidence(tape, report);
  return groups.map(group => {
    const timing = group.estimate.watch_time_note?.startsWith(watchTime) ? 1 : .65;
    const share = group.fit / 10 * timing;
    return { name: group.name, share, viewers: group.estimate.population * share };
  });
}

export function turnout(tape: Programme, report: Report, minute: number): number | null {
  const groups = audienceOpportunities(tape, report, minute);
  return groups.length ? groups.reduce((total, group) => total + group.viewers, 0) : null;
}

export function bestOpening(tape: Programme, report: Report, anchor: number) {
  const runtime = tape.duration_minutes + adMinutes(tape, report.ad_load_percent);
  const until = Math.min(report.end_tick * 15, anchor + 7 * DAY_MINUTES);
  let best: { start: number; score: number } | null = null;
  for (const gap of report.gaps) {
    const start = Math.max(anchor, gap.start_minute, report.now_minute);
    const end = Math.min(until, gap.end_minute);
    if (start + runtime > end) continue;
    const candidates = [start];
    for (let hour = (Math.floor(start / 60) + 1) * 60; hour + runtime <= end; hour += 60) {
      candidates.push(hour);
    }
    for (const candidate of candidates) {
      const score = turnout(tape, report, candidate);
      if (score !== null && (best === null || score > best.score)) best = { start: candidate, score };
    }
  }
  return best;
}
