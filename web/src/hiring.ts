import type { Command, Report } from './protocol';
import { action, element, node } from './ui/dom';
import { personPortrait } from './ui/portraits';

export function initialiseHiring(ui: {
  send: (command: Command) => void;
  stamp: (tick: number) => string;
  money: (value: string) => string;
  showProject: (id: number) => void;
}) {
  const desk = element<HTMLDetailsElement>('hiring-desk');
  const root = element('hiring-offers');
  const signed = element('hiring-signed');
  let selectedHost: number | null = null;
  const selectedDates = new Set<number>();

  function render(report: Report) {
    const people = report.production.people.filter(person => person.role === 'host');
    const projects = report.production.projects;
    const agreements = report.production.host_agreements;
    const signedIds = new Set(agreements.flatMap(agreement => agreement.dates.map(date => date.production)));
    const host = people.find(person => person.id === selectedHost);
    const cards = node('div', '', 'talent-cards');
    for (const person of people) {
      const card = node('article', '', 'production-card');
      const portrait = personPortrait(person.id, person.name);
      if (portrait) card.append(portrait);
      card.append(node('h4', person.name), node('p', person.description),
        node('p', `${ui.money(person.fee)} Funds per date`));
      const select = action(`Discuss dates with ${person.name}`, () => {
        selectedHost = person.id;
        selectedDates.clear();
        render(report);
        root.querySelector<HTMLInputElement>('input[type="checkbox"]')?.focus();
      });
      select.setAttribute('aria-pressed', String(selectedHost === person.id));
      card.append(select);
      cards.append(card);
    }
    root.replaceChildren(cards);
    if (host) {
      const available = projects.filter(project => project.host === host.id && project.phase === 'draft'
        && project.recording_end <= host.normal_until_tick && project.record_at * 15 >= report.now_minute
        && !signedIds.has(project.id)
        && !host.commitments.some(date => project.record_at < date.end_tick && project.recording_end > date.start_tick));
      const currentRun = (id: number) => report.production.discussion_runs.find(run => run.editions.some(edition => edition.production_id === id))?.series;
      for (const id of selectedDates) if (!available.some(date => date.id === id)) selectedDates.delete(id);
      const dates = node('fieldset');
      dates.append(node('legend', `Guaranteed recording dates · ${host.name}`));
      const summary = node('p', '', 'small');
      const sign = action('Sign host agreement', () => {
        ui.send({ type: 'sign_host', host: host.id, productions: [...selectedDates] });
      });
      const controls = available.map(project => {
        const label = node('label', `${project.title} · ${ui.stamp(project.record_at)}–${ui.stamp(project.recording_end)}`);
        const input = node('input'); input.type = 'checkbox'; input.checked = selectedDates.has(project.id);
        input.addEventListener('change', () => {
          if (input.checked) selectedDates.add(project.id);
          else selectedDates.delete(project.id);
          update();
        });
        label.prepend(input);
        dates.append(label);
        return { input, id: project.id };
      });
      function update() {
        const first = selectedDates.values().next().value;
        const run = first === undefined ? undefined : currentRun(first);
        for (const { input, id } of controls) {
          input.disabled = !input.checked && first !== undefined
            && (run === undefined || currentRun(id) !== run || selectedDates.size >= 3);
        }
        const total = BigInt(host!.fee) * BigInt(selectedDates.size);
        summary.textContent = `${selectedDates.size} date(s) · Pay ${ui.money(String(total))} Funds now. Unused dates are still paid.`;
        sign.disabled = selectedDates.size === 0 || total > BigInt(report.cash);
      }
      if (!available.length) dates.append(node('p', 'Create an unbooked discussion concept or weekly run for this host to choose dates.'));
      update();
      root.append(dates, summary, sign);
    }

    signed.replaceChildren();
    for (const agreement of agreements) {
      const card = node('article', '', 'production-card');
      card.append(node('h4', `Signed · ${people.find(person => person.id === agreement.host)?.name ?? 'Host'}`),
        node('p', `${ui.money(agreement.total)} Funds prepaid · ${agreement.dates.length} guaranteed date(s)`));
      for (const date of agreement.dates) {
        const line = node('p', `${ui.stamp(date.start_tick)}–${ui.stamp(date.end_tick)} · ${date.status} · `);
        line.append(action(date.title, () => ui.showProject(date.production)));
        card.append(line);
      }
      signed.append(card);
    }
    if (!desk.open && agreements.length === 0) selectedDates.clear();
  }

  return { render };
}
