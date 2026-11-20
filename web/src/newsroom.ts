import type { Command, NewsroomStaff, Report } from './protocol';
import { action, element, node, option, select } from './ui/dom';
import { photoSpot, stationStudioCards, studioSpot } from './ui/casting-gallery';
import { personPortrait } from './ui/portraits';
import { bindDateClock } from './ui/date-clock';

export function initialiseNewsroom(ui: {
  report: () => Report;
  send: (command: Command) => void;
  stamp: (minute: number) => string;
  money: (value: string) => string;
  showAiring: (id: number) => void;
}) {
  const form = element<HTMLFormElement>('newsroom-form');
  const firstDay = select('newsroom-day');
  const field = (name: string) => element<HTMLInputElement>(`newsroom-${name}`);
  let days = 0;
  let previousNow = 0;
  let studio = 1;
  const cast = { reporter: 0, anchor: 0 };
  const staff = (): NewsroomStaff => ({ ...cast, studio });
  bindDateClock({ label: 'First bulletin', day: firstDay, time: field('airtime'), firstWeekday: () => ui.report().first_weekday });

  function renderStaff(report: Report) {
    const roles = ['reporter', 'anchor'] as const;
    element('newsroom-photo-spots').replaceChildren(...roles.map(role => {
      const candidates = report.production.people.filter(person => person.role === role).map(person => ({
        id: person.id, name: person.name, description: person.description,
        portrait: () => personPortrait(person.id, person.name), fee: `${ui.money(person.fee)} Funds`,
        notes: [`Available through ${ui.stamp(person.normal_until_tick * 15)}`,
          ...person.commitments.map(item => `${item.title} · ${ui.stamp(item.start_tick * 15)}–${ui.stamp(item.end_tick * 15)}`)],
      }));
      const label = role === 'reporter' ? 'Reporter' : 'Anchor';
      return photoSpot(`newsroom-${role}-photo`, label, candidates, cast[role], id => { cast[role] = id; renderStaff(report); });
    }));
    const studios = report.stations.find(station => station.is_player)?.owned_studios ?? 0;
    element('newsroom-studios').replaceChildren(studioSpot('newsroom-studio', stationStudioCards(studios, 'Station studio', '100 Funds per bulletin'),
      studio, id => { studio = id; renderStaff(report); }));
  }

  form.addEventListener('submit', event => {
    event.preventDefault();
    if (ui.report().newsroom) {
      ui.send({ type: 'update_newsroom_staff', staff: staff() });
      return;
    }
    const [hours, minutes] = field('airtime').value.split(':').map(Number);
    ui.send({
      type: 'start_newsroom',
      title: field('title').value,
      first_day: Number(firstDay.value),
      minute_of_day: hours * 60 + minutes,
      ad_minutes: field('ads').valueAsNumber,
      staff: staff(),
    });
  });

  function render(report: Report) {
    const personName = (id: number) => report.production.people.find(p => p.id === id)?.name ?? 'Unassigned';
    const room = report.newsroom;
    const count = Math.floor(report.end_tick / 96);
    if (count !== days || report.now_minute < previousNow) {
      days = count;
      for (const role of ['reporter', 'anchor'] as const) {
        cast[role] = report.production.people.find(person => person.role === role)?.id ?? 0;
      }
      studio = 1;
      firstDay.replaceChildren(...Array.from({ length: Math.max(0, days - 4) }, (_, day) => option(day, `Day ${day + 1}`)));
      firstDay.value = '2';
    }
    previousNow = report.now_minute;
    renderStaff(report);
    element<HTMLFieldSetElement>('newsroom-first-plan').disabled = room !== null;
    element('newsroom-first-plan').hidden = room !== null;
    const submit = element<HTMLButtonElement>('newsroom-start');
    submit.textContent = room ? 'Apply staff to unstarted editions' : 'Start five daily editions';
    submit.disabled = room !== null && !room.editions.some(edition => edition.phase === 'planned');
    const board = element('newsroom-editions');
    board.replaceChildren();
    if (!room) return;
    board.append(node('h4', room.title));
    for (const [index, edition] of room.editions.entries()) {
      const card = node('article', '', 'production-card');
      card.id = `news-edition-${index + 1}`;
      const airing = report.schedule.find(a => a.airing_id === edition.airing_id);
      const status = airing?.status === 'completed' ? 'Aired' : airing?.status === 'off_air' ? 'Off air' : edition.phase;
      card.append(node('h5', `Edition ${index + 1} · ${status}`),
        node('p', `${personName(edition.reporter)} reporting · ${personName(edition.anchor)} presenting · Studio ${edition.studio}`),
        node('p', `Reporting ${ui.stamp(edition.reporting_tick * 15)} · Studio ${ui.stamp(edition.recording_tick * 15)}–${ui.stamp(edition.recording_end_tick * 15)}`),
        node('p', `Tape ready ${ui.stamp(edition.delivery_tick * 15)} · Air ${ui.stamp(airing?.start_minute ?? edition.air_minute)} · ${ui.money(edition.cost)} Funds`));
      if (edition.exception) card.append(node('p', `${edition.exception}. No production charge.`, 'small'));
      if (edition.airing_id !== null) {
        const id = edition.airing_id;
        card.append(action('Open bulletin airing', () => ui.showAiring(id)));
      } else if (edition.phase !== 'missed') {
        card.append(action('Advance to tape delivery', () => ui.send({ type: 'advance', to_tick: edition.delivery_tick })));
      }
      board.append(card);
    }
  }

  return { render };
}
