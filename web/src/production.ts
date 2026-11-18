// Production controls only. All scheduling, fees, willingness, rewards and scoring live in Rust.
import type { Command, ProductionConcept, ProductionInfo, ProductionTone, Report, Subject } from './protocol';
interface Callbacks { report: () => Report; send: (command: Command) => void; showProgramme: (id: number) => void; stamp: (tick: number) => string; money: (text: string) => string }
const element = <T extends HTMLElement = HTMLElement>(id: string) => document.getElementById(id)! as T;
const select = (id: string) => element<HTMLSelectElement>(id);
function node<K extends keyof HTMLElementTagNameMap>(tag: K, text = '', className = ''): HTMLElementTagNameMap[K] { const result = document.createElement(tag); result.textContent = text; result.className = className; return result; }
function option(id: number, text: string): HTMLOptionElement { const result = node('option', text); result.value = String(id); return result; }
// Fixed scenario artwork only; identity and simulation attributes remain in Rust.
const portraitFiles: Record<number, string> = { 1: 'elian-north', 2: 'mina-sol', 3: 'nora-vale', 4: 'felix-reed', 5: 'jules-hart', 6: 'rafi-linden' };
export function initialiseProduction(ui: Callbacks) {
  let editing: number | null = null;
  let initialized = false;
  const personName = (id: number) => ui.report().production.people.find(p => p.id === id)?.name ?? String(id);
  const action = (label: string, callback: () => void, className = 'quiet') => { const b = node('button', label, className); b.type = 'button'; b.addEventListener('click', callback); return b; };
  function readConcept(): ProductionConcept {
    return { title: element<HTMLInputElement>('production-name').value, host: Number(select('production-host').value), guests: [Number(select('production-guest-one').value), Number(select('production-guest-two').value)], subject: select('production-subject').value as Subject, tone: select('production-tone').value as ProductionTone, budget: select('production-budget').value, record_at: Number(select('production-day').value) * 96 + Number(select('production-time').value) };
  }
  function rosterPreview(): void {
    const report = ui.report(); const preview = element('production-roster'); preview.replaceChildren();
    const ids = [select('production-host').value, select('production-guest-one').value, select('production-guest-two').value].map(Number);
    for (const id of ids) {
      const p = report.production.people.find(p => p.id === id); if (!p) continue;
      const row = node('div', '', 'talent-preview');
      const artwork = portraitFiles[id];
      if (artwork) {
        const portrait = node('img', '', 'talent-portrait'); portrait.src = `${import.meta.env.BASE_URL}portraits/${artwork}.png`;
        portrait.alt = `Portrait of ${p.name}`; portrait.width = 96; portrait.height = 96; row.append(portrait);
      }
      const details = node('section'); details.append(node('strong', `${p.name} · ${ui.money(p.fee)} credits`), node('p', p.description)); row.append(details); preview.append(row);
    }
  }
  function edit(p: ProductionInfo): void {
    editing = p.id;
    element<HTMLInputElement>('production-name').value = p.title;
    select('production-host').value = String(p.host); select('production-guest-one').value = String(p.guests[0]); select('production-guest-two').value = String(p.guests[1]);
    select('production-subject').value = p.subject; select('production-tone').value = p.tone; select('production-budget').value = p.budget;
    select('production-day').value = String(Math.floor(p.record_at / 96)); select('production-time').value = String(p.record_at % 96);
    element('production-editor-title').textContent = `Edit draft #${p.id}`; element('create-production').textContent = `Update draft #${p.id}`; rosterPreview(); element('production-form').scrollIntoView({ behavior: 'smooth', block: 'center' });
  }
  function newDraft(): void {
    editing = null; element('production-editor-title').textContent = 'New concept'; element('create-production').textContent = 'Review concept';
    const now = ui.report().now_tick;
    if (Number(select('production-day').value) * 96 + Number(select('production-time').value) < now) { select('production-day').value = String(Math.min(6, Math.floor(now / 96))); select('production-time').value = String(now % 96); }
  }
  element('production-form').addEventListener('submit', event => { event.preventDefault(); const concept = readConcept(); ui.send(editing === null ? { type: 'create_production', concept } : { type: 'revise_production', production_id: editing, concept }); });
  element('production-new').addEventListener('click', newDraft);
  for (const id of ['production-host', 'production-guest-one', 'production-guest-two']) select(id).addEventListener('change', rosterPreview);
  function render(report: Report): void {
    if (!initialized) {
      const tickTime = (tick: number) => `${String(Math.floor(tick / 4)).padStart(2, '0')}:${String(tick % 4 * 15).padStart(2, '0')}`;
      select('production-day').replaceChildren(...Array.from({ length: report.end_tick / 96 }, (_, i) => option(i, `Day ${i + 1}`)));
      select('production-time').replaceChildren(...Array.from({ length: 96 }, (_, i) => option(i, tickTime(i)))); select('production-time').value = '40';
      initialized = true;
    }
    for (const [id, role, fallback] of [['production-host', 'host', 1], ['production-guest-one', 'guest', 3], ['production-guest-two', 'guest', 4]] as const) {
      const previous = select(id).value;
      select(id).replaceChildren(...report.production.people.filter(p => p.role === role).map(p => option(p.id, `${p.name} · ${ui.money(p.fee)} cr`)));
      select(id).value = report.production.people.some(p => String(p.id) === previous && p.role === role) ? previous : String(fallback);
    }
    rosterPreview();
    element<HTMLButtonElement>('create-production').disabled = report.now_tick >= report.end_tick;
    const swayPanel = element('sway-panel'); swayPanel.replaceChildren(node('h3', `Pooled Sway · ${report.production.sway_balance}`));
    swayPanel.append(node('p', 'Station-wide social capital. Earn from explicit completed requests; spend on anyone’s negotiable participation or on cooperation. Amounts and effects are provisional.', 'small muted'));
    for (const request of report.production.requests) {
      const card = node('article', '', 'sway-request'); card.id = `request-${request.id}`;
      card.append(node('strong', `${personName(request.proposer)} · request #${request.id} · reward ${request.reward} Sway`), node('p', request.statement));
      card.append(node('p', `Achievement required: ${request.achievement.replaceAll('_', ' ')}. Booking alone earns nothing.`, 'small'));
      if (!request.promise) card.append(node('p', 'Available: explicitly promise this request when booking a matching concept.', 'muted small'));
      else card.append(node('p', request.promise.fulfilled_at === null ? `Promised for production #${request.promise.production_id}; not fulfilled yet.` : `Fulfilled at ${ui.stamp(request.promise.fulfilled_at)} for production #${request.promise.production_id}. Reward earned once.`));
      swayPanel.append(card);
    }
    if (report.production.sway_history.length) {
      const history = node('ul'); history.id = 'sway-history';
      for (const entry of report.production.sway_history) history.append(node('li', `${ui.stamp(entry.tick)} · production #${entry.production_id} · ${entry.kind.replaceAll('_', ' ')}${entry.person_id === null ? '' : `: ${personName(entry.person_id)}`}${entry.request_id === null ? '' : ` #${entry.request_id}`} · ${entry.change > 0 ? '+' : ''}${entry.change} Sway · balance ${entry.balance}`));
      swayPanel.append(node('h4', 'Sway history'), history);
    }
    const projects = element('production-projects'); projects.replaceChildren();
    if (!report.production.projects.length) projects.append(node('p', 'No production concepts yet. Review a concept to see its dates and fees before booking.', 'muted'));
    for (const p of report.production.projects) {
      const card = node('article', '', 'production-card'); card.id = `production-${p.id}`; card.dataset.production = String(p.id);
      const heading = node('div', '', 'panel-heading'); heading.append(node('h3', `${p.title} · #${p.id}`), node('span', p.phase.replace('_', ' ').toUpperCase(), 'tag')); card.append(heading);
      card.append(node('p', `${personName(p.host)} with ${p.guests.map(personName).join(' and ')}.`));
      card.append(node('p', `${p.subject.replace('_', ' ')} · ${p.tone} · budget ${ui.money(p.budget)} credits, including ${ui.money(p.fees)} in fees.`));
      card.append(node('p', `Record ${ui.stamp(p.record_at)}–${ui.stamp(p.recording_end)}. Library ready ${ui.stamp(p.ready_at)}.`, 'small muted'));
      if (p.exceptional_person !== null) card.append(node('p', `Exceptional participation secured: ${personName(p.exceptional_person)}. Fees paid from the production budget.`, 'small'));
      if (p.smooth_cooperation) card.append(node('p', 'Cooperation smoothing applied: tone-coordination shortfall halved. Craft and topic limitations remain; execution is unknown.', 'small'));
      const buttons = node('div', '', 'form-row');
      if (p.phase === 'draft') {
        const promise = node('select'); promise.id = `request-choice-${p.id}`;
        const none = node('option', 'No request promised'); none.value = ''; promise.append(none);
        promise.append(...report.production.requests.filter(r => !r.promise).map(r => option(r.id, `Request #${r.id} · ${personName(r.proposer)} · ${r.reward} Sway`)));
        const promiseLabel = node('label', 'Explicit request for this production'); promiseLabel.append(promise); card.append(promiseLabel);
        const exceptional = node('select'); exceptional.id = `exceptional-${p.id}`;
        const normal = node('option', 'Use normal availability'); normal.value = ''; exceptional.append(normal);
        exceptional.append(...report.production.people.filter(t => [p.host, ...p.guests].includes(t.id) && t.negotiable_from_tick !== null && p.record_at >= t.negotiable_from_tick && p.recording_end > t.normal_until_tick).map(t => option(t.id, `Negotiate ${t.name} · ${report.production.participation_cost} Sway`)));
        const exceptionalLabel = node('label', 'Exceptional participation · fees and conflicts still apply'); exceptionalLabel.append(exceptional); card.append(exceptionalLabel);
        const smooth = node('input'); smooth.type = 'checkbox'; smooth.id = `smooth-${p.id}`;
        const smoothLabel = node('label', '', 'check-label'); smoothLabel.append(smooth, node('span', `Smooth cooperation · ${report.production.cooperation_cost} Sway · halve tone-coordination shortfall; no guarantee of creative success`)); card.append(smoothLabel);
        buttons.append(action(`Book production #${p.id}`, () => ui.send({ type: 'book_production', production_id: p.id, promise_request: promise.value === '' ? null : Number(promise.value), exceptional_person: exceptional.value === '' ? null : Number(exceptional.value), smooth_cooperation: smooth.checked }), ''));
        buttons.append(action('Edit draft', () => edit(p)), action('Discard draft', () => ui.send({ type: 'discard_production', production_id: p.id })));
      } else if (p.phase === 'completed') {
        card.append(node('p', `Completed programme #${p.programme} is in your library. Recording budget was spent once; each broadcast has its own airing cost.`, 'muted small'));
        buttons.append(action('Show in programme library', () => { if (p.programme !== null) ui.showProgramme(p.programme); }, ''));
      } else {
        card.append(node('p', 'Participants and budget are committed. Advancing time also processes intervening broadcasts and obligations.', 'muted small'));
        const advance = action(`Run through production #${p.id} · to ${ui.stamp(p.ready_at)}`, () => ui.send({ type: 'advance', to_tick: p.ready_at }), ''); advance.disabled = p.ready_at <= report.now_tick; buttons.append(advance);
      }
      card.append(buttons); projects.append(card);
    }
  }
  return { render, commandSucceeded(command: Command) { if (command.type === 'create_production' || command.type === 'revise_production' || command.type === 'discard_production' || command.type === 'reset') newDraft(); }, showProject(id: number) { element(`production-${id}`).scrollIntoView({ behavior: 'smooth', block: 'center' }); } };
}
