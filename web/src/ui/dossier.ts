import { action, node } from './dom';

export type FileType = 'Tape' | 'Airing' | 'Preparation' | 'Production' | 'Character';

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
  options: { selected?: string; close?: () => void } = {}) {
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
  const buttons = papers.map(({ label }, index) => action(label, () => selectPaper(index)));
  function selectPaper(index: number) {
    paper.replaceChildren(papers[index].content);
    paper.scrollTop = 0;
    buttons.forEach((button, position) => button.setAttribute('aria-pressed', String(index === position)));
  }
  tabs.append(...buttons);
  tabs.hidden = papers.length < 2;
  selectPaper(Math.max(0, papers.findIndex(({ label }) => label === options.selected)));
  file.append(heading, tabs, paper);
  return file;
}
