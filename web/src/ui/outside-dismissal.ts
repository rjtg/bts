type Dismissible = { surface: HTMLElement; close: () => void };

/** A press must start and end outside the same visible workspace. */
export function bindOutsideDismissal(current: () => Dismissible | null) {
  let outside: HTMLElement | null = null;
  function isOutside(event: Event, surface: HTMLElement): boolean {
    if (!(event.target instanceof Element) || document.querySelector('dialog[open], #station-directory:popover-open')) return false;
    const target = event.target;
    return !surface.contains(target) && !target.closest('.transport, .masthead, #pause-screen');
  }

  document.addEventListener('pointerdown', event => {
    const active = current();
    outside = active && isOutside(event, active.surface) ? active.surface : null;
  }, true);
  document.addEventListener('pointercancel', () => { outside = null; }, true);
  document.addEventListener('click', event => {
    const pressedOutside = outside;
    outside = null;
    const active = current();
    if (!active || active.surface !== pressedOutside || !isOutside(event, active.surface)) return;
    event.preventDefault();
    event.stopImmediatePropagation();
    active.close();
  }, true);
}
