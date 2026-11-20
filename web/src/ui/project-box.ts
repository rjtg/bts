import type { FilmCard, Report } from '../protocol';
import { action, node } from './dom';

export type ProjectShelf = 'ideas' | 'idle' | 'writing' | 'scripts' | 'casting' | 'recording' | 'completed';

export const shelfLabels: Record<ProjectShelf, string> = {
  ideas: 'Saved ideas',
  idle: 'Idle scripts',
  writing: 'Scripts in progress',
  scripts: 'Finished scripts',
  casting: 'Ready for production',
  recording: 'Productions in progress',
  completed: 'Completed films',
};

export function shelfFor(phase: FilmCard['phase']): ProjectShelf[] {
  switch (phase) {
    case 'developing_idea': return ['ideas'];
    case 'pitch': return ['ideas', 'idle'];
    case 'writing': case 'revising': return ['writing'];
    case 'script_review': return ['scripts'];
    case 'casting': return ['casting'];
    case 'booked': case 'recording': case 'post': return ['recording'];
    case 'complete': return ['completed'];
  }
}

export function projectBox(desk: Report['film_desk'], filter: ProjectShelf | null, series: number | null,
  selected: number | null, ui: { open: (id: number) => void; browseSeries: (id: number | null) => void; newPitch: (series: boolean) => void; close: () => void }) {
  const box = node('section', '', 'script-file-box');
  const run = desk.scripted_runs.find(item => item.series === series);
  const entries = desk.projects.filter(film => !filter || shelfFor(film.phase).includes(filter));
  const rows = node('div', '', 'script-file-tabs');
  const cover = node('header', '', 'dossier-cover');
  cover.append(node('h3', run?.title ?? (filter ? shelfLabels[filter] : 'Project files')), action('Close file', ui.close));
  box.append(cover, rows);
  const tab = (label: string, type: string, run: () => void, id?: number) => {
    const button = action(label, run, 'script-file-tab');
    button.append(node('small', type));
    if (id !== undefined) button.setAttribute('aria-current', String(selected === id));
    return button;
  };
  if (run) {
    box.prepend(action('← Show folders', () => ui.browseSeries(null)));
    if (run.brief.length) {
      const brief = node('details', '', 'show-folder-brief');
      brief.append(node('summary', 'Read the standing brief'), node('p', run.brief.map(q => `${q.name} · ${q.level}`).join(' / ')));
      box.append(brief);
    }
    for (const film of entries.filter(film => film.episode?.series === series).sort((a, b) => a.episode!.sequence - b.episode!.sequence)) {
      rows.append(tab(`Episode ${film.episode!.sequence}`, shelfLabels[shelfFor(film.phase)[0]], () => ui.open(film.id), film.id));
    }
  } else {
    const folders = new Map<number, FilmCard[]>();
    for (const film of entries) {
      if (film.episode) {
        const group = folders.get(film.episode.series) ?? [];
        group.push(film);
        folders.set(film.episode.series, group);
      } else rows.append(tab(film.title, shelfLabels[shelfFor(film.phase)[0]], () => ui.open(film.id), film.id));
    }
    for (const [id, episodes] of folders) {
      const show = desk.scripted_runs.find(item => item.series === id);
      rows.append(tab(show?.title ?? episodes[0].title, `${episodes.length} ${episodes.length === 1 ? 'episode file' : 'episode files'} ›`, () => ui.browseSeries(id)));
    }
  }
  if (!rows.childElementCount) rows.append(node('p', 'No files in this section.', 'empty-file-box'));
  if (!run && filter === 'ideas') {
    const choices = node('div', '', 'choice-tiles');
    choices.append(action('Draft film', () => ui.newPitch(false)), action('Draft series', () => ui.newPitch(true)));
    box.append(choices);
  }
  return box;
}
