import type { Programme, Report } from '../protocol';
import { dossier, leaf, spread } from './dossier';
import { action, element, node } from './dom';
import { personPortrait } from './portraits';

const creative = ['Humour', 'Playfulness', 'Tension', 'Warmth', 'Intimacy', 'Spectacle', 'Pace'];
const editorial = ['Information depth', 'Topical urgency', 'Editorial rigour'];
const bands: Record<string, number> = { Low: .2, Moderate: .5, High: .8, Serious: .2, Balanced: .5, Playful: .8 };

export function matchesProductionType(tape: Programme, filter: string): boolean {
  if (!filter || tape.production_type === filter) return true;
  if (tape.production_type === 'Fiction') {
    return filter === (tape.series_name === null ? 'Film' : 'Series');
  }
  return filter === (tape.genre === 'News' ? 'News' : 'Magazine');
}

export function programmeChart(tape: Programme) {
  const panel = node('figure', '', 'programme-chart');
  const qualities = new Map(tape.qualities.map(quality => [quality.name, quality.level]));
  const axes = tape.production_type === 'Fiction' ? creative : [...creative, ...editorial];
  const values = axes.map(name => bands[qualities.get(name) ?? '']);
  panel.append(radar(axes, values, 'Programme quality radar, based on published quality bands'));
  const legend = node('figcaption', '', 'sr-only');
  legend.textContent = [...creative, ...editorial].map(name =>
    `${name}: ${qualities.get(name) ?? (axes.includes(name) ? 'Unknown' : 'Inapplicable')}`).join('. ');
  panel.append(legend);
  const absent = [...(tape.production_type === 'Fiction' ? ['Editorial qualities do not apply to fiction.'] : []),
    ...axes.filter((_, index) => values[index] === undefined).map(name => `${name} unknown.`)];
  if (absent.length) panel.append(node('p', absent.join(' '), 'paper-footnote'));
  return panel;
}

export function radar(axes: string[], values: (number | undefined)[], description: string) {
  const point = (index: number, radius: number) => {
    const angle = index * 2 * Math.PI / axes.length - Math.PI / 2;
    return [155 + Math.cos(angle) * radius, 125 + Math.sin(angle) * radius];
  };
  const outline = (values: number[]) => values.map((value, index) => point(index, value * 72).join(',')).join(' ');
  const chart = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
  chart.setAttribute('viewBox', '0 0 310 248');
  chart.setAttribute('role', 'img');
  chart.setAttribute('aria-label', description);
  const polygon = (points: string, kind: string) => {
    const mark = document.createElementNS(chart.namespaceURI, 'polygon');
    mark.setAttribute('points', points);
    mark.setAttribute('class', kind);
    chart.append(mark);
  };
  for (const ring of [.34, .67, 1]) polygon(outline(axes.map(() => ring)), 'radar-grid');
  for (const [index, name] of axes.entries()) {
    const [x, y] = point(index, 72);
    const line = document.createElementNS(chart.namespaceURI, 'line');
    for (const [key, value] of Object.entries({ x1:155, y1:125, x2:x, y2:y })) line.setAttribute(key, String(value));
    line.setAttribute('class', 'radar-grid');
    chart.append(line);
    const [tx, ty] = point(index, 90);
    const label = document.createElementNS(chart.namespaceURI, 'text');
    label.setAttribute('x', String(tx));
    label.setAttribute('y', String(ty));
    label.setAttribute('text-anchor', tx < 120 ? 'end' : tx > 188 ? 'start' : 'middle');
    label.textContent = name === 'Information depth' ? 'Depth' : name === 'Topical urgency' ? 'Urgency' : name === 'Editorial rigour' ? 'Rigour' : name;
    chart.append(label);
  }
  if (values.every(value => value !== undefined)) {
    const known = values as number[];
    polygon(outline(known), 'radar-profile');
    polygon(outline(known.map((value, index) => Math.min(1, value * (index % 2 ? .975 : 1.025)))), 'radar-pencil');
  }
  else {
    for (const [index, value] of values.entries()) {
      if (value === undefined) continue;
      const [cx, cy] = point(index, value * 72);
      const marker = document.createElementNS(chart.namespaceURI, 'circle');
      for (const [key, amount] of Object.entries({ cx, cy, r:3 })) marker.setAttribute(key, String(amount));
      marker.setAttribute('class', 'radar-profile');
      chart.append(marker);
    }
  }
  return chart;
}

