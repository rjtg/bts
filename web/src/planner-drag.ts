import type { Command, Report } from './protocol';
import { element, node } from './ui/dom';

type Drag = { programme: number; airing: number | null; minutes: number; previewAds?: boolean; adMinutes?: number | null };
import { DAY_MINUTES } from './ui/airtime';

export function initialisePlannerDrag(ui: {
  report: () => Report; player: () => number; send: (command: Command) => void;
  stamp: (tick: number) => string; time: (tick: number) => string; pause: () => void;
}) {
  let active: Drag | null = null;
  let ghost: HTMLElement | null = null;
  let lastStart: number | null = null;

  function inspect(work: Drag, start: number, previewOnly = false, statusId = 'placement-preview') {
    const report = ui.report();
    const end = start + work.minutes;
    const conflict = report.schedule.find(a =>
      a.station === ui.player() && a.airing_id !== work.airing && start < a.end_minute && end > a.start_minute);
    const issue = start < report.now_minute ? 'That time has passed.' : end > report.end_tick * 15
      ? 'This does not fit inside the calendar.' : conflict
      ? `Conflicts with ${conflict.programme_name} at ${ui.stamp(conflict.start_minute)}.` : '';
    const status = element(statusId);
    const endLabel = Math.floor(start / DAY_MINUTES) === Math.floor(end / DAY_MINUTES) ? ui.time(end) : ui.stamp(end);
    status.textContent = `Preview · ${ui.stamp(start)}–${endLabel} · ${work.minutes} min runtime. ${previewOnly ? 'Preview only. Review ad minutes in the airing file. ' : ''}${issue}`;
    status.hidden = false;
    status.classList.toggle('placement-conflict', Boolean(issue));
    return { end, issue };
  }
  function clear() { ghost?.remove(); ghost = null; lastStart = null; }

  return {
    bindSource(source: HTMLElement, getWork: () => Drag | null) {
      source.draggable = true;
      source.addEventListener('dragstart', event => {
        const work = getWork();
        if (!event.dataTransfer || !work) { event.preventDefault(); return; }
        ui.pause();
        active = work;
        event.dataTransfer.effectAllowed = work.airing === null ? 'copy' : 'move';
        event.dataTransfer.setData('application/x-bts-airing', String(work.airing ?? work.programme));
      });
      source.addEventListener('dragend', () => {
        active = null;
        clear();
      });
    },
    bindLane(lane: HTMLElement, dayStart: number, compact = false) {
      const offset = (event: DragEvent) => {
        const bounds = lane.getBoundingClientRect();
        const raw = Math.max(0, Math.min(DAY_MINUTES - 1, Math.round((event.clientX - bounds.left) * DAY_MINUTES / bounds.width)));
        const candidates = [Math.round(raw / 15) * 15];
        for (const airing of ui.report().schedule) {
          if (airing.station === ui.player() && airing.airing_id !== active?.airing) {
            candidates.push(airing.start_minute - dayStart, airing.end_minute - dayStart);
          }
        }
        candidates.sort((a, b) => Math.abs(a - raw) - Math.abs(b - raw));
        const nearest = candidates[0];
        return nearest >= 0 && nearest < DAY_MINUTES && Math.abs(nearest - raw) <= 2 ? nearest : raw;
      };
      lane.addEventListener('dragover', event => {
        if (!active || (compact && active.airing !== null)) return;
        event.preventDefault();
        const tick = offset(event);
        const start = dayStart + tick;
        if (event.dataTransfer) event.dataTransfer.dropEffect = active.airing === null ? 'copy' : 'move';
        if (start === lastStart) return;
        clear(); lastStart = start;
        const { end, issue } = inspect(active, start, active.previewAds, compact ? 'today-preview' : 'placement-preview');
        ghost = node('span', issue ? 'Conflict' : 'Preview', `drag-ghost${issue ? ' conflict' : ''}`);
        ghost.style.left = compact ? `${100 * tick / DAY_MINUTES}%` : `calc(${tick / 15} * var(--tick))`;
        const width = Math.min(DAY_MINUTES - tick, end - start);
        ghost.style.width = compact ? `${100 * width / DAY_MINUTES}%` : `calc(${width / 15} * var(--tick))`;
        ghost.setAttribute('aria-hidden', 'true');
        lane.append(ghost);
      });
      lane.addEventListener('dragleave', event => {
        if (!(event.relatedTarget instanceof Node) || !lane.contains(event.relatedTarget)) {
          clear();
          if (compact) element('today-preview').hidden = true;
        }
      });
      lane.addEventListener('drop', event => {
        if (!active || (compact && active.airing !== null)) return;
        event.preventDefault();
        const work = active;
        const start = dayStart + offset(event);
        inspect(work, start, work.previewAds, compact ? 'today-preview' : 'placement-preview');
        clear(); active = null;
        if (compact) element('today-preview').hidden = true;
        ui.send(work.airing === null
          ? { type: 'place_exact', programme: work.programme, start_minute: start, ad_minutes: work.adMinutes ?? null }
          : { type: 'move_exact', airing_id: work.airing, start_minute: start });
      });
    },
    inspect,
  };
}
