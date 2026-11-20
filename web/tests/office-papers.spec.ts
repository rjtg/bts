import { expect, test } from '@playwright/test';
import { mkdirSync } from 'node:fs';

test('office opens one research dossier and one studio sheet at a time', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await expect(page.locator('#status')).toContainText('WASM READY');
  const capture = process.env.BTS_OFFICE_PAPERS_CAPTURES;
  if (capture) mkdirSync(capture, { recursive: true });

  await page.locator('[data-prop="research"]').click();
  await expect(page.locator('#office-paper-mount > #audience')).toBeVisible();
  const binder = page.getByRole('navigation', { name: 'Research binder contents' });
  const young = binder.getByRole('button', { name: 'Young viewers' });
  await young.press('Enter');
  await expect(young).toHaveAttribute('aria-pressed', 'true');
  const selectedSurvey = await young.getAttribute('aria-controls');
  await expect(page.locator(`#${selectedSurvey}`)).toBeFocused();
  await expect(page.locator('.binder-leaves > :visible')).toHaveCount(1);
  await expect(page.getByRole('region', { name: 'Research dossier' })).toBeVisible();
  await page.getByRole('slider', { name: 'Insight to spend' }).fill('3');
  await binder.getByRole('button', { name: 'Survey orders' }).click();
  await expect(page.getByRole('button', { name: /^Daily/ })).toBeVisible();
  await young.click();
  await expect(page.getByRole('slider', { name: 'Insight to spend' })).toHaveValue('3');
  if (capture) await page.screenshot({ path: `${capture}/research-binder.png` });
  await page.locator('#audience').getByRole('link', { name: 'Back to Office' }).click();
  await expect(page.locator('[data-prop="research"]')).toBeFocused();

  await page.locator('[data-prop="script"]').click();
  await page.getByRole('button', { name: 'Return to desk papers' }).click();
  await expect(page.locator('#office-paper-mount > #production')).toBeVisible();
  const index = page.getByRole('navigation', { name: 'Production papers' });
  await expect(index.getByRole('button', { name: 'Discussion brief' })).toHaveAttribute('aria-pressed', 'true');
  await page.getByLabel('Programme title').fill('A Saturday in the Studio');
  await index.getByRole('button', { name: 'Facilities' }).press('Enter');
  await expect(page.locator('.studio-leaves > :visible')).toHaveCount(1);
  await expect(page.locator('#station-teams')).toContainText('Elian North, Mina Sol');
  await expect(page.locator('#station-teams')).toBeFocused();
  await index.getByRole('button', { name: 'Discussion brief' }).click();
  await expect(page.getByLabel('Programme title')).toHaveValue('A Saturday in the Studio');
  await page.getByRole('button', { name: 'Review concept' }).click();
  await expect(index.getByRole('button', { name: 'Work orders' })).toHaveAttribute('aria-pressed', 'true');
  await expect(page.locator('#studio-jobs')).toContainText('A Saturday in the Studio');
  await expect(page.locator('#cash')).toHaveText('20,000');
  if (capture) await page.screenshot({ path: `${capture}/studio-work-orders.png` });

  await page.setViewportSize({ width: 390, height: 844 });
  await index.getByRole('button', { name: 'Newsroom' }).click();
  await expect(page.getByRole('button', { name: 'Start five daily editions' })).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
});
