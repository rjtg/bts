/** A small selection panel anchored beside the active file when space allows. */
export function showDeskPicker(trigger: HTMLElement, panel: HTMLDialogElement, firstFocus: HTMLElement) {
  function position() {
    const anchor = (trigger.closest('.dossier, .office-job, .placement-slip') ?? trigger).getBoundingClientRect();
    const width = panel.offsetWidth;
    const height = panel.offsetHeight;
    const right = anchor.right + 10;
    const left = anchor.left - width - 10;
    const x = right + width + 10 <= innerWidth ? right : left >= 10 ? left : innerWidth - width - 10;
    panel.style.setProperty('--picker-x', `${Math.max(10, x)}px`);
    panel.style.setProperty('--picker-y', `${Math.max(10, Math.min(anchor.top, innerHeight - height - 10))}px`);
  }
  document.body.append(panel);
  panel.showModal();
  position();
  firstFocus.focus({ preventScroll: true });
  window.addEventListener('resize', position);
  panel.addEventListener('close', () => {
    window.removeEventListener('resize', position);
    panel.remove();
    const restored = trigger.id ? document.getElementById(trigger.id) : null;
    (restored ?? trigger).focus({ preventScroll: true });
  }, { once: true });
}
