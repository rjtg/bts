import { element } from './dom';

const poseSelectors = [
  '.office-idle-staff', '.studio-tally', '.office-idle-standing', '.office-idle-reader',
  '.office-idle-colleague', '.studio-idle-camera', '.studio-idle-host', '.studio-idle-guest',
  '.studio-idle-second-camera', '.office-lamp-glow',
  '[data-quiet-pose]',
];

const activeProp = ':is(#pull-tapes,[data-prop]):is(:hover,:focus-visible),.office-phone:hover';

export function initialiseOfficeIdle() {
  const office = element('office');
  const reduced = matchMedia('(prefers-reduced-motion: reduce)');
  const plant = office.querySelector<HTMLElement>('.office-plant')!;
  const poses = Array.from(office.querySelectorAll<HTMLElement>(poseSelectors.join(',')));
  const subjects = [...poses, plant];
  let remaining: HTMLElement[] = [];
  let previous: HTMLElement | undefined;

  function enabled() {
    const setting = office.dataset.ambientMotion;
    return setting === 'on' || (setting !== 'off' && !reduced.matches);
  }

  async function tick() {
    if (enabled() && !document.hidden && office.getClientRects().length
      && !office.matches('[inert],.prop-travelling,.file-open,.tray-open')
      && !document.querySelector('dialog[open]') && !office.querySelector(activeProp)) {
      const visible = subjects.filter(subject => getComputedStyle(subject).display !== 'none');
      remaining = remaining.filter(subject => visible.includes(subject));
      if (!remaining.length) remaining = visible;
      const available = remaining.length > 1 ? remaining.filter(subject => subject !== previous) : remaining;
      const subject = available[Math.floor(Math.random() * available.length)];
      if (subject) {
        remaining.splice(remaining.indexOf(subject), 1);
        previous = subject;
        office.dataset.idleSubject = subject.dataset.quietPose ?? subject.className;
        if (subject === plant) {
          plant.classList.remove('rustling');
          void plant.offsetWidth;
          plant.classList.add('rustling');
        } else {
          const count = Number(subject.dataset.poseCount ?? 2);
          const current = Number(subject.dataset.pose ?? 0);
          const step = count > 2 ? 1 + Math.floor(Math.random() * (count - 1)) : 1;
          const next = (current + step) % count;
          subject.dataset.pose = String(next);
        }
        const movements = subject.getAnimations({ subtree: true });
        await Promise.allSettled(movements.map(movement => movement.finished));
        window.setTimeout(tick, 40 + Math.floor(Math.random() * 81));
        return;
      }
    }
    window.setTimeout(tick, 200);
  }

  window.setTimeout(tick, 600);
}
