import { action, element } from './dom';

export type StudioSheet = 'concept' | 'jobs' | 'requests' | 'hiring' | 'newsroom' | 'facilities';

const sheets = {
  concept: ['Discussion brief', 'production-form'],
  jobs: ['Work orders', 'studio-jobs'],
  requests: ['Sway requests', 'sway-panel'],
  hiring: ['Hiring', 'hiring-desk'],
  newsroom: ['Newsroom', 'newsroom-board'],
  facilities: ['Facilities', 'station-teams'],
} satisfies Record<StudioSheet, [string, string]>;

export function initialiseStudioBook() {
  const index = element('studio-index');
  const entries = Object.entries(sheets) as [StudioSheet, [string, string]][];
  const tabs = new Map<StudioSheet, HTMLButtonElement>();
  function show(key: StudioSheet, focus = false) {
    for (const [id, [, target]] of entries) {
      const page = element(target);
      const current = id === key;
      page.hidden = !current;
      tabs.get(id)?.setAttribute('aria-pressed', String(current));
      if (current && focus) {
        page.tabIndex = -1;
        page.focus({ preventScroll: true });
      }
    }
    const page = element(sheets[key][1]);
    if (page instanceof HTMLDetailsElement) page.open = true;
  }
  for (const [id, [label, target]] of entries) {
    const tab = action(label, () => show(id, true), 'binder-tab');
    tab.setAttribute('aria-controls', target);
    index.append(tab);
    tabs.set(id, tab);
  }
  show('concept');
  return { show };
}
