import { expect, test, type Page } from '@playwright/test';
import { mkdirSync } from 'node:fs';
import { pickTime } from './planner-helpers';

async function capture(page: Page, name: string) {
  const directory = process.env.BTS_OFFICE_CAPTURES;
  if (!directory) return;
  mkdirSync(directory, { recursive: true });
  await page.screenshot({ path: `${directory}/${name}.png`, animations: 'disabled' });
}

async function evening(page: Page) {
  await page.goto('/');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('#office-calendar').click();
  await pickTime(page, 75);
  await page.locator('#place').click();
  await pickTime(page, 72);
  if (await page.locator('#library-picker').isVisible()) await page.locator('#close-library').click();
  await page.locator('#back-office').click();
  await page.locator('#pull-tapes').click();
  await page.getByRole('button', { name: 'Browse all tapes' }).click();
  await page.locator('.series-box summary').click();
  await page.locator('[data-programme="1"]').press('Enter');
  await expect(page.getByRole('img', { name: /Programme quality radar/ })).toBeVisible();
}

test('office, exact planning, calendars and test-card pause', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await evening(page);
  await page.evaluate(() => scrollTo(0, 0));
  for (const selector of ['#desk-tape', '#paper-on-desk']) {
    await expect(page.locator(selector)).toBeInViewport({ ratio: 0.95 });
  }
  await expect(page.locator('#notebook-cover')).toContainText('18:00');
  await capture(page, 'office');
  const dossier = page.locator('#paper-on-desk');
  await dossier.getByRole('button', { name: 'Reception', exact: true }).click();
  await expect(dossier).toContainText('We haven’t aired this tape yet.');
  await dossier.getByText('Try another time').click();
  await dossier.getByLabel('Time').fill('19:17');
  await expect(dossier).toContainText('19:17');
  await capture(page, 'reception');
  await dossier.getByRole('button', { name: 'Members', exact: true }).click();
  await expect(dossier).toContainText('Director and writer are unknown.');
  await dossier.getByRole('button', { name: 'Bottom line', exact: true }).click();
  await expect(dossier).toContainText('Ad receipts · 0 Funds');
  await dossier.getByRole('button', { name: 'Brief', exact: true }).click();
  await page.getByRole('button', { name: 'Play', exact: true }).click();
  await page.getByRole('button', { name: 'Pause', exact: true }).click();
  await expect(page.getByRole('dialog', { name: 'OFF AIR' })).toBeVisible();
  await expect(page.locator('#continue-play')).toBeFocused();
  await capture(page, 'paused');
  await page.locator('#continue-play').press('Enter');
  await expect(page.getByRole('img', { name: /Programme quality radar/ })).toBeVisible();
  await page.keyboard.press('Escape');
  await page.locator('#office-calendar').click();
  await page.getByRole('button', { name: 'Week', exact: true }).click();
  await expect(page.locator('#calendar-summary')).toContainText('ad min unsold');
  await page.getByRole('button', { name: 'Next period' }).click();
  await expect(page.locator('#calendar-position')).toHaveText('Browsing the calendar');
  await page.locator('#advance-tick').click();
  await expect(page.getByRole('heading', { name: 'Week 2', exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Today', exact: true }).click();
  await capture(page, 'week');
  await page.getByRole('button', { name: 'Month', exact: true }).click();
  await expect(page.getByRole('heading', { name: 'Month 1', exact: true })).toBeVisible();
  await expect(page.locator('#calendar-summary')).not.toContainText('Outside playable window');
  await page.getByRole('button', { name: 'Next period' }).click();
  await expect(page.getByRole('heading', { name: 'Month 2', exact: true })).toBeVisible();
  await expect(page.locator('#calendar-summary')).toContainText('Outside playable window');
  await page.getByRole('button', { name: 'Today', exact: true }).click();
  await expect(page.locator('#calendar-position')).toHaveText('Following the clock');
  await page.getByRole('button', { name: 'Day', exact: true }).click();
  await capture(page, 'planning');
  await page.locator('#back-office').click();
  await expect(page.locator('#office-calendar')).toBeVisible();
  await page.locator('#pull-tapes').click();
  await page.getByRole('button', { name: 'Browse all tapes' }).click();
  await page.locator('[data-programme="1"]').click();
  await expect(page.getByRole('img', { name: /Programme quality radar/ })).toBeVisible();
});

test('narrow office keeps the CRT and Programme paper accessible with reduced motion', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await evening(page);
  await expect(page.locator('#office-calendar')).toHaveAttribute('inert', '');
  await expect(page.locator('#desk-file-piles')).toBeVisible();
  await expect(page.getByRole('img', { name: /Programme quality radar/ })).toBeVisible();
  await page.evaluate(() => scrollTo(0, 0));
  await capture(page, 'narrow');
  await page.locator('#paper-on-desk').scrollIntoViewIfNeeded();
  await capture(page, 'narrow-paper');
});
