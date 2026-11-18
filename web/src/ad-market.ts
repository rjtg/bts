import type { Airing, Command, MinuteBooking, Report } from './protocol';
import { action, element, node } from './ui/dom';

export const salesFor = (report: Report, airing: number) => report.minute_sales.filter(sale => sale.airing_id === airing);
export const minuteRevenue = (sales: MinuteBooking[]) => sales.reduce((total, sale) => total + BigInt(sale.settlement?.payment ?? '0'), 0n);

export function renderAdMarket(
  airing: Airing,
  report: Report,
  send: (command: Command) => void,
  brandName: (id: number) => string,
  money: (value: string) => string,
) {
  const root = element('ad-minute-market');
  root.replaceChildren();
  if (!report.stations.some(station => station.is_player && station.id === airing.station)) return;
  if (airing.proposed_advertiser !== null) return;
  const id = airing.airing_id;
  const time = airing.duration_minutes + airing.ad_minutes;
  const reserved = (airing.end_tick - airing.start_tick) * report.tick_minutes;
  const free = airing.ad_minutes - airing.sold_minutes;
  const sales = salesFor(report, id);
  const editable = airing.status === 'planned';
  const panel = node('section', '', 'contract-panel minute-market');
  panel.append(node('h4', 'Sell ad time'));
  panel.append(node('p', `${airing.duration_minutes} min programme + ${airing.ad_minutes} min ad block · ${time} min runtime · ${reserved} min reserved`));
  panel.append(node('p', `${airing.sold_minutes} min sold · ${free} min available`));
  if (airing.ad_minutes > 0) {
    const strip = node('ul', '', 'minute-strip');
    for (const sale of sales) {
      const segment = node('li', `${brandName(sale.advertiser)} · ${sale.minutes} min`, 'minute-sold');
      segment.style.flexGrow = String(sale.minutes);
      strip.append(segment);
    }
    if (free > 0) {
      const segment = node('li', `${free} min open`, 'minute-free');
      segment.style.flexGrow = String(free);
      strip.append(segment);
    }
    panel.append(strip);
  }
  if (editable) {
    const settings = node('form', '', 'minute-settings');
    const amount = node('input');
    amount.type = 'number'; amount.min = String(airing.sold_minutes); amount.max = '30';
    amount.step = '1'; amount.required = true; amount.value = String(airing.ad_minutes || 15);
    const label = node('label', 'Minutes to list'); label.append(amount);
    settings.append(label, node('button', 'Set ad block'));
    settings.addEventListener('submit', event => {
      event.preventDefault();
      send({ type: 'resize_ad_block', airing_id: id, minutes: amount.valueAsNumber });
    });
    if (free > 0) {
      settings.append(action('Find offers', () => send({ type: 'request_minute_offers', airing_id: id })));
      settings.append(action('Remove unsold minutes', () => send({ type: 'resize_ad_block', airing_id: id, minutes: airing.sold_minutes })));
    }
    panel.append(settings);
    if (sales.length) panel.append(node('p', 'Sales hold this airing in place. You can still resize the block around sold minutes.'));
  }
  if (sales.length) {
    const confirmed = node('ul', '', 'minute-confirmed');
    for (const sale of sales) confirmed.append(node('li', `${brandName(sale.advertiser)} · ${sale.minutes} min · ${money(sale.price)} credits agreed`));
    panel.append(confirmed);
  }
  const batch = report.minute_offers;
  if (batch?.airing_id === id && free > 0) {
    if (!batch.offers.length) panel.append(node('p', 'No brand has a payable offer for the remaining minutes.'));
    const cards = node('div', '', 'minute-offers');
    for (const offer of batch.offers) {
      const card = node('article', '', 'minute-offer');
      const name = brandName(offer.advertiser);
      const review = node('details');
      const perMinute = (Number(offer.price) / offer.minutes).toLocaleString('en', { maximumFractionDigits: 2 });
      const mark = node('span', name.slice(0, 2).toUpperCase(), 'minute-brand-mark');
      mark.setAttribute('aria-hidden', 'true');
      const title = node('h5', name);
      title.prepend(mark);
      card.append(title, node('p', `${offer.minutes} min · ${money(offer.price)} credits · ${perMinute} / min`),
        node('p', `About ${Math.round(offer.expected_target_viewers).toLocaleString('en')} target viewers · provisional forecast`));
      review.append(node('summary', 'Review offer'),
        node('p', `Sell ${offer.minutes} minutes and leave ${free - offer.minutes} open. Payment is due if the purchased time airs.`),
        action(`Accept ${name} offer`, () => send({
          type: 'accept_minute_offer', airing_id: id, advertiser: offer.advertiser, token: batch.token,
        })));
      card.append(review); cards.append(card);
    }
    panel.append(cards);
  }
  root.append(panel);
}
