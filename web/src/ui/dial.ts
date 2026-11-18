import { node } from './dom';

/** A rotary face with native range semantics for keyboard and screen readers. */
export function rotaryDial(input: HTMLInputElement): HTMLElement {
  const dial = node('div', '', 'rotary-dial');
  const face = node('span', '', 'rotary-face');
  face.setAttribute('aria-hidden', 'true');
  const update = () => {
    const maximum = Number(input.max);
    const fraction = maximum > 0 ? input.valueAsNumber / maximum : 0;
    face.style.setProperty('--dial-angle', `${-135 + fraction * 270}deg`);
  };
  const turn = (event: PointerEvent) => {
    const bounds = dial.getBoundingClientRect();
    const x = event.clientX - (bounds.left + bounds.width / 2);
    const y = event.clientY - (bounds.top + bounds.height / 2);
    if (Math.hypot(x, y) < 8) return;
    const angle = Math.atan2(x, -y) * 180 / Math.PI;
    const fraction = Math.max(0, Math.min(1, (angle + 135) / 270));
    input.value = String(Math.round(Number(input.max) * fraction));
    input.dispatchEvent(new Event('input', { bubbles: true }));
  };
  input.addEventListener('pointerdown', event => {
    if (input.disabled || event.button !== 0) return;
    event.preventDefault();
    input.focus();
    input.setPointerCapture(event.pointerId);
    turn(event);
  });
  input.addEventListener('pointermove', event => {
    if (input.hasPointerCapture(event.pointerId)) turn(event);
  });
  input.addEventListener('input', update);
  update();
  dial.append(face, input);
  return dial;
}
