import { expect, test, type Page } from '@playwright/test';
import { mkdirSync } from 'node:fs';
import { pickTime } from './planner-helpers';

const narrow = { width: 390, height: 844 };
const desktop = { width: 1280, height: 900 };

async function shot(page: Page, name: string) {
  const directory = process.env.BTS_HUB_CAPTURES;
  if (!directory) return;
  mkdirSync(directory, { recursive: true });
  await page.screenshot({ path: `${directory}/${name}.png`, animations: 'disabled' });
}

async function loadOffice(page: Page) {
  await page.goto('/');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await expect(page.locator('#office-crt')).toBeVisible();
  await expect(page.getByRole('region', { name: 'Tape dossier' })).toHaveCount(0);
}

async function airingJourney(page: Page, small: boolean) {
  const name = small ? 'narrow' : 'desktop';
  await page.setViewportSize(small ? narrow : desktop);
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await loadOffice(page);
  await shot(page, `${name}-resting`);
  await expect(page.locator('#desk-tape')).toBeHidden();
  await page.locator('#open-notebook').click();
  await expect(page.locator('#desk-notebook')).toContainText('Next gap');
  await page.locator('#desk-notebook').getByRole('button', { name: 'Close notebook' }).click();
  await page.locator('#pull-tapes').click();
  await page.locator('.series-box summary').click();
  const tape = page.locator('[data-programme="1"]');
  small ? await tape.tap() : await tape.press('Enter');
  const file = page.getByRole('region', { name: 'Tape dossier' });
  await expect(file).toBeVisible();
  await expect(page.locator('#office-crt')).toHaveAttribute('inert', '');
  await expect(page.locator('#desk-file-piles')).toBeVisible();
  await shot(page, `${name}-tape`);
  if (small) await file.getByRole('button', { name: 'Close file' }).tap();
  else await page.keyboard.press('Escape');
  await expect(page.locator('#pull-tapes')).toBeFocused();
  await page.locator('#office-crt').click();
  await page.locator('#jump-evening').click();
  await shot(page, `${name}-planner`);
  await pickTime(page, 72);
  const adMinutes = page.locator('#placement-ad-minutes');
  await adMinutes.fill('16');
  await adMinutes.press('Tab');
  await expect(page.locator('#placement-runtime')).toHaveText('46 min occupied');
  await page.locator('#place').click();
  const airing = page.getByRole('region', { name: 'Airing dossier' });
  await expect(airing).toContainText('18:00–18:46');
  await airing.getByRole('button', { name: 'Find offers' }).click();
  const offer = airing.locator('.minute-offer').filter({ hasText: 'Everyday Snacks' });
  await offer.locator('summary').click();
  await offer.getByRole('button', { name: 'Accept Everyday Snacks offer' }).click();
  await expect(airing.locator('.minute-sold')).toHaveCount(1);
  await expect(airing.locator('#move')).toBeDisabled();
  const list = airing.getByRole('spinbutton', { name: 'Minutes to list' });
  await list.fill('0');
  expect(await list.evaluate((input: HTMLInputElement) => input.validity.rangeUnderflow)).toBe(true);
  // Submit without the browser's range guard to verify Rust also rejects the edit.
  await list.evaluate((input: HTMLInputElement) => input.form!.dispatchEvent(new Event('submit', { cancelable: true })));
  await expect(page.locator('#error')).toBeVisible();
  await expect(airing).toContainText('18:00–18:46');
  await page.evaluate(() => scrollTo(0, 0));
  await airing.locator(small ? '#ad-minute-market' : '.dossier-cover').evaluate(el => el.scrollIntoView({ block: 'start' }));
  await shot(page, `${name}-airing-planned`);
  await airing.locator('#advance-selected').click();
  await expect(airing).toContainText('Minute sale settlement');
  await expect(airing).toContainText('Pre-air outlook');
  await airing.locator(small ? '.airing-forecast' : '.dossier-cover').evaluate(el => el.scrollIntoView({ block: 'start' }));
  await shot(page, `${name}-airing-completed`);
  await airing.getByRole('button', { name: 'Close file' }).click();
  await expect(airing).toHaveCount(0);
  await page.locator('#back-office').click();
  await expect(page.locator('#office-crt')).toBeVisible();
}

test('desktop office through minute sale and settlement', async ({ page }) => airingJourney(page, false));
test.describe('touch office', () => {
  test.use({ hasTouch: true });
  test('narrow office through minute sale and settlement', async ({ page }) => airingJourney(page, true));
});