export function programmeCredits(tape: Programme, report: Report, stamp?: (minute: number) => string) {
  const credits = node('section', '', 'programme-credits');
  credits.append(node('h4', 'People on this programme'));
  const list = node('div', '', 'credit-clippings');
  credits.append(list);
  function entry(role: string, name: string, description: string, portrait?: HTMLElement | null,
    obligations: string[] = []) {
    const pick = action(`${role} · ${name}`, () => {
      const dialog = element<HTMLDialogElement>('character-dossier');
      const bio = leaf(name);
      if (portrait) bio.append(portrait.cloneNode(true));
      bio.append(node('p', role), node('p', description));
      const diary = leaf('Known availability and commitments');
      for (const line of obligations) diary.append(node('p', line));
      if (!obligations.length) diary.append(node('p', 'No commitments on file.'));
      dialog.replaceChildren(dossier('Character', [{ label: 'Public file', content: spread(bio, diary) }],
        { close: () => dialog.close() }));
      dialog.showModal();
    });
    if (portrait) pick.prepend(portrait);
    list.append(pick);
  }
  const bulletin = report.newsroom?.editions.find(edition => edition.programme === tape.id);
  if (bulletin) {
    for (const [role, id] of [['Reporter', bulletin.reporter], ['Anchor', bulletin.anchor]] as const) {
      const person = report.production.people.find(person => person.id === id);
      if (person) entry(role, person.name, person.description, personPortrait(id, person.name));
    }
  }
  const discussion = report.production.projects.find(project => project.programme === tape.id);
  if (discussion) {
    for (const [role, id] of [['Host', discussion.host] as const, ...discussion.guests.map(id => ['Guest', id] as const)]) {
      const person = report.production.people.find(person => person.id === id);
      if (person) entry(role, person.name, person.description, personPortrait(id, person.name), stamp ? [
        `Public availability through ${stamp(person.normal_until_tick * 15)}`,
        ...person.commitments.map(item => `${item.title} · ${stamp(item.start_tick * 15)}–${stamp(item.end_tick * 15)}`),
      ] : []);
    }
  }
  const film = report.film_desk.projects.find(project => project.programme === tape.id);
  if (film) {
    const roles: (readonly [string, number])[] = [];
    if (film.booking) {
      roles.push(...film.booking.cast.map((id, index) => [report.film_desk.roles[index], id] as const), ['Director', film.booking.director]);
    }
    if (film.writer !== null) roles.push(['Writer', film.writer]);
    for (const [role, id] of roles) {
      const person = report.film_desk.people.find(candidate => candidate.id === id);
      if (!person) continue;
      const portrait = node('span', '', 'film-portrait');
      portrait.style.setProperty('--portrait-x', `${((id - 101) % 4) * 100 / 3}%`);
      portrait.style.setProperty('--portrait-y', id <= 104 ? '0%' : '100%');
      portrait.setAttribute('aria-hidden', 'true');
      entry(role, person.name, person.specialty, portrait, stamp ? [
        `Public availability through ${stamp(person.available_until * 15)}`,
        ...person.commitments.map(([title, start, end]) => `${title} · ${stamp(start * 15)}–${stamp(end * 15)}`),
      ] : []);
    }
  }
  if (!list.childElementCount) list.append(node('p', 'The station has no cast list on file. Director and writer are unknown.'));
  return credits;
}
