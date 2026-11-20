import { action, node } from './dom';

export type FileType = 'Tape' | 'Airing' | 'Idea' | 'Preparation' | 'Production' | 'Character' | 'Market' | 'Research' | 'Ledger';

export function leaf(title?: string) {
  const paper = node('section', '', 'office-leaf');
  if (title) paper.append(node('h3', title));
  return paper;
}

export function spread(left: HTMLElement, right: HTMLElement) {
  const sheets = node('div', '', 'office-spread');
  sheets.append(left, right);
  return sheets;
}

export function dossier(kind: FileType, papers: { label: string; content: HTMLElement }[],
  options: { selected?: string; close?: () => void; turn?: (label: string) => void } = {}) {
  const file = node('section', '', 'dossier');
  file.dataset.file = kind.toLowerCase();
  file.setAttribute('aria-label', `${kind} dossier`);
  const heading = node('header', '', 'dossier-cover');
  heading.append(node('strong', `${kind} file`));
  if (options.close) heading.append(action('Close file', options.close, 'dossier-close'));
  const tabs = node('div', '', 'desk-paper-tabs');
  tabs.setAttribute('role', 'group');
  tabs.setAttribute('aria-label', 'Dossier papers');
  const paper = node('div', '', 'station-paper');
  const footer = node('footer', '', 'dossier-turns');
  const pageNumber = node('span');
  pageNumber.setAttribute('aria-live', 'polite');
  let active = 0;
  const previous = action('', () => turn(active - 1), 'page-corner page-corner-left');
  const next = action('', () => turn(active + 1), 'page-corner page-corner-right');
  previous.setAttribute('aria-label', 'Previous page');
  next.setAttribute('aria-label', 'Next page');
  footer.append(previous, pageNumber, next);
  function turn(index: number) {
    if (index < 0 || index >= papers.length) return;
    const focused = document.activeElement;
    selectPaper(index);
    options.turn?.(papers[index].label);
    if (focused === previous && previous.disabled) next.focus();
    if (focused === next && next.disabled) previous.focus();
  }
  const buttons = papers.map(({ label }, index) => action(label, () => turn(index)));
  function selectPaper(index: number) {
    active = index;
    previous.disabled = index === 0;
    next.disabled = index === papers.length - 1;
    pageNumber.textContent = `${index + 1} / ${papers.length} · ${papers[index].label}`;
    paper.replaceChildren(papers[index].content);
    paper.scrollTop = 0;
    buttons.forEach((button, position) => button.setAttribute('aria-pressed', String(index === position)));
  }
  tabs.append(...buttons);
  tabs.hidden = papers.length < 2;
  selectPaper(Math.max(0, papers.findIndex(({ label }) => label === options.selected)));
  file.append(heading, tabs, paper);
  if (options.turn && papers.length > 1) { file.classList.add('paged-dossier'); file.append(footer); }
  return file;
}
