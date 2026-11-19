import type { Programme, Report } from '../protocol';

export function surveyEvidence(tape: Programme, report: Report) {
  const supported = tape.qualities.filter(quality => ['Moderate', 'High', 'Playful', 'Balanced'].includes(quality.level));
  if (!supported.length) return [];
  return (report.research?.groups ?? []).map(group => {
    const scores = supported.map(quality => group.estimate.tastes[quality.name]).filter(value => value !== undefined);
    return { ...group, fit: scores.length ? scores.reduce((sum, rating) => sum + rating, 0) / scores.length : 0 };
  }).filter(group => group.fit >= 6).sort((a, b) => b.fit - a.fit || a.id - b.id);
}

export function assistantNote(tape: Programme, report: Report, gapMinutes: number, adMinutes: number,
  startTick: number, historyCount: number, coveredEveningCosts: boolean): string {
  const reserved = Math.ceil((tape.duration_minutes + adMinutes) / 15) * 15;
  const cheaperThanMost = report.catalogue.filter(other => BigInt(other.broadcast_cost) > BigInt(tape.broadcast_cost)).length
    > report.catalogue.length / 2;
  if (gapMinutes > 0 && reserved <= gapMinutes && cheaperThanMost) {
    return `Good gap-filler here. Holds ${reserved} of ${gapMinutes} minutes and costs less to air than most of your tapes.`;
  }
  if (coveredEveningCosts) return 'Worth another evening slot. Its last two evening showings covered their airing costs.';
  const audience = surveyEvidence(tape, report).find(group => group.estimate.watch_time_note?.endsWith('look promising.'));
  if (audience) {
    const promising = audience.estimate.watch_time_note!;
    const weekend = ((report.first_weekday + Math.floor(startTick / 96)) % 7) >= 5;
    const hour = Math.floor(startTick % 96 / 4);
    const part = hour < 12 ? 'mornings' : hour < 18 ? 'afternoons' : 'evenings';
    const current = `${weekend ? 'Weekend' : 'Weekday'} ${part}`;
    return promising.startsWith(current)
      ? `This slot looks promising for ${audience.name.toLowerCase()}, going by the latest survey.`
      : `Try ${promising.replace(' look promising.', '').toLowerCase()} if you’re aiming for ${audience.name.toLowerCase()}.`;
  }
  if (historyCount === 1) return 'We’ve only aired this once. See the ledger before settling on a regular slot.';
  if (historyCount > 1) return 'Check the ledger for repeat performance before giving this a regular slot.';
  return 'We haven’t aired this yet. Treat the first showing as a trial.';
}
