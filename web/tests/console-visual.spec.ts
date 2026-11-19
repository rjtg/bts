import { pickTime, setAdLoad } from './planner-helpers';
import { expect, test, type Page } from '@playwright/test';
import { mkdirSync } from 'node:fs';

async function stageEvening(page: Page) {
  await page.goto('/#planning');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await pickTime(page, 75);
  await page.locator('#place').click();
  await page.locator('#jump-evening').click();
  await page.getByRole('button', { name: 'Choose placement Day 1 · 18:00' }).click();
  await setAdLoad(page, 33);
  await expect(page.locator('#desk-notes')).toContainText('18:00 · 45 min');
  await page.getByText('Neighbourhood Laughs · 3 episodes', { exact: true }).click();
  await page.locator('[data-programme="1"]').press('Enter');
  await expect(page.locator('#desk-tape')).toContainText('Episode 1 · 30 min');
  await expect(page.locator('.shelf-slot-preview')).toContainText('Neighbourhood Laughs');
}

async function capture(page: Page, name: string) {
  const directory = process.env.BTS_VISUAL_DIR;
  if (!directory) return;
  mkdirSync(directory, { recursive: true });
  await page.screenshot({ path: `${directory}/${name}.png`, animations: 'disabled' });
}

test('replay the reference evening and open its Audience paper', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await stageEvening(page);
  await page.evaluate(() => window.scrollTo(0, 0));
  await capture(page, 'schedule');
  for (const id of ['clock', 'play-pause', 'timeline-scroller']) {
    await expect(page.locator('#' + id)).toBeInViewport({ ratio: 1 });
  }
  await expect(page.locator('.shelf-slot-preview')).toBeInViewport({ ratio: 1 });
  const file = page.locator('#paper-on-desk');
  await file.getByRole('button', { name: 'Audience estimate' }).click();
  await expect(file).toContainText('Slot under consideration · Saturday · Day 1 · 18:00');
  await capture(page, 'audience');
  await page.getByRole('button', { name: 'Place at 18:00' }).press('Enter');
  await expect(page.locator('#selected-name')).toHaveText('Neighbourhood Laughs');
});

test.describe('touch console', () => {
  test.use({ hasTouch: true });
  test('narrow reduced-motion console keeps the selected placement reachable', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await stageEvening(page);
    await expect(page.getByRole('dialog', { name: 'Tape shelf' })).toBeVisible();
    await expect(page.locator('#desk-tape')).toHaveCSS('animation-name', 'none');
    await expect(page.getByRole('button', { name: 'Place at 18:00' })).toBeInViewport();
    await capture(page, 'narrow-shelf');
    await page.getByRole('button', { name: 'Open station file' }).click();
    const file = page.getByRole('dialog', { name: 'Neighbourhood Laughs' });
    await file.getByRole('button', { name: 'Audience estimate' }).click();
    await expect(file).toBeInViewport({ ratio: 1 });
    await capture(page, 'narrow-audience');
    await file.getByRole('button', { name: 'Close record' }).click();
    await page.getByRole('button', { name: 'Place at 18:00' }).tap();
    await expect(page.locator('#selected-window')).toContainText('18:00–18:40');
  });
});
