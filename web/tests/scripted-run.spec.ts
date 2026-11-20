import { expect, test } from '@playwright/test';

test('pitch distinct episode briefs and reuse the first episode cast', async ({ page }, testInfo) => {
  test.setTimeout(90_000);
  await page.goto('/#film-desk');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.getByLabel('Film title').fill('Harbour Lights');
  await page.getByText('Pitch a three-episode series', { exact: true }).click();
  await page.getByLabel('Episode 1 tension', { exact: true }).fill('0.1');
  await page.getByLabel('Episode 2 tension', { exact: true }).fill('0.9');
  await page.getByRole('button', { name: 'Open scripted series binder' }).click();
  const binder = page.locator('.scripted-binder');
  const first = binder.locator('#film-1');
  const second = binder.locator('#film-2');
  await expect(binder.locator('.film-project')).toHaveCount(3);
  await expect(second).toContainText('Tension target 90%');
  await expect(second).toContainText('Large shifts');
  await binder.screenshot({ path: testInfo.outputPath('scripted-series-binder.png') });

  await first.getByRole('button', { name: 'Commission script' }).click();
  await first.getByRole('button', { name: 'Advance to script delivery' }).click();
  await first.getByRole('button', { name: 'Accept script' }).click();
  await first.getByRole('button', { name: 'Review film booking' }).click();
  await first.getByRole('button', { name: 'Confirm film booking' }).click();
  await first.getByRole('button', { name: 'Advance to film delivery' }).click();
  await expect(first.getByRole('button', { name: 'Open in programme library' })).toBeVisible();

  await page.locator('.rail a[href="#film-desk"]').click();
  await second.getByRole('button', { name: 'Commission script' }).click();
  await second.getByRole('button', { name: 'Advance to script delivery' }).click();
  await second.getByRole('button', { name: 'Accept script' }).click();
  await expect(second).toContainText('The recurring actors keep their established roles.');
  await expect(second.locator('.film-seats section').nth(0).locator('.film-person')).toHaveCount(1);
  await expect(second.locator('.film-seats section').nth(1).locator('.film-person')).toHaveCount(1);
});
