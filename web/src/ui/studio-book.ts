import { element } from './dom';
import { paperIndex } from './paper-index';

export type StudioSheet = 'concept' | 'jobs' | 'requests' | 'hiring' | 'newsroom' | 'facilities';

const sheets = {
  concept: ['Discussion brief', 'production-form'],
  jobs: ['Work orders', 'studio-jobs'],
  requests: ['Sway requests', 'sway-panel'],
  hiring: ['Hiring', 'hiring-desk'],
  newsroom: ['Newsroom', 'newsroom-board'],
  facilities: ['Facilities', 'station-teams'],
} satisfies Record<StudioSheet, [string, string]>;

export function initialiseStudioBook() {
  const papers = paperIndex<StudioSheet>(element('studio-index'));
  for (const [id, [label, target]] of Object.entries(sheets) as [StudioSheet, [string, string]][]) {
    papers.add(id, label, element(target));
  }
  papers.show('concept');
  return { show: papers.show };
}
