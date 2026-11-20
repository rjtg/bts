import type { Command, MarketTitle, Report } from './protocol';
import { brandHeadline } from './ad-market';
import { dossier, leaf, spread } from './ui/dossier';
import { action, element, node } from './ui/dom';

const descriptions: Record<string, string> = {
  released: 'New listing', purchased: 'Rights acquired', rival_purchased: 'Rival acquired the rights',
  assessment_ready: 'Assessment ready', expired: 'Listing closed', resale_bid: 'Resale offer received',
  resold: 'Rights sold to the rival',
};

export function initialiseRightsMarket(ui: {
  send: (command: Command) => void; stamp: (tick: number) => string;
  money: (value: string) => string; showProgramme: (id: number) => void; showAiring: (id: number) => void;
}) {
  const draftInsight = new Map<number, number>();
  const expanded = new Set<number>();
  let lastTick = 0;
  let selection: number | null = null;
  const titleFacts = (title: MarketTitle) => {
    const details = node('details', '', 'rights-assessment');
    details.open = expanded.has(title.programme);
    details.addEventListener('toggle', () => {
      if (details.open) expanded.add(title.programme); else expanded.delete(title.programme);
    });
    const audience = node('ul');
    for (const [name, viewers] of title.groups) audience.append(node('li', `${name} · about ${viewers.toLocaleString('en')}`));
    details.append(node('summary', 'Audience and programme notes'),
      node('p', title.qualities.map(q => `${q.name} · ${q.level}`).join(' / ')), audience);
    return details;
  };
  return { render(report: Report) {
    if (report.now_tick < lastTick) { draftInsight.clear(); expanded.clear(); selection = null; }
    lastTick = report.now_tick;
    const market = report.rights_market;
    const player = report.stations.find(station => station.is_player)!.id;
    const cash = BigInt(report.cash);
    const insight = report.research?.insight ?? 0;
    const news = element('rights-news');
    const root = element('rights-catalogue');
    const collection = node('nav', '', 'portfolio-index');
    collection.setAttribute('aria-label', 'Title index');
    const files = node('div', '', 'portfolio-files');
    selection ??= market.titles[0]?.programme ?? null;
    const openTitle = (id: number, focus = true) => {
      selection = id;
      for (const file of files.children) (file as HTMLElement).hidden = file.id !== `market-title-${id}`;
      for (const tab of collection.querySelectorAll('button')) tab.setAttribute('aria-pressed', String(tab.dataset.title === String(id)));
      if (focus) document.getElementById(`market-title-${id}`)?.focus();
    };
    const wires = node('div', '', 'rights-wire');
    wires.append(node('h3', 'Industry wire'), node('p', `${insight} Insight available`));
    const stories = market.news.map(event => {
      const title = market.titles.find(title => title.programme === event.programme);
      const line = node('p', `${ui.stamp(event.tick)} · ${descriptions[event.kind]} · `);
      if (title) line.append(action(title.name, () => openTitle(title.programme), 'row-link'));
      return { tick: event.tick, line };
    });
    for (const event of report.brand_news) {
      const brand = report.advertisers.find(item => item.id === event.advertiser);
      stories.push({ tick: event.tick, line: node('p', `${ui.stamp(event.tick)} · ${brand?.name ?? 'Advertiser'} · ${brandHeadline(event.direction)}`) });
    }
    stories.sort((a, b) => b.tick - a.tick);
    wires.append(...stories.slice(0, 6).map(story => story.line));
    news.replaceChildren(wires);
    root.replaceChildren(collection, files);
    for (const title of [...market.titles].sort((a, b) => a.name.localeCompare(b.name))) {
      const id = title.programme;
      const owned = title.owner === player;
      const card = node('article', '', 'production-card rights-card');
      card.id = `market-title-${id}`; card.tabIndex = -1;
      const art = node('div', '', `rights-art rights-art-${title.art}`);
      art.setAttribute('aria-hidden', 'true');
      const tab = action(title.name, () => openTitle(id), 'portfolio-tab');
      tab.dataset.title = String(id);
      tab.setAttribute('aria-controls', card.id);
      tab.prepend(art);
      tab.append(node('small', title.acquisition ? 'Available' : owned ? 'Owned' : 'Industry file'));
      collection.append(tab);
      const evidence = leaf(title.name);
      const decisions = leaf('Acquisition & assessment');
      evidence.append(node('p', title.evidence),
        node('p', `${title.duration} min · ${ui.money(title.broadcast_cost)} Funds per airing`),
        node('p', `About ${title.expected_viewers.toLocaleString('en')} viewers at 18:00 · ${title.uncertainty}`, 'rights-forecast'),
        titleFacts(title));
      if (title.assessment_at !== null) decisions.append(node('p', `Assessment delivered ${ui.stamp(title.assessment_at)}`));
      if (title.assessment_due !== null) {
        const due = title.assessment_due;
        decisions.append(node('p', `Assessment due ${ui.stamp(due)}`),
          action('Advance to assessment', () => ui.send({ type: 'advance', to_tick: due })));
      }
      if ((title.acquisition || owned) && title.assessment_due === null && report.now_tick + market.assessment_ticks <= report.end_tick) {
        const study = node('form', '', 'rights-study');
        const input = node('input');
        input.type = 'number'; input.min = '1'; input.max = String(insight);
        input.value = String(Math.min(insight, draftInsight.get(id) ?? 3)); input.required = true;
        const label = node('label', 'Insight for assessment'); label.append(input);
        const submit = node('button', 'Commission assessment');
        input.disabled = submit.disabled = insight === 0;
        input.addEventListener('input', () => draftInsight.set(id, input.valueAsNumber || 1));
        study.append(label, node('p', `Ready ${ui.stamp(report.now_tick + market.assessment_ticks)}`), submit);
        study.addEventListener('submit', event => {
          event.preventDefault(); ui.send({ type: 'assess_title', programme: id, insight: input.valueAsNumber });
        });
        decisions.append(study);
      }
      if (title.acquisition) {
        const offer = title.acquisition;
        const buy = action(`Buy rights · ${ui.money(offer.price)} Funds`, () =>
          ui.send({ type: 'buy_rights', programme: id, token: offer.token }));
        buy.disabled = cash < BigInt(offer.price);
        decisions.append(node('p', `Available until ${ui.stamp(offer.deadline)}`), buy);
      } else if (owned) {
        decisions.append(node('p', 'In your library', 'receipt-stamp'),
          action('Pull tape', () => ui.showProgramme(id)));
        if (title.blocker !== null) decisions.append(action(`Review blocking airing #${title.blocker}`, () => ui.showAiring(title.blocker!)));
        if (title.resale) {
          const offer = title.resale;
          const accept = action(`Sell rights · ${ui.money(offer.price)} Funds`, () =>
            ui.send({ type: 'accept_resale', programme: id, token: offer.token }));
          accept.disabled = title.blocker !== null;
          decisions.append(node('p', `Rival offer expires ${ui.stamp(offer.deadline)}`), accept);
        } else decisions.append(action('List for resale', () => ui.send({ type: 'list_resale', programme: id })));
      } else if (title.owner !== null) {
        const station = report.stations.find(station => station.id === title.owner);
        decisions.append(node('p', `On the shelf at ${station?.name ?? 'the rival station'}`));
        if (title.rival_airing !== null) decisions.append(action('See rival airing', () => ui.showAiring(title.rival_airing!)));
      } else decisions.append(node('p', 'Listing closed'));
      card.append(dossier('Market', [{ label: 'Rights', content: spread(evidence, decisions) }]));
      files.append(card);
    }
    if (selection !== null) openTitle(selection, false);
  }};
}
