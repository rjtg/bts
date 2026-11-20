// Player controls and presentation only. Rust owns bookings, money and support effects.
import { element, select, node, option, action } from './ui/dom';
import { personPortrait } from './ui/portraits';
import { renderChoices } from './ui/choices';
import { amountControl } from './ui/amount';
import { modalPicker } from './ui/modal-picker';
import type { Command, DiscussionEdition, ProductionConcept, ProductionInfo, ProductionTone, Report, Subject } from './protocol';
interface Callbacks { report: () => Report; send: (command: Command) => void; showProgramme: (id: number) => void; showAiring: (id: number) => void; stamp: (tick: number) => string; money: (text: string) => string }
const tones: ProductionTone[] = ['calm', 'conversational', 'lively'];
const subjects: Record<Subject, string> = { science: 'Science & invention', everyday_life: 'Everyday life', entertainment: 'Entertainment & culture' };
type Seat = 0 | 1 | 2;
interface Choices { exceptional: number | null; support: number }
export function initialiseProduction(ui: Callbacks) {
  let editing: number | null = null;
  let cast: [number, number, number] = [1, 3, 4];
  let subject: Subject = 'science';
  let activeSeat: Seat = 0;
  let initialized = false;
  const choices = new Map<number, Choices>();
  const person = (id: number) => ui.report().production.people.find(p => p.id === id)!;
  const personName = (id: number) => person(id)?.name ?? String(id);
  const portrait = (id:number) => personPortrait(id,personName(id));
  function readConcept(): ProductionConcept {
    return { title: element<HTMLInputElement>('production-name').value, host: cast[0], guests: [cast[1], cast[2]], subject, tone: tones[Number(element<HTMLInputElement>('production-tone').value)], budget: (document.querySelector('input[name="production-budget"]:checked') as HTMLInputElement).value, record_at: Number(select('production-day').value) * 96 + Number(select('production-time').value) };
  }
  function concerns(ids: number[], container: HTMLElement): void {
    const notes = ui.report().production.known_concerns.filter(c => c.people.every(id => ids.includes(id)));
    container.replaceChildren(...notes.map(c => node('p', c.note, 'working-concern')));
    if (!notes.length) container.append(node('p', 'No working concerns are recorded for this combination. That is not a quality forecast.', 'muted small'));
  }
  function availability(id: number): { label: string; blocked: boolean } {
    const p = person(id); const start = readConcept().record_at; const end = start + 4;
    if (start < ui.report().now_tick) return { label: 'Choose a future recording time', blocked: true };
    const commitment = p.commitments.find(c => c.production_id !== editing && start < c.end_tick && end > c.start_tick);
    if (commitment) return { label: `Already recording ${commitment.title} · ${ui.stamp(commitment.start_tick)}–${ui.stamp(commitment.end_tick)}`, blocked: true };
    if (end <= p.normal_until_tick) return { label: 'Available for this recording', blocked: false };
    if (p.negotiable_from_tick !== null && start >= p.negotiable_from_tick) return { label: `Negotiation needed · ${ui.report().production.participation_cost} Sway, plus normal fees`, blocked: false };
    return { label: 'Unavailable for these recording dates', blocked: true };
  }
  function renderSeats(): void {
    const seats = element('casting-seats'); seats.replaceChildren();
    for (const [index, label] of ['Host', 'First guest', 'Second guest'].entries()) {
      const id = cast[index]; const p = person(id); if (!p) continue;
      const seat = action(``, () => openPicker(index as Seat), 'cast-seat'); seat.id = `seat-${index}`; seat.setAttribute('aria-label', `Choose ${label.toLowerCase()}: ${p.name}`);
      const img = portrait(id); if (img) seat.append(img);
      seat.append(node('span', label, 'overline'), node('strong', p.name), node('span', `${ui.money(p.fee)} Funds`), node('small', availability(id).label)); seats.append(seat);
    }
    concerns(cast, element('cast-concerns'));
    renderChoices<Subject>(element('production-subjects'),Object.entries(subjects).map(([value,label])=>({value:value as Subject,label})),subject,value=>{subject=value;renderSeats();});
    element('tone-label').textContent = tones[Number(element<HTMLInputElement>('production-tone').value)].replace(/^./, c => c.toUpperCase());
  }
  const picker = modalPicker<Report['production']['people'][number]>({ dialog:element<HTMLDialogElement>('talent-picker'), title:element('talent-picker-title'), list:element('talent-cards'), closeButton:element('close-talent-picker'), restoreFocus:()=>element(`seat-${activeSeat}`).focus(), renderItem:p=>{
      const card = node('article', '', 'talent-card'); const img = portrait(p.id); if (img) card.append(img);
      card.append(node('h4', p.name), node('p', p.station_id===null?'Freelance guest':'Your station’s staff', 'overline'), node('p', p.description), node('strong', `${ui.money(p.fee)} Funds`));
      const commitments=node('ul','','personal-schedule');
      for(const booking of p.commitments.filter(c=>c.end_tick>ui.report().now_tick))commitments.append(node('li',`${booking.title}: ${ui.stamp(booking.start_tick)}–${ui.stamp(booking.end_tick)}`));
      if(commitments.childElementCount)card.append(node('small','Upcoming recording commitments'),commitments);
      const occupied = cast.some((id, index) => index !== activeSeat && id === p.id); const available = availability(p.id);
      card.append(node('p', occupied ? 'Already in another seat' : available.label, 'small'));
      card.append(node('p', `Normal bookings through ${ui.stamp(p.normal_until_tick)}${p.negotiable_from_tick === null ? '' : `; negotiable from ${ui.stamp(p.negotiable_from_tick)}`}.`, 'small muted'));
      const choose = action(`Choose ${p.name}`, () => { cast[activeSeat] = p.id; picker.close(); renderSeats(); element(`seat-${activeSeat}`).focus(); }, ''); choose.disabled = occupied || available.blocked; card.append(choose); return card;
  } });
  function openPicker(seat:Seat):void {
    activeSeat=seat;picker.open(seat===0?'Choose your host':'Choose your guest',ui.report().production.people.filter(p=>p.role===(seat===0?'host':'guest')));
  }
  function newDraft(): void {
    editing = null; element('discussion-run-options').hidden = false; element('production-editor-title').textContent = 'New concept'; element('create-production').textContent = 'Review concept';
    const now = ui.report().now_tick;
    if (Number(select('production-day').value) * 96 + Number(select('production-time').value) < now) { select('production-day').value = String(Math.min(Math.ceil(ui.report().end_tick / 96) - 1, Math.floor(now / 96))); select('production-time').value = String(now % 96); }
  }
  function edit(p: ProductionInfo): void {
    editing = p.id; element('discussion-run-options').hidden = true; cast = [p.host, ...p.guests]; subject = p.subject;
    element<HTMLInputElement>('production-name').value = p.title;
    element<HTMLInputElement>('production-tone').value = String(tones.indexOf(p.tone));
    const budget = document.querySelector<HTMLInputElement>(`input[name="production-budget"][value="${p.budget}"]`);
    if (budget) budget.checked = true;
    else { const label = node('label', `Custom · ${ui.money(p.budget)} Funds`); const input = node('input'); input.type = 'radio'; input.name = 'production-budget'; input.value = p.budget; input.checked = true; label.prepend(input); element('production-form').querySelector('.budget-presets')!.append(label); }
    select('production-day').value = String(Math.floor(p.record_at / 96)); select('production-time').value = String(p.record_at % 96);
    element('production-editor-title').textContent = `Revise ${p.title}`; element('create-production').textContent = 'Update concept'; renderSeats(); element('production-form').scrollIntoView({ behavior: 'smooth', block: 'center' });
  }
  element('production-form').addEventListener('submit', e => { e.preventDefault(); const concept = readConcept(); ui.send(editing === null ? { type: 'create_production', concept } : { type: 'revise_production', production_id: editing, concept }); });
  element('create-discussion-run').addEventListener('click', () => {
    if (!element<HTMLFormElement>('production-form').reportValidity()) return;
    const brief = readConcept();
    const [hour, minute] = element<HTMLInputElement>('discussion-air-time').value.split(':').map(Number);
    const first_air_minute = Math.floor(brief.record_at / 96) * 1440 + hour * 60 + minute;
    const ad_minutes = element<HTMLInputElement>('discussion-ad-minutes').valueAsNumber;
    ui.send({ type: 'create_discussion_run', brief, first_air_minute, ad_minutes });
  });
  element('production-new').addEventListener('click', () => { newDraft(); renderSeats(); });
  element('production-tone').addEventListener('input', renderSeats);
  for (const id of ['production-day', 'production-time']) select(id).addEventListener('change', renderSeats);

  function bookingControls(p: ProductionInfo, card: HTMLElement): void {
    const report = ui.report(); const c = choices.get(p.id) ?? { exceptional: null, support: 0 }; choices.set(p.id, c);
    const participation = node('fieldset'); participation.append(node('legend','Bring someone in outside normal availability'));
    const group=node('div');group.id=`participation-${p.id}`;participation.append(group);
    const participationChoices:{value:number|null;label:string}[]=[{value:null,label:'Normal availability'},...report.production.people.filter(t=>[p.host,...p.guests].includes(t.id)&&p.recording_end>t.normal_until_tick&&t.negotiable_from_tick!==null&&p.record_at>=t.negotiable_from_tick).map(t=>({value:t.id,label:`Bring ${t.name} in · ${report.production.participation_cost} Sway`}))];
    renderChoices<number|null>(group,participationChoices,c.exceptional,id=>{c.exceptional=id;c.support=Math.min(c.support,supportMaximum());render(ui.report());});card.append(participation);
    const supportMaximum = () => Math.max(0, report.production.sway_balance - (c.exceptional === null ? 0 : report.production.participation_cost));
    const support = node('fieldset'); support.append(node('legend', 'Help the team work together'));
    support.append(node('p', 'Ease working friction, with smaller gains from each extra point. Support cannot supply missing skills or guarantee a successful show.', 'small muted'));
    const remaining=node('p','','support-remaining');const reviewArea=node('section','','booking-review');reviewArea.id=`booking-review-${p.id}`;
    const review=action('Review booking',()=>ui.send({type:'review_production',production_id:p.id,exceptional_person:c.exceptional,cooperation_sway:c.support}));review.id=`review-${p.id}`;
    const update=(valid:boolean)=>{review.disabled=!valid;remaining.textContent=valid?`${c.support} Sway for team support · ${c.exceptional===null?0:report.production.participation_cost} for participation · ${supportMaximum()-c.support} left after booking`:`Choose a whole amount from 0 to ${supportMaximum()} Sway.`;};
    const amount=amountControl({id:`support-${p.id}`,exactId:`support-exact-${p.id}`,label:`Team support for ${p.title}`,maximum:supportMaximum(),value:c.support,onChange:(value,valid)=>{c.support=value;reviewArea.replaceChildren();update(valid);}});
    update(amount.valid());support.append(amount.element,remaining);card.append(support,review,reviewArea);
    const r = report.booking_review;
    if (r && r.production_id === p.id && r.exceptional_person === c.exceptional && r.cooperation_sway === c.support) {
      reviewArea.append(node('h4', 'Ready to book?'), node('p', `${personName(p.host)} with ${p.guests.map(personName).join(' and ')}. ${subjects[p.subject]} · ${p.tone}.`), node('p', `Record ${ui.stamp(p.record_at)}–${ui.stamp(p.recording_end)}. Ready ${ui.stamp(p.ready_at)}.`));
      reviewArea.append(node('p', 'Aiming for ' + p.ambition.map(q => q.name + ' · ' + q.level).join(', ') + '.'));
      reviewArea.append(node('p', `Commit ${ui.money(r.budget)} Funds, including ${ui.money(p.fees)} in fees. Cash remaining: ${ui.money(r.cash_remaining)}.`), node('p', `Spend ${r.total_sway} Sway (${r.cooperation_sway} support). Sway remaining: ${r.sway_remaining}.`));
      reviewArea.append(node('p', `This reserves Studio ${r.studio} and the cast’s recording time. Booked productions cannot be changed or cancelled.`, 'small'));
      const confirm = action('Confirm booking', () => ui.send({ type: 'book_production', production_id: p.id, exceptional_person: r.exceptional_person, cooperation_sway: r.cooperation_sway }), ''); confirm.id = `confirm-${p.id}`; reviewArea.append(confirm);
    }
  }
  function render(report: Report): void {
    if (!initialized) {
      const time = (t: number) => `${String(Math.floor(t / 4)).padStart(2, '0')}:${String(t % 4 * 15).padStart(2, '0')}`;
      select('production-day').replaceChildren(...Array.from({ length: report.end_tick / 96 }, (_, i) => option(i, `Day ${i + 1}`)));
      select('production-time').replaceChildren(...Array.from({ length: 96 }, (_, i) => option(i, time(i)))); select('production-time').value = '40'; initialized = true;
    }
    const teams=element('station-teams');teams.replaceChildren(...report.stations.map(station=>{
      const card=node('section','','station-team');card.append(node('strong',station.is_player?'Your station':station.name),node('p',`Staff: ${station.staff.join(', ')||'None assigned'}.`, 'small'));
      if(station.is_player)card.append(node('p',`${report.catalogue.length} programmes in your library · ${station.owned_studios} owned studio(s).`,'small'));
      else card.append(node('p','Rival programmes are separate rights, not yours to schedule.','small muted'));
      return card;
    }));
    renderSeats(); element<HTMLButtonElement>('create-production').disabled = report.now_tick >= report.end_tick;
    const pool = element('sway-panel'); pool.replaceChildren(node('h3', `Sway · ${report.production.sway_balance}`), node('p', 'Earn cooperation by fulfilling requests. Spend it wherever it helps your station.', 'small'));
    const renderQuest = (r: Report['production']['requests'][number], target: HTMLElement) => {
      const request = node('article', '', 'sway-request'); request.id = `request-${r.id}`;
      request.append(node('strong', `${personName(r.proposer)} · ${r.reward} Sway`), node('p', r.statement), node('p', `Required: ${r.achievement.replaceAll('_', ' ')}. Matching a concept is not completion.`, 'small'));
      if (r.status === 'available') {
        request.append(node('p', `Time limit after accepting: ${r.time_limit_ticks / 4} hours, or scenario end if earlier. Recordings must start no earlier than acceptance.`, 'small'));
        const accept = action(`Accept ${personName(r.proposer)}’s quest`, () => ui.send({ type: 'accept_request', request_id:r.id }), ''); accept.disabled = report.now_tick >= report.end_tick; accept.id = `accept-request-${r.id}`; request.append(accept);
      } else if (r.status === 'open') request.append(node('p', `Open · due ${ui.stamp(r.acceptance!.deadline_tick)} · ${Math.floor(Math.max(0,r.acceptance!.deadline_tick-report.now_tick)/4)}h ${Math.max(0,r.acceptance!.deadline_tick-report.now_tick)%4*15}m left.`, 'quest-open'));
      else request.append(node('p', r.status === 'completed' ? `Completed at ${ui.stamp(r.acceptance!.fulfilled_at!)}. Reward earned once.` : `Expired at ${ui.stamp(r.acceptance!.deadline_tick)}. No reward; it cannot be accepted again.`));
      target.append(request);
    };
    pool.append(node('h4', 'Open quests')); const open = report.production.requests.filter(r => r.status === 'open');
    if (!open.length) pool.append(node('p', 'No open quests. Accept an opportunity before starting a matching recording.', 'small muted'));
    for (const r of open) renderQuest(r,pool);
    const available = report.production.requests.filter(r => r.status === 'available');
    if (available.length) { pool.append(node('h4', 'Available opportunities')); for (const r of available) renderQuest(r,pool); }
    const ended = report.production.requests.filter(r => r.status === 'completed' || r.status === 'expired');
    if (ended.length) { const history = node('details'); history.append(node('summary', 'Completed & expired quests')); for (const r of ended) renderQuest(r,history); pool.append(history); }
    if (report.production.sway_history.length) {
      const history = node('ul'); history.id = 'sway-history';
      for (const e of report.production.sway_history) history.append(node('li', `${ui.stamp(e.tick)} · ${e.kind.replaceAll('_', ' ')}${e.person_id === null ? '' : `: ${personName(e.person_id)}`} · ${e.change > 0 ? '+' : ''}${e.change} Sway · balance ${e.balance}`)); pool.append(history);
    }
    const binders = element('discussion-run-binders'); binders.replaceChildren();
    const editionCards = new Map<number, { edition: DiscussionEdition; binder: HTMLElement }>();
    for (const run of report.production.discussion_runs) {
      const binder = node('section', '', 'discussion-binder');
      binder.append(node('h3', run.title), node('p', `Weekly discussion · ${personName(run.host)} · Three editions`, 'small'));
      for (const edition of run.editions) editionCards.set(edition.production_id, { edition, binder });
      binders.append(binder);
    }
    const projects = element('production-projects'); projects.replaceChildren();
    if (!report.production.projects.length) projects.append(node('p', 'Choose your cast and review a concept to get started.', 'muted'));
    for (const p of report.production.projects) {
      const run = editionCards.get(p.id);
      const card = node('article', '', 'production-card'); card.id = `production-${p.id}`;
      const heading = node('div', '', 'panel-heading'); heading.append(node('h3', p.title), node('span', run?.edition.missed ? 'MISSED' : p.phase.replace('_', ' ').toUpperCase(), 'tag')); card.append(heading);
      card.append(node('p', `${personName(p.host)} with ${p.guests.map(personName).join(' and ')}.`), node('p', `${subjects[p.subject]} · ${p.tone} · ${ui.money(p.budget)} Funds, including ${ui.money(p.fees)} in fees.`), node('p', `Record ${ui.stamp(p.record_at)}–${ui.stamp(p.recording_end)}. Ready ${ui.stamp(p.ready_at)}.`, 'small muted'));
      for (const hint of p.quest_hints) {
        const quest = report.production.requests.find(r => r.id === hint.request_id)!;
        card.append(node('p', `${personName(quest.proposer)}’s open quest: ${hint.matches_requirements ? hint.can_finish_in_time ? `matches production requirements; could fulfil at ${ui.stamp(hint.achievement_tick)} if completed as planned.` : 'matches the concept, but cannot meet the accepted timing requirements.' : 'this concept does not match the requirements.'} Actual achievement is still required.`, 'quest-hint'));
      }
      if (p.host_prepaid !== '0') {
        const line = node('p', `Host agreement · ${ui.money(p.host_prepaid)} Funds prepaid · `);
        line.append(action('Open hiring desk', () => {
          const desk = element<HTMLDetailsElement>('hiring-desk');
          desk.open = true; desk.querySelector('summary')?.focus();
        }));
        card.append(line);
      }
      const notes = node('div'); concerns([p.host, ...p.guests], notes); card.append(notes);
      if(p.studio!==null)card.append(node('p',`Studio ${p.studio} reserved for this recording.`,'small'));
      if (p.exceptional_person !== null) card.append(node('p', `Participation secured: ${personName(p.exceptional_person)}. Normal fees still apply.`, 'small'));
      if (p.cooperation_sway > 0) card.append(node('p', `Team support: ${p.cooperation_sway} Sway committed to easing cooperation problems. It does not change anyone’s skills.`, 'small'));
      const buttons = node('div', '', 'form-row');
      if (run) {
        const airing = report.schedule.find(item => item.airing_id === run.edition.airing_id);
        const minute = airing?.start_minute ?? run.edition.planned_air_minute;
        const time = `${String(Math.floor(minute / 60) % 24).padStart(2, '0')}:${String(minute % 60).padStart(2, '0')}`;
        card.append(node('p', `Airtime · Day ${Math.floor(minute / 1440) + 1} at ${time} · ${airing?.ad_minutes ?? run.edition.ad_minutes} ad minutes`, 'small'));
        if (airing) buttons.append(action('Open airing file', () => ui.showAiring(airing.airing_id)));
        else if (run.edition.missed) card.append(node('p', 'Recording time passed without a booking. No cost or airing.', 'small'));
        else if (p.phase === 'draft') {
          const studios = report.stations.find(station => station.is_player)?.owned_studios ?? 0;
          const label = node('label', 'Recording studio ');
          const studio = node('select');
          for (let number = 1; number <= studios; number++) studio.append(option(number, `Studio ${number}`));
          label.append(studio);
          const book = action(`Book edition · ${ui.money(p.balance_due)} Funds`, () => ui.send({
            type: 'book_discussion_edition', production_id: p.id, studio: Number(studio.value),
          }));
          book.disabled = studios === 0;
          card.append(label);
          buttons.append(action('Edit edition', () => edit(p)), book);
        }
      }
      if (p.phase === 'draft' && !run) { bookingControls(p, card); buttons.append(action('Edit concept', () => edit(p)));
        if (!report.production.host_agreements.some(agreement => agreement.dates.some(date => date.production === p.id))) {
          buttons.append(action('Discard concept', () => ui.send({ type: 'discard_production', production_id: p.id })));
        } }
      else if (p.phase === 'completed') { card.append(node('p', 'Completed and ready for your schedule. Each broadcast has its own airing cost.', 'small muted')); buttons.append(action('Show in programme library', () => { if (p.programme !== null) ui.showProgramme(p.programme); }, '')); }
      else if (p.phase !== 'draft') { card.append(node('p', 'Cast and budget committed. Running to completion also processes all other broadcasts and obligations.', 'small muted')); buttons.append(action(`Finish ${p.title} · ${ui.stamp(p.ready_at)}`, () => ui.send({ type: 'advance', to_tick: p.ready_at }), '')); }
      card.append(buttons); (run?.binder ?? projects).append(card);
    }
  }
  return { render, commandSucceeded(command: Command) {
    if (command.type === 'reset') {
      choices.clear();cast=[1,3,4];subject='science';editing=null;
      element<HTMLInputElement>('production-tone').value='0';select('production-day').value='0';select('production-time').value='40';element<HTMLInputElement>('production-name').value='Workshop Conversations';
      const budget=document.querySelector<HTMLInputElement>('input[name="production-budget"][value="1000"]');if(budget)budget.checked=true;
      element('production-editor-title').textContent='New concept';element('create-production').textContent='Review concept';return;
    }
    if (command.type === 'create_discussion_run' || command.type === 'create_production' || command.type === 'revise_production' || command.type === 'discard_production') newDraft();
  }, showProject(id: number) { element(`production-${id}`).scrollIntoView({ behavior: 'smooth', block: 'center' }); } };
}
