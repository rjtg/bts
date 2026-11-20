import { action, node } from './dom';

export type DeskIcon = 'tape' | 'calendar' | 'clapper' | 'script' | 'portfolio' | 'film' | 'series' | 'news' | 'magazine' | 'clock' | 'star' | 'repeat';

const drawings: Record<DeskIcon, string> = {
  tape: 'M3 5h18v14H3z M6 8h12v6H6z M7 19l2-3h6l2 3 M9 11h.1m5.9 0h.1',
  calendar: 'M4 6h16v15H4z M4 10h16 M8 3v5m8-5v5 M8 14h.1m4 0h.1m4 0h.1M8 17h.1m4 0h.1',
  clapper: 'M3 11h18v10H3z M3 11l-1-5 17-4 1 5z M6 5l3 4m3-6 3 4 M7 16h10',
  script: 'M5 3h10l4 4v14H5z M15 3v5h4 M8 12h8m-8 4h8',
  portfolio: 'M3 7h7l2 2h9v11H3z M5 7V4h13v5 M7 14h10',
  film: 'M4 3h16v18H4z M8 3v18m8-18v18 M4 8h4m-4 8h4m8-8h4m-4 8h4',
  series: 'M3 6h12v15H3z M7 3h14v15h-3 M6 10h6m-6 4h6m-6 4h4',
  news: 'M3 5h18v14H3z M6 8h5v5H6z M14 8h4m-4 4h4M6 16h12',
  magazine: 'M4 4h14v17H4z M7 4v17 M10 8h5v6h-5zm0 9h5',
  clock: 'M12 3a9 9 0 1 1 0 18 9 9 0 0 1 0-18z M12 7v5l4 2',
  star: 'M12 3l3 6 7 1-5 5 1 6-6-3-6 3 1-6-5-5 7-1z',
  repeat: 'M4 9a7 7 0 0 1 12-4l4 3m0-5v5h-5M20 15a7 7 0 0 1-12 4l-4-3m0 5v-5h5',
};

function icon(name: DeskIcon) {
  const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
  svg.setAttribute('viewBox', '0 0 24 24');
  svg.setAttribute('aria-hidden', 'true');
  const path = document.createElementNS(svg.namespaceURI, 'path');
  path.setAttribute('d', drawings[name]);
  svg.append(path);
  return svg;
}

export interface RadialChoice {
  label: string;
  icon: DeskIcon;
  next: RadialPage | (() => void);
  disabled?: boolean;
}
export interface RadialPage {
  label: string;
  icon: DeskIcon;
  choices: RadialChoice[];
}

export function openRadial(trigger: HTMLElement, root: RadialPage, alternative?: { label: string; run: () => void }) {
  const dialog = node('dialog', '', 'desk-radial');
  const heading = node('h2', '', 'radial-heading');
  heading.id = 'radial-branch';
  dialog.setAttribute('aria-labelledby', heading.id);
  const ring = node('div', '', 'radial-wheel');
  const controls = node('footer');
  const branch = [root];
  const anchor = trigger.getBoundingClientRect();
  let restore = true;

  function choose(run: () => void) {
    restore = false;
    dialog.close();
    run();
  }

  function position() {
    const bounds = dialog.getBoundingClientRect();
    const clamp = (point: number, size: number, viewport: number) =>
      Math.max(8, Math.min(viewport - size - 8, point - size / 2));
    dialog.style.left = `${clamp(anchor.left + anchor.width / 2, bounds.width, innerWidth)}px`;
    dialog.style.top = `${clamp(anchor.top + anchor.height / 2, bounds.height, innerHeight)}px`;
  }

  function render() {
    const page = branch[branch.length - 1];
    heading.textContent = branch.map(item => item.label).join(' › ');
    const center = node('div', '', 'radial-center');
    center.append(icon(page.icon), node('strong', page.label));
    ring.replaceChildren(center);
    const buttons = page.choices.map((choice, index) => {
      const pick = action(choice.label, () => {
        if (typeof choice.next === 'function') choose(choice.next);
        else { branch.push(choice.next); render(); }
      }, 'radial-choice');
      pick.prepend(icon(choice.icon));
      pick.disabled = !!choice.disabled;
      const angle = index * 2 * Math.PI / page.choices.length;
      pick.style.setProperty('--radial-x', String(Math.sin(angle)));
      pick.style.setProperty('--radial-y', String(-Math.cos(angle)));
      return pick;
    });
    ring.append(...buttons);
    const back = action('Back', () => { branch.pop(); render(); });
    back.hidden = branch.length === 1;
    const cancel = action('Cancel', () => dialog.close());
    controls.replaceChildren(back, cancel);
    if (alternative) controls.prepend(action(alternative.label, () => choose(alternative.run)));
    const available = buttons.filter(button => !button.disabled);
    ring.onkeydown = event => {
      if (!['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Home', 'End'].includes(event.key) || !available.length) return;
      event.preventDefault();
      const current = available.indexOf(document.activeElement as HTMLButtonElement);
      const step = ['ArrowLeft', 'ArrowUp'].includes(event.key) ? -1 : 1;
      const index = event.key === 'Home' ? 0 : event.key === 'End' ? available.length - 1
        : (current + step + available.length) % available.length;
      available[index].focus();
    };
    position();
    (available[0] ?? cancel).focus({ preventScroll: true });
  }

  dialog.append(heading, ring, controls);
  document.body.append(dialog);
  dialog.showModal();
  render();
  window.addEventListener('resize', position);
  dialog.addEventListener('close', () => {
    window.removeEventListener('resize', position);
    dialog.remove();
    if (restore && trigger.isConnected) trigger.focus({ preventScroll: true });
  }, { once: true });
}
