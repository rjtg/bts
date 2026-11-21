import { expect, test } from '@playwright/test';
import { writeFileSync } from 'node:fs';
import { pickTime } from './planner-helpers';
import { goToDesk } from './station-directory';

const figure = (value: string | null) => Number(value?.match(/-?[\d,]+/)?.[0].replaceAll(',', ''));

test('a full season links rights, audience evidence, scheduling and signed receipts', async ({ page }, testInfo) => {
  test.setTimeout(120_000);
  page.setDefaultTimeout(10_000);
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('[data-prop="rights"]').click();
  const market = page.locator('#rights-market');
  await market.getByRole('button', { name: /The Corner Shop/ }).first().click();
  const listing = market.locator('article').filter({ has: page.getByRole('heading', { name: 'The Corner Shop' }) });
  await listing.getByLabel('Insight for assessment').fill('3');
  await listing.getByRole('button', { name: 'Commission assessment' }).click();
  await listing.getByRole('button', { name: 'Advance to assessment' }).click();
  await listing.getByRole('button', { name: /Buy rights/ }).click();
  await expect(listing).toContainText('In your library');
  await page.screenshot({ path: testInfo.outputPath('season-rights.png') });
  await listing.getByRole('button', { name: 'Pull tape' }).click();
  const tape = page.getByRole('region', { name: 'Tape dossier' });
  await tape.getByRole('button', { name: 'Reception', exact: true }).click();
  await tape.getByRole('button', { name: 'Find airtime' }).click();
  await pickTime(page, 72, 0);
  await page.locator('#placement-ad-minutes').fill('10');
  await page.locator('#placement-ad-minutes').press('Tab');
  await page.locator('#place').click();

  const airing = page.getByRole('region', { name: 'Airing dossier' });
  await expect(airing).toContainText('The Corner Shop');
  const outlook = await airing.locator('.airing-forecast p').first().textContent();
  expect(outlook).toMatch(/survey/);
  await airing.getByRole('button', { name: 'Find offers' }).click();
  const offer = airing.locator('.minute-offer').filter({ hasText: 'Everyday Snacks' });
  await offer.locator('summary').click();
  await offer.getByRole('button', { name: 'Accept Everyday Snacks offer' }).click();
  await expect(airing.locator('#move')).toBeDisabled();
  await airing.locator('#advance-selected').click();
  await expect(airing).toContainText('Pre-air outlook');
  await expect(airing).toContainText('Minute sale settlement');
  const result = airing.locator('.contract-panel').filter({ hasText: 'Minute sale settlement' });
  const statement = result.getByRole('table').filter({ has: page.locator('caption', { hasText: 'Minute sale settlement' }) });
  const paid = figure(await statement.locator('tbody tr td').nth(2).textContent());
  const minutes = figure(await statement.locator('tbody tr td').nth(1).textContent());
  const margin = figure(await result.locator('.finance-stat').filter({ hasText: 'Airing margin' }).locator('strong').textContent());
  const groups = result.getByRole('table').filter({ has: page.locator('caption', { hasText: 'Provisional expected counts' }) });
  const turnout = await groups.locator('tbody tr').evaluateAll(rows => rows.map(row => {
    const [group, count] = Array.from(row.querySelectorAll('td'), cell => cell.textContent ?? '');
    return { group, viewers: Number(count.replaceAll(',', '')) };
  }));
  expect(turnout.reduce((total, group) => total + group.viewers, 0)).toBeGreaterThan(0);
  expect(paid).toBeGreaterThan(0);
  expect(minutes).toBeGreaterThan(0);
  await page.screenshot({ path: testInfo.outputPath('season-first-receipt.png') });

  // Use the first receipt to set later selling terms, then get a new audience survey.
  const floor = Math.max(1, Math.min(35, Math.floor(paid / minutes / 4)));
  await goToDesk(page, /Audience/);
  const research = page.locator('#audience-research');
  await research.getByRole('button', { name: /^Daily/ }).click();
  await research.getByRole('button', { name: 'Young viewers', exact: true }).click();
  const young = research.locator('article').filter({ has: page.getByRole('heading', { name: 'Young viewers' }) });
  await young.getByLabel('Insight to spend').fill('8');
  await young.getByRole('button', { name: 'Spend 8 Insight' }).click();
  await page.locator('#advance-day').click();
  await expect(young).toContainText('Day 2 · 00:00');
  await expect(young).toContainText('Familiarity at poll: High');
  await page.screenshot({ path: testInfo.outputPath('season-survey.png') });
  await research.getByRole('button', { name: 'Survey orders' }).click();
  await research.getByRole('button', { name: /^Off/ }).click();

  await page.locator('#audience').getByRole('link', { name: 'Back to Office' }).click();
  await expect(airing).toContainText('The Corner Shop');
  await airing.getByRole('button', { name: 'Close file' }).click();
  await page.locator('#pull-tapes').click();
  const wheel = page.locator('.desk-radial.film-can');
  await wheel.getByRole('button', { name: 'Browse all tapes' }).click();
  await page.locator('.series-box summary').first().click();
  await page.locator('[data-programme="1"]').click();
  await page.getByRole('button', { name: 'Schedule this tape' }).click();
  await wheel.getByRole('button', { name: 'Regular', exact: true }).click();
  await wheel.getByRole('button', { name: 'Choose days and time' }).click();
  const weekly = page.locator('#weekly-planner');
  await weekly.getByLabel('Weekly programme').selectOption('1');
  await weekly.getByLabel('Regular airtime').fill('18:00');
  await weekly.getByLabel('Default ad minutes').fill('10');
  await weekly.getByRole('button', { name: 'Preview weekly slots' }).click();
  await expect(weekly.locator('.weekly-date')).toHaveCount(3);
  await weekly.getByRole('button', { name: 'Confirm weekly schedule' }).click();
  await page.locator('#open-ad-sales').click();
  const ads = page.locator('#ad-minute-market');
  await ads.getByText('Autosell this run').click();
  await ads.getByLabel('Minimum Funds per minute').fill(String(floor));
  await ads.getByLabel('Maximum minutes sold').fill('8');
  await ads.getByRole('button', { name: 'Save autosell limits' }).click();
  await expect(ads).toContainText('Autosell on for this run');
  await page.screenshot({ path: testInfo.outputPath('season-policy.png') });
  await goToDesk(page, '▦ Schedule');
  await page.locator('#advance-selected').click();
  await expect(page.locator('#airing-paper-content')).toContainText('Autosell');
  const advance = page.locator('#advance-day');
  let days = 0;
  while (await advance.isEnabled() && days++ < 42) await advance.click();
  await expect(page.locator('#playback-state')).toHaveText('COMPLETE');
  await page.getByRole('button', { name: 'Station notes', exact: true }).click();
  const notes = page.getByRole('dialog', { name: 'Station notes' });
  await expect(notes).toContainText('Season closed · 3 of 3 goals complete');
  await page.screenshot({ path: testInfo.outputPath('season-close.png') });
  writeFileSync(testInfo.outputPath('season-browser.json'), JSON.stringify({
    outlook, turnout, firstReceipt: { paid, minutes, margin }, recurringFloor: floor,
    closedAt: await page.locator('#clock').textContent(), funds: await page.locator('#cash').textContent(),
    goals: '3 of 3',
  }, null, 2) + '\n');
});
