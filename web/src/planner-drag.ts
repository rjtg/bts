import type { Command, Report } from './protocol';
import { element, node } from './ui/dom';

type Drag = { programme: number; airing: number | null; minutes: number };
const TICKS_PER_DAY = 96;

export function initialisePlannerDrag(ui: {
  report: () => Report; player: () => number; send: (command: Command) => void;
  stamp: (tick: number) => string; time: (tick: number) => string; pause: () => void;
}) {
  let active: Drag | null = null;
  let ghost: HTMLElement | null = null;
  let lastStart: number | null = null;

  function inspect(work: Drag, start: number) {
    const report = ui.report();
    const end = start + Math.ceil(work.minutes / 15);
    const conflict = report.schedule.find(a =>
      a.station === ui.player() && a.airing_id !== work.airing && start < a.end_tick && end > a.start_tick);
    const issue = start < report.now_tick ? 'That time has passed.' : end > report.end_tick
      ? 'This does not fit inside the calendar.' : conflict
      ? `Conflicts with ${conflict.programme_name} at ${ui.stamp(conflict.start_tick)}.` : '';
    const status = element('placement-preview');
    const endLabel = Math.floor(start / 96) === Math.floor(end / 96) ? ui.time(end) : ui.stamp(end);
    status.textContent = `Preview · ${ui.stamp(start)}–${endLabel} · ${work.minutes} min runtime. ${issue}`;
    status.hidden = false;
    status.classList.toggle('placement-conflict', Boolean(issue));
    return { end, issue };
  }
  function clear() { ghost?.remove(); ghost = null; lastStart = null; }

  return {
    bindSource(source: HTMLElement, work: Drag) {
      source.draggable = true;
      source.addEventListener('dragstart', event => {
        if (!event.dataTransfer) return;
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
    bindLane(lane: HTMLElement, dayStart: number) {
      const offset = (event: DragEvent) => {
        const bounds = lane.getBoundingClientRect();
        return Math.max(0, Math.min(95, Math.floor((event.clientX - bounds.left) * TICKS_PER_DAY / bounds.width)));
      };
      lane.addEventListener('dragover', event => {
        if (!active) return;
        event.preventDefault();
        const tick = offset(event);
        const start = dayStart + tick;
        if (event.dataTransfer) event.dataTransfer.dropEffect = active.airing === null ? 'copy' : 'move';
        if (start === lastStart) return;
        clear(); lastStart = start;
        const { end, issue } = inspect(active, start);
        ghost = node('span', issue ? 'Conflict' : 'Preview', `drag-ghost${issue ? ' conflict' : ''}`);
        ghost.style.left = `calc(${tick} * var(--tick))`;
        ghost.style.width = `calc(${Math.min(96 - tick, end - start)} * var(--tick))`;
        ghost.setAttribute('aria-hidden', 'true');
        lane.append(ghost);
      });
      lane.addEventListener('dragleave', event => {
        if (!(event.relatedTarget instanceof Node) || !lane.contains(event.relatedTarget)) clear();
      });
      lane.addEventListener('drop', event => {
        if (!active) return;
        event.preventDefault();
        const work = active;
        const start = dayStart + offset(event);
        inspect(work, start);
        clear(); active = null;
        ui.send(work.airing === null
          ? { type: 'place', programme: work.programme, start_tick: start, advertiser: null }
          : { type: 'move', airing_id: work.airing, start_tick: start });
      });
    },
    inspect,
  };
}
