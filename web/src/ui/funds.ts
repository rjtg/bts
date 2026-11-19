import { node } from './dom';

/** An F with a short cross-stroke through the lower stem. */
export function fundsMark(): HTMLElement {
  const mark = node('span', '', 'funds-mark');
  mark.setAttribute('aria-label', 'Funds');
  mark.setAttribute('role', 'img');
  mark.innerHTML = '<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M8 21V3H21M8 10H17M4 15H11"/></svg>';
  return mark;
}
