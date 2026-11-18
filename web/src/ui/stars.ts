import { node } from './dom';

export function starRating(halfStars: number): HTMLElement {
  const rating = node('span', '', 'star-rating');
  const description = `${halfStars / 2} out of 5 stars`;
  rating.setAttribute('role', 'img');
  rating.setAttribute('aria-label', description);
  rating.title = description;
  for (let index = 0; index < 5; index++) {
    const star = node('span', '★', 'rating-star');
    star.setAttribute('aria-hidden', 'true');
    star.dataset.fill = halfStars >= 2 * index + 2 ? 'full' : halfStars === 2 * index + 1 ? 'half' : 'empty';
    rating.append(star);
  }
  return rating;
}
