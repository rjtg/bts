import { element } from './dom';

export function initialiseDesks(closeShelf: () => void) {
  const views = Array.from(document.querySelectorAll<HTMLElement>('[data-desk]'));
  const rail = Array.from(document.querySelectorAll<HTMLAnchorElement>('.rail a'));
  const offsets = new Map<string, number>();
  const timeline = element('timeline-scroller');
  let active = '';
  let scheduleOffset = 0;

  history.scrollRestoration = 'manual';

  function show(name: string, focus = true) {
    const next = views.find(view => view.dataset.desk === name) ?? views[0];
    if (next.dataset.desk === active) return;
    if (active === 'planning') {
      closeShelf();
      scheduleOffset = timeline.scrollLeft;
    }
    if (active) offsets.set(active, window.scrollY);
    active = next.dataset.desk!;
    for (const view of views) view.hidden = view !== next;
    for (const link of rail) {
      const current = link.hash === '#' + active;
      link.classList.toggle('active', current || (link.hash === '#planning' && ['weekly-planner', 'ad-sales'].includes(active)));
      if (current) link.setAttribute('aria-current', 'page');
      else link.removeAttribute('aria-current');
    }
    if (active === 'planning') timeline.scrollLeft = scheduleOffset;
    if (focus) {
      const heading = next.querySelector<HTMLElement>('h1,h2');
      if (heading) { heading.tabIndex = -1; heading.focus({ preventScroll: true }); }
    }
    window.scrollTo(0, offsets.get(active) ?? 0);
  }

  function navigate(name: string) {
    if (location.hash !== '#' + name) history.pushState(null, '', '#' + name);
    show(name);
  }

  document.addEventListener('click', event => {
    if (!(event.target instanceof Element) || !(event instanceof MouseEvent)
      || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
    const link = event.target.closest<HTMLAnchorElement>('a[href^="#"]');
    const name = link?.hash.slice(1);
    if (!name || !views.some(view => view.dataset.desk === name)) return;
    event.preventDefault();
    navigate(name);
  });
  window.addEventListener('popstate', () => show(location.hash.slice(1)));
  show(location.hash.slice(1), false);
  return { navigate };
}
