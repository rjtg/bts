import { action } from './dom';

/** Switches among existing papers without replacing their contents. */
export function paperIndex<Key>(index: HTMLElement) {
  const entries = new Map<Key, { tab: HTMLButtonElement; paper: HTMLElement }>();

  function show(key: Key, focus = false): boolean {
    const entry = entries.get(key);
    if (!entry) return false;
    for (const { tab, paper } of entries.values()) {
      const current = tab === entry.tab;
      tab.setAttribute('aria-pressed', String(current));
      paper.hidden = !current;
    }
    if (focus) {
      entry.paper.tabIndex = -1;
      entry.paper.focus({ preventScroll: true });
    }
    if (entry.paper instanceof HTMLDetailsElement) entry.paper.open = true;
    return true;
  }

  function add(key: Key, label: string, paper: HTMLElement): HTMLButtonElement {
    const tab = action(label, () => show(key, true), 'binder-tab');
    tab.setAttribute('aria-controls', paper.id);
    index.append(tab);
    entries.set(key, { tab, paper });
    return tab;
  }

  return { add, show };
}
