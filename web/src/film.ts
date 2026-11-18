import type { Command, FilmBooking, FilmCard, FilmPerson, Report, Subject } from './protocol';
import { action, element, node, option } from './ui/dom';

const qualities = ['Humour', 'Playfulness', 'Tension', 'Warmth', 'Intimacy', 'Spectacle', 'Pace'];
const presets = {
  'Small-town story': [.4, .35, .35, .75, .7, .12, .45],
  'Big-city thriller': [.2, .2, .93, .32, .48, .95, .86],
};
const phases: Record<FilmCard['phase'], string> = {
  pitch: 'Find a writer', writing: 'Script in progress', script_review: 'Script ready',
  revising: 'Revision in progress', casting: 'Cast and book', booked: 'Filming booked',
  recording: 'On the set', post: 'In the edit', complete: 'Film ready',
};

export function initialiseFilmDesk(ui: {
  send: (command: Command) => void; stamp: (tick: number) => string;
  money: (value: string) => string; showProgramme: (id: number) => void;
}) {
  const pitch = node('form', '', 'film-pitch production-form');
  const title = node('input'); title.required = true; title.maxLength = 60; title.value = 'A Light in the Window';
  const subject = node('select');
  subject.append(option('everyday_life', 'Everyday life'), option('entertainment', 'Entertainment'), option('science', 'Science and invention'));
  const label = (name: string, input: HTMLElement) => { const l = node('label', name); l.append(input); return l; };
  const brief = node('div', '', 'film-qualities');
  const controls = qualities.map(name => {
    const range = node('input'); range.type = 'range'; range.min = '0'; range.max = '1'; range.step = '.01';
    brief.append(label(name, range)); return range;
  });
  const setPreset = (values: number[]) => controls.forEach((c, i) => { c.value = String(values[i]); });
  setPreset(presets['Small-town story']);
  const briefs = node('div', '', 'choice-tiles');
  for (const [name, values] of Object.entries(presets)) briefs.append(action(name, () => setPreset(values)));
  pitch.append(node('h3', 'Pitch a film'), label('Film title', title), label('Subject', subject), briefs, brief, node('button', 'Save free pitch'));
  pitch.addEventListener('submit', event => {
    event.preventDefault();
    ui.send({ type: 'pitch_film', title: title.value, subject: subject.value as Subject, ambition: controls.map(c => c.valueAsNumber) });
  });
  element('film-pitch').replaceChildren(pitch);

  const bookings = new Map<number, FilmBooking>();
  const writing = new Map<number, { writer?: number; focus?: number }>();
  let previousTick = 0;
  const credits = (value: string) => `${ui.money(value)} credits`;
  const projectLink = (film: FilmCard) => action(film.title, () => {
    const project = element(`film-${film.id}`);
    project.scrollIntoView({ block: 'nearest' }); project.focus();
  });
  const portrait = (person: FilmPerson): HTMLElement => {
    const tile = node('span', '', 'film-portrait');
    tile.style.setProperty('--portrait-x', `${((person.id - 101) % 4) * 100 / 3}%`);
    tile.style.setProperty('--portrait-y', person.id <= 104 ? '0%' : '100%');
    tile.setAttribute('aria-hidden', 'true');
    return tile;
  };
  function peopleCards(people: FilmPerson[], active: number, selected: (id: number) => void, revision = false): HTMLElement {
    const cards = node('div', '', 'film-people');
    for (const person of people) {
      const card = action('', () => selected(person.id), 'film-person');
      card.setAttribute('aria-pressed', String(active === person.id));
      card.append(portrait(person), node('strong', person.name),
        node('small', person.specialty),
        node('small', `${credits(revision ? person.revision_fee : person.fee)} · available until ${ui.stamp(person.available_until)}`));
      for (const [name, start, end] of person.commitments) card.append(node('small', `Booked: ${name} · ${ui.stamp(start)}–${ui.stamp(end)}`));
      cards.append(card);
    }
    return cards;
  }
  function render(report: Report) {
    if (report.now_tick < previousTick) { bookings.clear(); writing.clear(); }
    previousTick = report.now_tick;
    const desk = report.film_desk;
    const root = element('film-projects');
    const notices = element('film-notices');
    root.replaceChildren();
    notices.replaceChildren();
    for (const film of desk.projects) {
      const card = node('article', '', 'production-card film-project');
      card.id = `film-${film.id}`; card.tabIndex = -1;
      card.append(node('h3', film.title), node('p', `${phases[film.phase]} · ${credits(film.committed)} committed${film.due !== null ? ` · ${ui.stamp(film.due)}` : ''}`));
      if (film.phase === 'script_review' || film.phase === 'complete') {
        const notice = node('p', film.phase === 'complete' ? '✦ Film ready · ' : '✎ Script ready · ', 'film-notice');
        notice.append(projectLink(film)); notices.append(notice);
      }
      if (film.script) {
        const notes = node('ul', '', 'film-script-notes');
        for (const quality of film.script.qualities) notes.append(node('li', `${quality.name} · ${quality.level}`));
        card.append(node('h4', 'Script notes'), notes,
          node('p', film.script.studio_demand),
          node('p', film.script.weaknesses.length ? `Needs work: ${film.script.weaknesses.map(([,name]) => name).join(', ')}` : 'The script meets the brief.'));
      }
      if (film.phase === 'script_review') card.append(action('Accept script', () => ui.send({ type: 'accept_script', film_id: film.id })));
      if (film.phase === 'pitch' || (film.phase === 'script_review' && !film.revised && film.script?.weaknesses.length)) {
        const revision = film.phase === 'script_review';
        const pool = desk.people.filter(p => p.job === 'writer');
        const select = writing.get(film.id) ?? { writer: pool[0].id, focus: film.script?.weaknesses[0]?.[0] };
        writing.set(film.id, select);
        card.append(node('h4', revision ? 'Commission a revision' : 'Choose a writer'));
        if (revision && film.script) {
          const focus = node('select');
          for (const [id, name] of film.script.weaknesses) focus.append(option(id, name));
          focus.value = String(select.focus);
          focus.addEventListener('change', () => { select.focus = Number(focus.value); });
          card.append(label('What should the rewrite improve?', focus));
        }
        card.append(peopleCards(pool, select.writer ?? 0, id => { select.writer = id; render(report); }, revision));
        const due = report.now_tick + (revision ? desk.revision_ticks : desk.writing_ticks);
        const writer = pool.find(p => p.id === select.writer)!;
        card.append(node('p', `Delivery ${ui.stamp(due)} · ${credits(revision ? writer.revision_fee : writer.fee)}`));
        const commission = action(revision ? 'Pay for revision' : 'Commission script', () => ui.send({
          type: 'write_film', film_id: film.id, writer: writer.id, focus: revision ? select.focus ?? null : null,
        }));
        commission.disabled = due > Math.min(writer.available_until, report.end_tick) || writer.commitments.length > 0;
        card.append(commission);
      }
      if (film.phase === 'writing' || film.phase === 'revising' || ['booked','recording','post'].includes(film.phase)) {
        card.append(action(film.script && film.booking ? 'Advance to film delivery' : 'Advance to script delivery', () => {
          if (film.due !== null) ui.send({ type: 'advance', to_tick: film.due });
        }));
      }
      if (film.phase === 'casting') {
        const actors = desk.people.filter(p => p.job === 'actor');
        const directors = desk.people.filter(p => p.job === 'director');
        const draft = bookings.get(film.id) ?? {
          cast: [actors[0].id, actors[1].id], director: directors[0].id,
          studio: desk.studios[0].id, start: Math.min(report.end_tick - 30, report.now_tick + 8),
        };
        bookings.set(film.id, draft);
        const seats = node('div', '', 'film-seats');
        for (const [index, role] of desk.roles.entries()) {
          const slot = node('section');
          slot.append(node('h4', role), peopleCards(actors, draft.cast[index], id => { draft.cast[index] = id; render(report); }));
          seats.append(slot);
        }
        const direction = node('section');
        direction.append(node('h4', 'Director'), peopleCards(directors, draft.director, id => { draft.director = id; render(report); }));
        seats.append(direction); card.append(seats);
        const studio = node('select');
        for (const room of desk.studios) studio.append(option(room.id, `${room.name} · ${credits(room.fee)}`));
        studio.value = String(draft.studio);
        const room = desk.studios.find(room => room.id === draft.studio)!;
        studio.addEventListener('change', () => { draft.studio = Number(studio.value); render(report); });
        const day = node('select'), time = node('select');
        for (let i = 0; i < 7; i++) day.append(option(i, `Day ${i + 1}`));
        for (let i = 0; i < 96; i++) time.append(option(i, ui.stamp(i).split(' · ')[1]));
        day.value = String(Math.floor(draft.start / 96));
        time.value = String(draft.start % 96);
        const date = () => { draft.start = Number(day.value) * 96 + Number(time.value); render(report); };
        day.addEventListener('change', date); time.addEventListener('change', date);
        card.append(label('Studio', studio), node('p', room.summary));
        for (const [name, start, end] of room.commitments) card.append(node('p', `Reserved: ${name} · ${ui.stamp(start)}–${ui.stamp(end)}`));
        card.append(label('Recording day', day), label('Recording time', time), action('Review film booking', () => ui.send({ type: 'review_film', film_id: film.id, booking: draft })));
        const review = report.film_review;
        if (review?.id === film.id && review.cast.every((id, i) => id === draft.cast[i]) &&
            review.director === draft.director && review.studio === draft.studio && review.start === draft.start) {
          const reviewPanel = node('div', '', 'contract-panel');
          const name = (id: number) => desk.people.find(person => person.id === id)?.name;
          reviewPanel.append(node('h4', 'Ready to film?'),
            node('p', `${name(draft.cast[0])} and ${name(draft.cast[1])} · Directed by ${name(draft.director)} · ${room.name}`),
            node('p', `${credits(review.fees)} talent + ${credits(review.rent)} studio = ${credits(review.total)}`),
            node('p', `Record ${ui.stamp(review.start)}–${ui.stamp(review.recording_end)} · Ready ${ui.stamp(review.ready)}`));
          if (review.studio_shortfall) reviewPanel.append(node('p', 'This room falls short of the script’s staging needs. You can still book it.'));
          reviewPanel.append(action('Confirm film booking', () => ui.send({ type: 'book_film', film_id: film.id, booking: draft })));
          card.append(reviewPanel);
        }
      }
      if (film.programme !== null) card.append(action('Open in programme library', () => ui.showProgramme(film.programme!)));
      root.append(card);
    }
  }
  return { render, showProject: (id: number) => {
    const project = element(`film-${id}`);
    project.scrollIntoView({ block: 'nearest' }); project.focus();
  }};
}
