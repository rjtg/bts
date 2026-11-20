import type { Command, FilmBooking, FilmBookingPreview, FilmCard, FilmPerson, PitchPace, Report, Subject } from './protocol';
import { dossier, leaf, spread } from './ui/dossier';
import { action, element, node, option } from './ui/dom';
import { filmPhaseLabels, projectBox, type ProjectShelf } from './ui/project-box';
import { photoSpot, studioSpot } from './ui/casting-gallery';
import { bindDateClock } from './ui/date-clock';

type FilmDraft = Omit<FilmBooking, 'cast' | 'director'> & { cast: [number | null, number | null]; director: number | null };

const qualities = ['Humour', 'Playfulness', 'Tension', 'Warmth', 'Intimacy', 'Spectacle', 'Pace'];
const startingRatings = [3, 3, 2, 4, 4, 2, 3];
const preparing = (film: FilmCard) => ['developing_idea', 'pitch', 'writing', 'script_review', 'revising'].includes(film.phase);

export function initialiseFilmDesk(ui: {
  send: (command: Command) => void; stamp: (tick: number) => string;
  money: (value: string) => string; showProgramme: (id: number) => void;
  previewBooking: (id: number, booking: FilmBooking) => FilmBookingPreview;
  close: () => void;
}) {
  const pitch = node('form', '', 'film-pitch production-form');
  const title = node('input'); title.required = true; title.maxLength = 60;
  const subject = node('select');
  subject.append(option('everyday_life', 'Everyday life'), option('entertainment', 'Entertainment'), option('science', 'Science and invention'));
  const label = (name: string, input: HTMLElement) => { const l = node('label', name); l.append(input); return l; };
  const description = node('textarea'); description.required = true; description.rows = 4; description.maxLength = 800;
  const descriptionLabel = label('Description', description);
  const subjectLabel = label('Subject', subject);
  const brief = node('div', '', 'pitch-ambitions');
  const levels = [...startingRatings];
  const targets = () => levels.map(stars => (stars - 1) / 4);
  for (const [index, name] of qualities.entries()) {
    const group = node('fieldset', '', 'pitch-rating');
    group.append(node('legend', name));
    const stars = node('div', '', 'pitch-stars');
    for (let value = 1; value <= 5; value++) {
      const radio = node('input'); radio.type = 'radio'; radio.name = `pitch-quality-${index}`;
      radio.value = String(value); radio.checked = value === levels[index];
      radio.setAttribute('aria-label', `${name}, ${value} ${value === 1 ? 'star' : 'stars'}`);
      radio.addEventListener('change', () => { levels[index] = value; });
      const choice = label('', radio);
      const glyph = node('span', '★'); glyph.setAttribute('aria-hidden', 'true');
      choice.append(glyph); stars.append(choice);
    }
    group.append(stars); brief.append(group);
  }
  const pace = node('fieldset', '', 'pitch-pace');
  pace.append(node('legend', 'Development pace'));
  for (const [value, name] of [['rush', 'Rush it'], ['steady', 'Steady'], ['take_time', 'Take your time']]) {
    const input = node('input'); input.type = 'radio'; input.name = 'pitch-pace'; input.value = value; input.checked = value === 'steady';
    pace.append(label(name, input));
  }
  const heading = node('h3', 'Pitch a film');
  const savePitch = node('button', 'Develop this idea');
  pitch.append(heading, label('Film title', title), descriptionLabel, subjectLabel);
  const serial = node('section');
  serial.append(node('h4', 'Three-episode series'),
    node('p', 'Use the stars as the standing brief. Set the tension in each script. Large changes demand more from the crew.'));
  const curve = node('div', '', 'film-qualities');
  const tension = [.45, .60, .75].map((target, index) => {
    const control = node('input');
    control.type = 'range'; control.min = '0'; control.max = '1'; control.step = '.01';
    control.value = String(target);
    const name = `Episode ${index + 1} tension`;
    control.setAttribute('aria-label', name);
    const output = node('output', `${Math.round(target * 100)}%`);
    control.addEventListener('input', () => { output.value = `${Math.round(control.valueAsNumber * 100)}%`; });
    const wrapper = label(name, control);
    wrapper.append(output);
    curve.append(wrapper);
    return control;
  });
  serial.append(curve);
  serial.hidden = true;
  pitch.append(serial, pace, savePitch);
  pitch.addEventListener('submit', event => {
    event.preventDefault();
    const ambition = targets();
    if (serial.hidden) ui.send({ type: 'develop_film_idea', title: title.value, description: description.value,
      ambition, pace: new FormData(pitch).get('pitch-pace') as PitchPace });
    else ui.send({ type: 'pitch_scripted_run', title: title.value, subject: subject.value as Subject,
      standing: ambition, tension: tension.map(control => control.valueAsNumber) });
  });
  const pitchMount = element('film-pitch');
  const idea = leaf('What are we aiming for?');
  idea.append(node('p', 'A quiet story that stays with you, or a Saturday night that shakes the room?', 'handwritten'),
    brief);
  const briefPaper = leaf();
  briefPaper.append(pitch);
  pitchMount.replaceChildren(dossier('Idea', [{ label: 'Brief', content: spread(idea, briefPaper) }], { close: ui.close }));

  const bookings = new Map<number, FilmDraft>();
  const writing = new Map<number, { writer?: number; focus?: number }>();
  let previousTick = 0;
  let selectedProject: number | null = null;
  let view: 'pitch' | 'box' | 'project' = 'box';
  let filter: ProjectShelf | null = null;
  let series: number | null = null;
  let report: Report | undefined;
  const pages = new Map<string, string>();
  const funds = (value: string) => `${ui.money(value)} Funds`;
  function chooseView(next: typeof view, focus = true) {
    view = next;
    if (!report) return;
    render(report);
    if (!focus) return;
    const root = element('film-projects');
    const target = next === 'pitch' ? pitch.querySelector('input')
      : next === 'box' ? root.querySelector<HTMLElement>('[aria-current="true"]') ?? root.querySelector('button')
      : root.querySelector<HTMLElement>('.film-project');
    target?.focus({ preventScroll: true });
  }
  function open(id: number) {
    selectedProject = id;
    chooseView('project');
  }
  const portrait = (person: FilmPerson): HTMLElement => {
    const tile = node('span', '', 'film-portrait');
    tile.style.setProperty('--portrait-x', `${((person.id - 101) % 4) * 100 / 3}%`);
    tile.style.setProperty('--portrait-y', person.id <= 104 ? '0%' : '100%');
    tile.setAttribute('aria-hidden', 'true');
    return tile;
  };
  function candidate(person: FilmPerson, revision = false, blocked?: string) {
    return { id: person.id, name: person.name, portrait: () => portrait(person), description: person.specialty,
      fee: funds(revision ? person.revision_fee : person.fee), unavailable: !!blocked,
      notes: [blocked ?? `Available until ${ui.stamp(person.available_until)}`,
        ...person.commitments.map(([name, start, end]) => `Booked: ${name} · ${ui.stamp(start)}–${ui.stamp(end)}`)] };
  }
  function render(next: Report) {
    if (next.now_tick < previousTick) { bookings.clear(); writing.clear(); resetView(); }
    report = next;
    previousTick = next.now_tick;
    const desk = next.film_desk;
    pitchMount.hidden = view !== 'pitch';
    const root = element('film-projects');
    root.hidden = view === 'pitch';
    if (view === 'pitch') { root.replaceChildren(); return; }
    const film = desk.projects.find(project => project.id === selectedProject);
    if (view === 'box' || !film) {
      root.replaceChildren(projectBox(desk, filter, series, selectedProject, {
        open, newPitch, close: ui.close, stamp: ui.stamp, browseSeries: id => { series = id; chooseView('box'); },
      }));
      return;
    }
    const run = desk.scripted_runs.find(item => item.series === film.episode?.series);
    const mode = preparing(film) ? 'preparation' : 'production';
      const card = node('article', '', 'production-card film-project');
      card.id = `film-${film.id}`; card.tabIndex = -1;
      const readyIdea = film.phase === 'pitch' && film.description !== null && filter === 'ideas';
      const summary = leaf(film.title);
      summary.append(node('p', `${readyIdea ? 'Idea ready' : filmPhaseLabels[film.phase]} · ${funds(film.committed)} committed${film.due !== null ? ` · ${ui.stamp(film.due)}` : ''}`));
      if (film.description) summary.append(node('p', film.description, 'film-logline'));
      if (film.episode) {
        const episode = film.episode;
        summary.append(node('p', `Episode ${episode.sequence} · Tension target ${Math.round(episode.tension * 100)}%`));
        if (episode.demanding_shift) summary.append(node('p', 'Large shifts in the brief add strain to this episode.', 'small'));
      }
      const work = leaf(film.phase === 'developing_idea' || readyIdea ? 'Idea development' : mode === 'preparation' ? 'Script work' : 'Studio work');
      const booking = leaf('Studio and booking');
      const call = leaf('Call sheet');
      const revisionWork = leaf('Commission a revision');
      if (film.phase === 'developing_idea' && film.due !== null) {
        const due = film.due;
        work.append(node('p', `Ready for the writing desk ${ui.stamp(due)}.`),
          action('Advance to pitch completion', () => ui.send({ type: 'advance', to_tick: due })));
      }
      if (readyIdea) work.append(action('Open writing file', () => { filter = 'idle'; chooseView('project'); }));
      if (film.script) {
        summary.append(node('h4', 'Script notes'),
          node('p', film.script.studio_demand),
          node('p', film.script.weaknesses.length ? `Needs work: ${film.script.weaknesses.map(([,name]) => name).join(', ')}` : 'The script meets the brief.'));
      }
      if (film.phase === 'script_review') work.append(action('Accept script', () => ui.send({ type: 'accept_script', film_id: film.id })));
      const revision = film.phase === 'script_review' && !film.revised && !!film.script?.weaknesses.length;
      if ((film.phase === 'pitch' && !readyIdea) || revision) {
        const assignment = revision ? revisionWork : work;
        const pool = desk.people.filter(p => p.job === 'writer');
        const select = writing.get(film.id) ?? { writer: film.writer ?? undefined, focus: film.script?.weaknesses[0]?.[0] };
        writing.set(film.id, select);
        if (revision && film.script) {
          const weaknesses = film.script.weaknesses;
          if (!weaknesses.some(([id]) => id === select.focus)) select.focus = weaknesses[0][0];
          const focus = node('select');
          for (const [id, name] of weaknesses) focus.append(option(id, name));
          focus.value = String(select.focus);
          focus.addEventListener('change', () => { select.focus = Number(focus.value); });
          assignment.append(label('What should the rewrite improve?', focus));
        }
        const due = next.now_tick + (revision ? desk.revision_ticks : desk.writing_ticks);
        const blocked = (writer: FilmPerson) => due > Math.min(writer.available_until, next.end_tick)
          ? 'Unavailable through delivery' : writer.commitments.some(([, start, end]) => start < due && end > next.now_tick)
            ? 'Already assigned during this draft' : undefined;
        assignment.append(photoSpot('film-writer-photo', 'Writer', pool.map(person => candidate(person, revision, blocked(person))),
          select.writer ?? null, id => { select.writer = id; render(next); }));
        const writer = pool.find(person => person.id === select.writer);
        if (writer) assignment.append(node('p', `Delivery ${ui.stamp(due)} · ${funds(revision ? writer.revision_fee : writer.fee)}`));
        const commission = action(revision ? 'Pay for revision' : 'Commission script', () => {
          if (writer) ui.send({ type: 'write_film', film_id: film.id, writer: writer.id, focus: revision ? select.focus ?? null : null });
        });
        commission.disabled = !writer || !!blocked(writer);
        assignment.append(commission);
      }
      if (film.phase === 'writing' || film.phase === 'revising' || ['booked','recording','post'].includes(film.phase)) {
        work.append(action(film.script && film.booking ? 'Advance to film delivery' : 'Advance to script delivery', () => {
          if (film.due !== null) ui.send({ type: 'advance', to_tick: film.due });
        }));
      }
      if (film.phase === 'casting') {
        call.append(node('p', film.title), node('p', 'Choose the studio and recording time. Review costs before reserving the shoot.', 'handwritten'));
        const actors = desk.people.filter(p => p.job === 'actor');
        const directors = desk.people.filter(p => p.job === 'director');
        const draft: FilmDraft = bookings.get(film.id) ?? {
          cast: run?.cast ? [...run.cast] : [null, null], director: null,
          studio: desk.studios[0].id, start: Math.min(next.end_tick - 30, next.now_tick + 8),
        };
        if (run?.cast) {
          draft.cast = [...run.cast];
          work.append(node('p', 'The recurring actors keep their established roles.'));
        } else if (film.episode && film.episode.sequence > 1) {
          work.append(node('p', 'Book episode one to establish the recurring cast.'));
        }
        bookings.set(film.id, draft);
        const seats = node('div', '', 'film-seats');
        for (const [index, role] of desk.roles.entries()) {
          const candidates = actors.map(person => candidate(person, false,
            draft.cast[1 - index] === person.id ? 'Already in another role' : undefined));
          seats.append(photoSpot(`film-actor-photo-${index}`, role, candidates, draft.cast[index],
            id => { draft.cast[index] = id; render(next); }, run?.cast ? 'Series cast' : undefined));
        }
        seats.append(photoSpot('film-director-photo', 'Director', directors.map(person => candidate(person)), draft.director,
          id => { draft.director = id; render(next); }));
        work.append(seats);
        const room = desk.studios.find(room => room.id === draft.studio)!;
        const studios = desk.studios.map(room => ({ id: room.id, name: room.name, description: room.summary, fee: funds(room.fee),
          photo: room.id === 1 ? 0 : room.id === 2 ? 1 : undefined,
          notes: room.commitments.map(([name, start, end]) => `Reserved: ${name} · ${ui.stamp(start)}–${ui.stamp(end)}`) }));
        const day = node('select'), time = node('select');
        day.id = 'film-recording-day'; time.id = 'film-recording-time';
        for (let i = 0; i < Math.ceil(next.end_tick / 96); i++) day.append(option(i, `Day ${i + 1}`));
        for (let i = 0; i < 96; i++) time.append(option(i, ui.stamp(i).split(' · ')[1]));
        day.value = String(Math.floor(draft.start / 96));
        time.value = String(draft.start % 96);
        const date = () => { draft.start = Number(day.value) * 96 + Number(time.value); render(next); };
        day.addEventListener('change', date); time.addEventListener('change', date);
        bindDateClock({ label: 'Film recording', day, time, firstWeekday: () => next.first_weekday });
        booking.append(studioSpot('film-studio', studios, draft.studio, id => { draft.studio = id; render(next); }), node('p', room.summary));
        for (const [name, start, end] of room.commitments) booking.append(node('p', `Reserved: ${name} · ${ui.stamp(start)}–${ui.stamp(end)}`));
        const [lead, confidant] = draft.cast;
        const planned: FilmBooking | null = lead !== null && confidant !== null && draft.director !== null
          ? { ...draft, cast: [lead, confidant], director: draft.director } : null;
        const preview = planned ? ui.previewBooking(film.id, planned) : null;
        const confirm = action('Confirm film booking', () => {
          if (planned) ui.send({ type: 'book_film', film_id: film.id, booking: planned });
        });
        confirm.disabled = !preview || 'Err' in preview;
        booking.append(label('Recording day', day), label('Recording time', time));
        if (!preview) booking.append(node('p', 'Fill the photo spots on Cast before booking.', 'handwritten'));
        else if ('Err' in preview) {
          const issue = node('p', preview.Err.message, 'placement-problem');
          issue.setAttribute('role', 'status');
          booking.append(issue);
        }
        booking.append(confirm);
        if (planned && preview && 'Ok' in preview) {
          const review = preview.Ok;
          const reviewPanel = node('div', '', 'contract-panel');
          const name = (id: number) => desk.people.find(person => person.id === id)?.name;
          reviewPanel.append(node('h4', 'Ready to film?'),
            node('p', `${name(planned.cast[0])} and ${name(planned.cast[1])} · Directed by ${name(planned.director)} · ${room.name}`),
            node('p', `${funds(review.fees)} talent + ${funds(review.rent)} studio = ${funds(review.total)}`),
            node('p', `Record ${ui.stamp(review.start)}–${ui.stamp(review.recording_end)} · Ready ${ui.stamp(review.ready)}`));
          if (review.studio_shortfall) reviewPanel.append(node('p', 'This room falls short of the script’s staging needs. You can still book it.'));
          call.append(reviewPanel);
        }
      }
      if (film.programme !== null) work.append(action('Open in programme library', () => ui.showProgramme(film.programme!)));
    const stageKey = `${film.id}:${readyIdea ? 'ready_idea' : film.phase}`;
    const workLabel = film.phase === 'developing_idea' || readyIdea ? 'Development' : film.phase === 'casting' ? 'Cast' : mode === 'preparation' ? 'Writing' : 'Studio log';
    const papers = [{ label: workLabel, content: spread(summary, work) }];
    if (film.phase === 'casting') {
      papers.push({ label: 'Booking', content: spread(call, booking) });
    } else if (film.script) {
      if (revision) {
        const choices = leaf('Notes for the rewrite');
        const weaknesses = node('ul');
        for (const [, note] of film.script.weaknesses) weaknesses.append(node('li', note));
        choices.append(weaknesses);
        papers.push({ label: 'Revision', content: spread(choices, revisionWork) });
      }
      const notes = leaf('Script');
      const qualities = leaf('Qualities');
      notes.append(node('p', film.script.studio_demand), node('p', film.script.weaknesses.map(([, note]) => note).join(' · ') || 'The script meets the brief.'));
      qualities.append(node('p', film.script.qualities.map(quality => `${quality.name} · ${quality.level}`).join(' / ')));
      papers.push({ label: 'Manuscript', content: spread(notes, qualities) });
    } else {
      const notes = leaf('Ambition');
      notes.append(node('p', film.title), node('p', 'The idea stays with this project when writing begins.', 'handwritten'));
      papers.push({ label: 'Idea', content: spread(notes, leaf()) });
    }
    const kind = film.phase === 'developing_idea' || readyIdea ? 'Idea' : mode === 'preparation' ? 'Preparation' : 'Production';
    card.append(dossier(kind, papers, {
      selected: pages.get(stageKey), close: () => chooseView('box'), turn: label => pages.set(stageKey, label),
    }));
    root.replaceChildren(card);
  }

  function resetView() { pages.clear(); selectedProject = null; filter = null; series = null; view = 'box'; }
  function newPitch(scripted = false) {
    filter = 'ideas';
    series = null;
    serial.hidden = !scripted;
    descriptionLabel.hidden = description.disabled = scripted;
    subjectLabel.hidden = !scripted;
    pace.hidden = scripted;
    heading.textContent = scripted ? 'Pitch a series' : 'Pitch a film';
    savePitch.textContent = scripted ? 'Open scripted series binder' : 'Develop this idea';
    chooseView('pitch');
  }
  return {
    render, resetView, showProject: open,
    hasOpenFile: () => view !== 'box' || filter !== null,
    showBox(shelf: ProjectShelf | null = null) { filter = shelf; series = null; chooseView('box'); },
    resume() { chooseView(view); },
    newPitch,
  };
}
