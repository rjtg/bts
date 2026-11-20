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

const watchBands = ['mornings', 'afternoons', 'evenings'];
const watchHours = [9, 15, 20];
function daypart(report: Report, minute: number) {
  const day = (report.first_weekday + Math.floor(minute / DAY_MINUTES)) % 7;
  const hour = Math.floor(minute % DAY_MINUTES / 60);
  return (day >= 5 ? 3 : 0) + (hour < 12 ? 0 : hour < 18 ? 1 : 2);
}

function bandName(index: number) {
  return `${index >= 3 ? 'Weekend' : 'Weekday'} ${watchBands[index % 3]}`;
}

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
  const watchTime = bandName(daypart(report, minute));
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

/** Relative daypart guidance only. Rust owns placement and advertising forecasts. */
export function daypartOutlook(tape: Programme, report: Report) {
  const evidence = surveyEvidence(tape, report);
  if (!evidence.some(group => group.estimate.watch_time_note?.endsWith('look promising.'))) return null;
  const scores = Array.from({ length: 6 }, (_, index) => {
    const day = ((index >= 3 ? 5 : 0) - report.first_weekday + 7) % 7;
    const sample = day * DAY_MINUTES + watchHours[index % 3] * 60;
    return turnout(tape, report, sample) ?? 0;
  });
  const order = scores.map((_, index) => index).sort((a, b) => scores[b] - scores[a] || a - b);
  const strongest = scores[order[0]];
  if (strongest <= 0 || strongest === scores[order[5]]) return null;
  const alternate = order.slice(1).find(index => scores[index] > scores[order[5]]);
  return {
    best: bandName(order[0]),
    also: alternate === undefined ? null : bandName(alternate),
    surveyed: Math.min(...evidence.map(group => group.estimate.tick)) * 15,
    fit(minute: number): 'Good' | 'OK' | 'Weak' {
      const relative = scores[daypart(report, minute)] / strongest;
      return relative >= .9 ? 'Good' : relative >= .75 ? 'OK' : 'Weak';
    },
  };
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
