import { element, node } from './dom';

const people = [
  ['.office-idle-staff', 'right'], ['.office-idle-reader', 'left'],
  ['.office-idle-standing', 'right'], ['.office-idle-colleague', 'left'],
  ['[data-quiet-pose="left-seat"]', 'right'], ['[data-quiet-pose="leaning"]', 'left'],
  ['[data-quiet-pose="through-glass"]', 'right'], ['[data-quiet-pose="floor-assistant"]', 'left'],
  ['.studio-idle-camera', 'right'], ['.studio-idle-second-camera', 'left'],
  ['.studio-idle-host', 'right'], ['.studio-idle-guest', 'left'],
] as const;

const deskWork = '[inert],.prop-travelling,.file-open,.tray-open';
const reduced = matchMedia('(prefers-reduced-motion: reduce)');

export function roomMotionEnabled(office: HTMLElement) {
  const setting = office.dataset.ambientMotion;
  return !document.hidden && (setting === 'on' || (setting !== 'off' && !reduced.matches))
    && !document.querySelector('dialog[open]');
}

export function initialiseOfficeIdle() {
  const office = element('office');
  const room = office.querySelector<HTMLElement>('.office-room')!;
  const plant = office.querySelector<HTMLElement>('.office-plant')!;
  const glow = office.querySelector<HTMLElement>('.office-lamp-glow')!;
  const actors = people.map(([selector, direction]) => {
    const sprite = room.querySelector<HTMLElement>(selector)!;
    sprite.style.setProperty('--alternate-image', getComputedStyle(sprite).backgroundImage);
    sprite.dataset.idleActor = direction;
    sprite.dataset.poseLarge = '0';
    sprite.dataset.poseMedium = '0';
    sprite.dataset.poseSmall = '0';
    const base = node('i', '', 'idle-base'), alternate = node('i', '', 'idle-alternate');
    sprite.append(base, alternate);
    return { sprite, base, due: performance.now() + 500 + Math.random() * 4400 };
  });

  // The resting pose is registered to the original scene at every room size.
  const align = () => {
    const scene = room.getBoundingClientRect();
    for (const { sprite, base } of actors) {
      const bounds = sprite.getBoundingClientRect();
      base.style.backgroundSize = `${scene.width}px ${scene.height}px`;
      base.style.backgroundPosition = `${scene.left - bounds.left}px ${scene.top - bounds.top}px`;
    }
  };
  new ResizeObserver(align).observe(room);
  align();

  let nextDetail = 1400;
  window.setInterval(() => {
    if (!roomMotionEnabled(office) || !room.getClientRects().length || office.matches(deskWork)) return;

    const now = performance.now();
    for (const actor of actors) {
      if (actor.due > now || !actor.sprite.getClientRects().length) continue;
      actor.due = now + 1250 + Math.random() * 6300;
      const draw = Math.random();
      const tier = draw < .07 ? 'poseLarge' : draw < .28 ? 'poseMedium' : 'poseSmall';
      const data = actor.sprite.dataset;
      data[tier] = data[tier] === '1' ? '0' : '1';
    }
    if (now >= nextDetail) {
      nextDetail = now + 3200 + Math.random() * 8000;
      plant.classList.remove('rustling');
      void plant.offsetWidth;
      plant.classList.add('rustling');
      glow.dataset.pose = glow.dataset.pose === '1' ? '0' : '1';
    }
  }, 160);
}