test('Preparation folder through script then studio booking and back to office', async ({ page }) => {
  await page.setViewportSize(desktop);
  await loadOffice(page);
  const writing = page.locator('[data-prop="script"]');
  await writing.focus();
  await expect(writing).toBeFocused();
  await writing.press('Enter');
  await page.getByRole('button', { name: 'Save free pitch' }).click();
  await page.getByRole('button', { name: 'Commission script' }).click();
  await page.getByRole('button', { name: 'Advance to script delivery' }).click();
  await expect(page.locator('[data-file="preparation"] .film-project')).toContainText('Script notes');
  await shot(page, 'desktop-preparation');
  await page.getByRole('button', { name: 'Accept script' }).click();
  await expect(page.locator('#film-production')).toBeVisible();
  await page.locator('#film-production').getByRole('link', { name: 'Back to Office' }).click();
  await page.locator('[data-prop="production"]').click();
  await page.getByRole('button', { name: 'Review film booking' }).click();
  await page.getByRole('button', { name: 'Confirm film booking' }).click();
  await expect(page.locator('#film-production .film-project')).toContainText('Filming booked');
  await shot(page, 'desktop-production');
  await page.getByRole('button', { name: 'Advance to film delivery' }).click();
  await page.getByRole('button', { name: 'Open in programme library' }).click();
  await page.locator('#back-office').click();
  const tape = page.getByRole('region', { name: 'Tape dossier' });
  await tape.getByRole('button', { name: 'Members' }).click();
  const writer = tape.getByRole('button', { name: /^Writer/ });
  await writer.click();
  const person = page.getByRole('dialog', { name: 'Character dossier' });
  await expect(person).toContainText('Known availability and commitments');
  await shot(page, 'desktop-character');
  await page.keyboard.press('Escape');
  await expect(person).toBeHidden();
  await expect(writer).toBeFocused();
});

test('Today strip accepts a tape drop and reopens the completed airing', async ({ page }) => {
  await page.setViewportSize(desktop);
  await loadOffice(page);
  const lane = page.locator('.today-lane');
  await expect(lane.getByRole('button')).toHaveCount(0);
  await page.locator('#pull-tapes').click();
  await page.locator('.series-box summary').click();
  const box = (await lane.boundingBox())!;
  await page.locator('[data-programme="1"]').dragTo(lane, {
    targetPosition: { x: Math.round(box.width * .75), y: box.height / 2 },
  });
  const airing = page.getByRole('region', { name: 'Airing dossier' });
  await expect(airing).toContainText('18:00–18:30');
  await airing.locator('#advance-selected').click();
  await airing.getByRole('button', { name: 'Close file' }).click();
  await lane.getByRole('button', { name: /18:00.*Neighbourhood Laughs/ }).click();
  await expect(airing).toContainText('Broadcast result');
});

test('office props react to hover and respect reduced motion', async ({ page }) => {
  await page.setViewportSize(desktop);
  await loadOffice(page);
  const monitors = page.locator('#office-crt');
  const reel = page.locator('.monitor-reel');
  await monitors.hover();
  await expect(reel).toHaveCSS('opacity', '1');
  await expect(reel.locator('i').first()).toHaveCSS('animation-name', 'monitor-preview');
  await shot(page, 'desktop-monitor-previews');
  const programmes = await reel.locator('i').evaluateAll(screens => screens.map(screen => getComputedStyle(screen).backgroundImage));
  expect(new Set(programmes).size).toBe(3);
  const firstScreen = reel.locator('i').first();
  const position = await firstScreen.evaluate(screen => getComputedStyle(screen).backgroundPosition);
  await expect.poll(() => firstScreen.evaluate(screen => getComputedStyle(screen).backgroundPosition)).not.toBe(position);
  for (const name of ['script', 'production', 'rights', 'ledger', 'reviews']) {
    const prop = page.locator(`#desk-file-piles [data-prop="${name}"]`);
    await prop.hover();
    expect(await prop.evaluate(el => getComputedStyle(el, '::before').backgroundImage)).toContain(`${name}-active.webp`);
  }
  const research = page.locator("#desk-file-piles").getByRole("link", { name: "Research" });
  await research.hover();
  expect(await research.evaluate(el => getComputedStyle(el, "::before").backgroundImage)).toContain("research-open.webp");
  await shot(page, "desktop-research-open");
  await expect(page.locator('#office-job-links > a')).toHaveCount(0);
  const files = page.locator('#desk-file-piles').getByRole('link', { name: 'Research' });
  await files.hover();
  await expect(files).toHaveCSS('animation-name', 'file-shuffle');
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(files).toHaveCSS('animation-name', 'none');
  await monitors.hover();
  await expect(reel).toHaveCSS('opacity', '1');
  await expect(reel.locator('i').first()).toHaveCSS('animation-name', 'none');
});
