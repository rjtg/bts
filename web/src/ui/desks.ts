import { element, node } from './dom';

const folders: Record<string, { label: string; prop: string }> = {
  planning: { label: 'Programming calendar', prop: 'calendar' },
  'weekly-planner': { label: 'Regular bookings', prop: 'calendar' },
  production: { label: 'Studio workbooks', prop: 'production' },
  'rights-market': { label: 'Incoming portfolio', prop: 'rights' },
  audience: { label: 'Research binder', prop: 'research' },
  results: { label: 'Transmission reviews', prop: 'reviews' },
  finances: { label: 'Station ledger', prop: 'ledger' },
};

export function initialiseDesks(closeShelf: () => void, onChange: (desk: string) => void) {
  const views = Array.from(document.querySelectorAll<HTMLElement>('[data-desk]'));
  const paperMount = element('office-paper-mount');
  const office = element('office');
  const scenery = office.querySelector<HTMLElement>('.office-room')!;
  const paperViews = new Set(['audience', 'production', 'film-dossier', 'results', 'rights-market', 'weekly-planner', 'finances']);
  const rail = Array.from(document.querySelectorAll<HTMLAnchorElement>('.rail a'));
  const offsets = new Map<string, number>();
  const workspaceOffsets = new Map<string, number>();
  const timeline = element('timeline-scroller');
  let active = '';
  let scheduleOffset = 0;

  history.scrollRestoration = 'manual';
  for (const view of views) {
    if (paperViews.has(view.dataset.desk!)) paperMount.append(view);
    const folder = folders[view.dataset.desk ?? ''];
    if (!folder) continue;
    view.classList.add('office-job');
    view.dataset.job = folder.prop;
    const cover = node('header', '', 'office-job-cover');
    const back = view.querySelector<HTMLAnchorElement>('#back-office') ?? node('a', '← Back to Office', 'desk-back');
    back.href = '#office';
    cover.append(node('h2', folder.label), back);
    view.prepend(cover);
  }

  function show(name: string, focus = true) {
    const canonical = ['film-desk', 'film-production'].includes(name) ? 'film-dossier' : name;
    const next = views.find(view => view.dataset.desk === canonical) ?? views[0];
    if (next.dataset.desk === active) return;
    if (active === 'planning') {
      closeShelf();
      scheduleOffset = timeline.scrollLeft;
    }
    if (active) {
      offsets.set(active, window.scrollY);
      const previous = views.find(view => view.dataset.desk === active);
      if (previous) workspaceOffsets.set(active, previous.scrollTop);
    }
    active = next.dataset.desk!;
    const alongsideOffice = active !== 'office';
    const paperOpen = paperViews.has(active);
    paperMount.hidden = !paperOpen;
    office.classList.toggle('paper-open', paperOpen);
    scenery.inert = paperOpen;
    document.body.classList.toggle('prop-workspace', alongsideOffice);
    for (const view of views) {
      const background = view.dataset.desk === 'office' && alongsideOffice;
      view.hidden = view !== next && !background;
      view.inert = background && !paperOpen;
    }
    for (const link of rail) {
      const current = link.hash === '#' + active;
      link.classList.toggle('active', current || (link.hash === '#planning' && active === 'weekly-planner')
        || (link.hash === '#production' && active === 'film-dossier'));
      if (current) link.setAttribute('aria-current', 'page');
      else link.removeAttribute('aria-current');
    }
    onChange(active);
    if (active === 'planning') timeline.scrollLeft = scheduleOffset;
    next.scrollTop = workspaceOffsets.get(active) ?? 0;
    if (focus) {
      const heading = next.querySelector<HTMLElement>('h1,h2');
      if (heading && active !== 'office') { heading.tabIndex = -1; heading.focus({ preventScroll: true }); }
    }
    window.scrollTo(0, offsets.get(active) ?? 0);
  }

  function navigate(name: string) {
    if (location.hash !== '#' + name) history.pushState(null, '', '#' + name);
    show(name);
  }

  document.addEventListener('click', event => {
    if (event.defaultPrevented || !(event.target instanceof Element) || !(event instanceof MouseEvent)
      || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
    const link = event.target.closest<HTMLAnchorElement>('a[href^="#"]');
    const name = link?.hash.slice(1);
    if (!name || !views.some(view => view.dataset.desk === name)) return;
    event.preventDefault();
    link?.closest<HTMLElement>('#station-directory')?.hidePopover();
    navigate(name);
  });
  window.addEventListener('popstate', () => show(location.hash.slice(1)));
  show(location.hash.slice(1), false);
  return { navigate };
}
