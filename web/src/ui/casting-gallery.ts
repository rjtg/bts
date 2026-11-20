import { action, node } from './dom';
import { showDeskPicker } from './desk-picker';

export interface CastingCandidate {
  id: number;
  name: string;
  portrait: () => HTMLElement | null;
  description: string;
  fee: string;
  notes?: string[];
  unavailable?: boolean;
  room?: boolean;
}

// Authored press and agency impressions. Unlisted qualities have no published rating.
const clippings: Record<number, readonly [quality: string, rating: number][]> = {
  1: [['Information depth', 4]], 2: [['Playfulness', 4]], 3: [['Information depth', 4]],
  4: [['Information depth', 4]], 5: [['Playfulness', 3]], 6: [['Warmth', 4]],
  101: [['Warmth', 4], ['Intimacy', 3]], 102: [['Tension', 4], ['Spectacle', 4]],
  103: [['Intimacy', 4]], 104: [['Spectacle', 4], ['Pace', 4]],
  105: [['Humour', 4]], 106: [['Warmth', 4]], 107: [['Tension', 4]],
};

function photo(person?: CastingCandidate) {
  const mount = node('span', '', 'casting-photo');
  const image = person?.portrait();
  if (image) mount.append(image);
  else mount.append(node('span', person?.room ? 'STUDIO' : person ? 'Photo pending' : '+', 'photo-placeholder'));
  return mount;
}

export function photoSpot(id: string, role: string, options: CastingCandidate[], selected: number | null,
  choose: (id: number) => void, lockedLabel?: string) {
  const person = options.find(candidate => candidate.id === selected);
  const slot = action('', () => openGallery(slot, role, options, selected, choose), 'casting-photo-spot');
  slot.id = id;
  slot.disabled = !!lockedLabel;
  slot.setAttribute('aria-label', person ? `${role}: ${person.name}` : `Cast ${role}`);
  slot.setAttribute('aria-haspopup', 'dialog');
  slot.append(photo(person), node('strong', role), node('small', person?.name ?? 'Place a photo'));
  if (lockedLabel) slot.append(node('small', lockedLabel));
  return slot;
}

function openGallery(trigger: HTMLElement, role: string, candidates: CastingCandidate[], selected: number | null,
  choose: (id: number) => void) {
  const gallery = node('dialog', '', 'casting-gallery desk-picker');
  gallery.setAttribute('aria-label', `Choose ${role.toLowerCase()}`);
  const close = action('Close', () => gallery.close());
  const heading = node('header', '', 'casting-gallery-cover');
  heading.append(node('h3', role), close);
  const cards = node('div', '', 'casting-card-stack');
  for (const person of candidates) {
    const card = node('article', '', 'casting-index-card');
    card.append(photo(person), node('h4', person.name), node('p', person.description), node('strong', person.fee));
    const published = person.room ? undefined : clippings[person.id];
    if (published?.length) {
      const notes = node('ul', '', 'casting-clippings');
      for (const [quality, rating] of published) {
        const row = node('li', quality);
        const stars = node('span', '★'.repeat(rating) + '☆'.repeat(5 - rating), 'casting-stars');
        stars.setAttribute('aria-label', `${rating} of 5 stars`);
        row.append(stars);
        notes.append(row);
      }
      card.append(node('small', 'From public clippings', 'casting-evidence'), notes);
    } else if (!person.room) card.append(node('p', 'No published ratings filed.', 'casting-evidence'));
    for (const note of person.notes ?? []) card.append(node('p', note, 'casting-note'));
    const pick = action(`Choose ${person.name}`, () => { gallery.close(); choose(person.id); });
    pick.disabled = !!person.unavailable;
    if (person.id === selected) pick.setAttribute('aria-pressed', 'true');
    card.append(pick);
    cards.append(card);
  }
  if (!candidates.length) cards.append(node('p', 'No candidates filed.'));
  gallery.append(heading, cards);
  showDeskPicker(trigger, gallery, close);
}

export interface StudioCard { id: number; name: string; description: string; fee: string; notes?: string[]; photo?: number }

export function studioSpot(id: string, studios: StudioCard[], selected: number, choose: (id: number) => void) {
  return photoSpot(id, 'Studio', studios.map(studio => ({ ...studio, room: true, portrait: () => {
    if (studio.photo === undefined) return null;
    const picture = node('span', '', 'studio-photo');
    picture.style.setProperty('--studio-photo-x', studio.photo % 2 === 0 ? '0%' : '100%');
    picture.style.setProperty('--studio-photo-y', studio.photo < 2 ? '0%' : '100%');
    picture.setAttribute('aria-hidden', 'true');
    return picture;
  } })),
    selected, choose);
}

export function stationStudioCards(count: number, description: string, fee: string): StudioCard[] {
  return Array.from({ length: count }, (_, index) => ({
    id: index + 1, name: `Studio ${index + 1}`, description, fee, photo: index < 2 ? index + 2 : undefined,
  }));
}
