import { openRadial, type RadialPage } from './radial-menu';
import type { ProjectShelf } from './project-box';

export type ProjectDesk = 'preparation' | 'production';

export function productionCan(trigger: HTMLElement, desk: ProjectDesk, ui: {
  pitch: (series: boolean) => void;
  browse: (shelf: ProjectShelf | null) => void;
  discussion: () => void;
  newsroom: () => void;
  workOrders: () => void;
  resume: () => void;
}) {
  const fiction: RadialPage = { label: 'Fiction', icon: 'film', choices: [
    { label: 'Film', icon: 'film', next: () => ui.pitch(false) },
    { label: 'Series', icon: 'series', next: () => ui.pitch(true) },
  ] };
  const nonfiction: RadialPage = { label: 'Nonfiction', icon: 'news', choices: [
    { label: 'Discussion', icon: 'clapper', next: ui.discussion },
    { label: 'News bulletin', icon: 'news', next: ui.newsroom },
  ] };
  const create: RadialPage = { label: 'Draft new', icon: 'script', choices: [
    { label: 'Fiction', icon: 'film', next: fiction },
    { label: 'Nonfiction', icon: 'news', next: nonfiction },
  ] };
  const ideas: RadialPage = { label: 'Ideas', icon: 'script', choices: [
    { label: 'Draft new', icon: 'script', next: create },
    { label: 'Existing', icon: 'portfolio', next: () => ui.browse('ideas') },
  ] };
  const scripts: RadialPage = { label: 'Scripts', icon: 'script', choices: [
    { label: 'Finished', icon: 'script', next: () => ui.browse('scripts') },
    { label: 'In progress', icon: 'clock', next: () => ui.browse('writing') },
    { label: 'Idle', icon: 'portfolio', next: () => ui.browse('idle') },
  ] };
  const production: RadialPage = { label: 'Production', icon: 'clapper', choices: [
    { label: 'Ready to book', icon: 'calendar', next: () => ui.browse('casting') },
    { label: 'In progress', icon: 'clock', next: () => ui.browse('recording') },
    { label: 'Delivered', icon: 'film', next: () => ui.browse('completed') },
    { label: 'Work orders', icon: 'portfolio', next: ui.workOrders },
  ] };
  const preparation: RadialPage = { label: 'Preparation', icon: 'script', choices: [
    { label: 'Ideas', icon: 'portfolio', next: ideas },
    { label: 'Scripts', icon: 'script', next: scripts },
    { label: 'Production', icon: 'clapper', next: production },
  ] };
  openRadial(trigger, desk === 'preparation' ? preparation : production,
    { label: 'Return to desk papers', caption: 'Desk papers', icon: 'portfolio', run: ui.resume });
}
