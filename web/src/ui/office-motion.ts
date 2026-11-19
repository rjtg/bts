import { element, node } from './dom';

const sources = '#office-crt, #pull-tapes, #open-notebook, #office [data-prop]';

export function initialiseOfficeMotion() {
  const office = element('office');
  const surface = element('office-blotter');
  const reduced = matchMedia('(prefers-reduced-motion: reduce)');
  let origin: HTMLElement | null = null;
  let pending: Animation | null = null;
  let replaying = false;

  function animate(trigger: HTMLElement, returning: boolean) {
    const from = trigger.getBoundingClientRect();
    const to = surface.getBoundingClientRect();
    if (reduced.matches || !from.width || !to.width) return null;
    const ghost = node('span', '', 'office-prop-in-flight');
    ghost.setAttribute('aria-hidden', 'true');
    const own = getComputedStyle(trigger);
    const artwork = own.getPropertyValue('--prop-active').trim()
      || own.getPropertyValue('--travel-art').trim() || own.backgroundImage;
    ghost.style.backgroundImage = artwork;
    if (!artwork || artwork === 'none') ghost.textContent = trigger.getAttribute('title') || trigger.textContent?.trim() || '';
    const left = to.left + (to.width - from.width) / 2;
    const top = to.top + (to.height - from.height) / 2;
    Object.assign(ghost.style, {
      left: `${from.left}px`, top: `${from.top}px`, width: `${from.width}px`, height: `${from.height}px`,
    });
    document.body.append(ghost);
    const resting = { transform: 'translate(0,0) scale(.98)', opacity: .92 };
    const overDesk = {
      transform: `translate(${left - from.left}px,${top - from.top}px) scale(1.3)`, opacity: 1,
    };
    const animation = ghost.animate(returning ? [overDesk, resting] : [resting, overDesk], {
      duration: returning ? 190 : 280, easing: 'cubic-bezier(.22,.68,.23,1)',
    });
    const remove = () => ghost.remove();
    animation.finished.then(remove, remove);
    return animation;
  }

  office.addEventListener('click', event => {
    if (replaying || !(event.target instanceof Element) || event.ctrlKey || event.shiftKey
      || event.altKey || event.metaKey) return;
    const trigger = event.target.closest<HTMLElement>(sources);
    if (!trigger || trigger.inert) return;
    event.preventDefault();
    event.stopImmediatePropagation();
    if (pending) return;
    const reveal = () => {
      origin = trigger;
      replaying = true;
      try { trigger.click(); } finally { replaying = false; }
    };
    const animation = animate(trigger, false);
    if (!animation) { reveal(); return; }
    pending = animation;
    office.classList.add('prop-travelling');
    animation.finished.then(() => {
      if (pending !== animation) return;
      pending = null;
      office.classList.remove('prop-travelling');
      if (!document.querySelector('#pause-screen[open]')) reveal();
    }, () => {});
  }, { capture: true });

  function cancelFlight() {
    pending?.cancel();
    pending = null;
    office.classList.remove('prop-travelling');
  }

  document.addEventListener('click', event => {
    if (pending && event.target instanceof Node && !office.contains(event.target)) cancelFlight();
  }, { capture: true });

  document.addEventListener('keydown', event => {
    if (event.key !== 'Escape' || !pending) return;
    event.preventDefault();
    event.stopImmediatePropagation();
    cancelFlight();
  }, { capture: true });

  return {
    get launched() { return origin !== null; },
    restore() {
      if (!origin) return;
      const source = origin;
      origin = null;
      if (source.closest('[inert]') || !source.getClientRects().length) return;
      source.focus({ preventScroll: true });
      animate(source, true);
    },
  };
}
