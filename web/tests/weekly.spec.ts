import { pickTime } from './planner-helpers';
import { expect, test, type Locator, type Page } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';

const root = resolve(import.meta.dirname, '../..');

async function dragAcrossConsole(page: Page, source: Locator, target: Locator) {
  await target.scrollIntoViewIfNeeded();
  await source.scrollIntoViewIfNeeded();
  const from = (await source.boundingBox())!;
  const to = (await target.boundingBox())!;
  await page.mouse.move(from.x + from.width / 2, from.y + from.height / 2);
  await page.mouse.down();
  await page.mouse.move(to.x + 2, to.y + to.height / 2, { steps: 12 });
  await page.mouse.move(to.x + 3, to.y + to.height / 2);
  await page.mouse.up();
}

test('three Saturdays with a moved exception, sold ad minutes and a completed first airing', async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  await page.goto('/#planning');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await expect(page.locator('#clock')).toContainText('Saturday');
  await page.getByRole('link', { name: /Audience/ }).click();
  await expect(page.locator('#audience .watch-time-note').first()).toContainText('Weekend evenings');
  await page.getByRole('link', { name: '▦ Schedule', exact: true }).click();
  await page.locator('[data-programme="2"]').click();
  await pickTime(page, 72, 7);
  await page.locator('#place').click();
  await page.getByRole('link', { name: 'Regular slot' }).click();
  const weekly = page.locator('#weekly-planner');
  await weekly.getByLabel('Weekly programme').selectOption('1');
  await weekly.getByLabel('Regular airtime').fill('18:00');
  await weekly.getByLabel('Default ad minutes').fill('10');
  await weekly.getByRole('button', { name: 'Preview weekly slots' }).click();
  await expect(weekly.locator('.weekly-date')).toHaveCount(3);
  await expect(weekly).toContainText('Saturday · Day 15 · 18:00');
  const conflict = weekly.locator('.weekly-conflict');
  await expect(conflict).toContainText('Saturday · Day 8 · 18:00');
  await expect(conflict).toContainText('Workshop Science');
  await expect(weekly.getByRole('button', { name: 'Confirm weekly schedule' })).toBeDisabled();
  if (process.env.BTS_WEEKLY_SCREENSHOTS) await weekly.screenshot({ path: '../contrib/weekly-planner/clash.png' });
  await conflict.locator('summary').click();
  await conflict.getByLabel('New time', { exact: true }).fill('20:07');
  await conflict.getByRole('button', { name: 'Update preview' }).click();
  await expect(weekly).toContainText('Moved to Saturday · Day 8 · 20:07');
  if (process.env.BTS_DESK_SCREENSHOTS) await page.screenshot({ path: '../contrib/focused-desks/weekly-run.png', fullPage: true });
  await weekly.getByRole('button', { name: 'Confirm weekly schedule' }).click();
  await expect(weekly).toBeHidden();
  await expect(page.locator('#selected-status')).toContainText('WEEKLY SLOT');
  await page.locator('#day').selectOption('7');
  await expect(page.locator('[data-station="1"] [data-start-minute="11287"]')).toHaveCount(1);
  await page.locator('#day').selectOption('0');
  await page.locator('[data-station="1"] [data-start="72"]').click();
  await expect(page.locator('#selected-status')).toContainText('WEEKLY SLOT');
  await page.locator('#open-ad-sales').click();
  const ads = page.locator('#ad-minute-market');
  await expect(ads).toContainText('10 min ad block');
  await ads.getByRole('button', { name: 'Find offers' }).click();
  for (const brand of ['Everyday Snacks', 'Precision Equipment']) {
    await ads.locator('.minute-offer').filter({ hasText: brand }).locator('summary').click();
    await ads.getByRole('button', { name: `Accept ${brand} offer` }).click();
  }
  await expect(ads.locator('.minute-sold')).toHaveCount(2);
  if (process.env.BTS_DESK_SCREENSHOTS) await page.screenshot({ path: '../contrib/focused-desks/ad-sales.png', fullPage: true });
  await page.clock.install();
  const before = await page.locator('#clock').textContent();
  await page.getByRole('button', { name: 'Play', exact: true }).click();
  await page.getByRole('button', { name: 'Pause', exact: true }).click();
  await page.clock.runFor(5000);
  await expect(page.locator('#clock')).toHaveText(before!);
  await page.getByRole('button', { name: 'Continue' }).click();
  await expect(ads.locator('.minute-sold')).toHaveCount(2);
  if (process.env.BTS_WEEKLY_SCREENSHOTS) await page.locator('#ad-sales').screenshot({ path: '../contrib/weekly-planner/booking.png' });
  await page.getByRole('link', { name: '▦ Schedule', exact: true }).click();
  await page.locator('#advance-selected').click();
  await page.locator('#open-review').click();
  await expect(page.locator('#selected-result')).toContainText('Minute sale settlement');
  await expect(page.locator('#selected-result')).toContainText('Young viewers');
  if (process.env.BTS_DESK_SCREENSHOTS) await page.screenshot({ path: '../contrib/focused-desks/review.png', fullPage: true });
  if (process.env.BTS_WEEKLY_SCREENSHOTS) await page.locator('#selected-result').screenshot({ path: '../contrib/weekly-planner/receipt.png' });
  await page.getByRole('link', { name: '▦ Schedule', exact: true }).click();
  await page.locator('#day').selectOption('7');
  await page.locator('[data-station="1"] [data-start-minute="11287"]').click();
  await expect(page.locator('#selected-status')).not.toContainText('RETURNING TO REGULAR SLOT');
  await page.locator('#day').selectOption('14');
  await page.locator('[data-station="1"] [data-start-minute="21240"]').click();
  await expect(page.locator('#selected-status')).toContainText('RETURNING TO REGULAR SLOT');
  await page.screenshot({ path: testInfo.outputPath('returning-slot.png') });
});

test('drag onto the grid, move a draft and preserve its interval after a conflicting drop', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1100 });
  await page.goto('/#planning');
  await expect(page.locator('#status')).toContainText('WASM READY');
  const lane = page.locator('[data-station="1"]');
  await page.locator('.series-box > summary').click();
  const source = page.locator('[data-programme="1"]');
  await dragAcrossConsole(page, source, lane.locator('[data-tick="72"]'));
  const airing = lane.locator('[data-airing]');
  await expect(airing).toHaveAttribute('data-start', '72');
  await dragAcrossConsole(page, airing, lane.locator('[data-tick="76"]'));
  await expect(airing).toHaveAttribute('data-start', '76');
  await page.locator('[data-programme="2"]').click();
  await expect(page.locator('#desk-tape')).toContainText('Workshop Science');
  await dragAcrossConsole(page, page.locator('#desk-tape'), lane.locator('[data-tick="80"]'));
  await expect(lane.locator('[data-start="80"]')).toHaveCount(1);
  const first = lane.locator('[data-start="76"]');
  // 19:45 is free at its start, but a 30-minute show extends past 20:00.
  await dragAcrossConsole(page, first, lane.locator('[data-tick="79"]'));
  await expect(page.getByRole('alert')).toContainText('overlap');
  await expect(page.locator('#placement-preview')).toContainText('Workshop Science');
  await expect(first).toHaveCount(1);
});

test('narrow agenda supports keyboard placement from the tape desk', async ({ page }) => {
  await page.setViewportSize({ width: 420, height: 850 });
  await page.goto('/#planning');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.getByRole('button', { name: 'Open tape shelf' }).click();
  await page.locator('.series-box > summary').click();
  await page.locator('[data-programme="1"]').press('Enter');
  await page.locator('#place').press('Enter');
  const agenda = page.locator('#day-agenda');
  await expect(agenda).toContainText('Saturday · Day 1');
  await agenda.getByRole('button', { name: /18:00 · Your channel · Neighbourhood Laughs/ }).click();
  await expect(page.locator('#selected-name')).toHaveText('Neighbourhood Laughs');
  await expect(agenda.getByRole('link', { name: 'Open parallel timeline' })).toBeVisible();
  if (process.env.BTS_WEEKLY_SCREENSHOTS) await agenda.screenshot({ path: '../contrib/weekly-planner/agenda.png' });
});

test('weekly preview, exceptions and placement replay in native Rust and Wasm', async ({ page }) => {
  const native = JSON.parse(execFileSync('cargo', ['run','--locked','--quiet','-p','bts-web','--example','weekly_planner','--','--sequence'], { cwd: root, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 }));
  await page.route('**/__test__/bts_web.js', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web.js'), contentType: 'text/javascript' }));
  await page.route('**/__test__/bts_web_bg.wasm', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web_bg.wasm'), contentType: 'application/wasm' }));
  await page.goto('/#planning');
  const actual = await page.evaluate(async requests => {
    // @ts-expect-error generated bindings are served by test routes
    const { default: init, WebSession } = await import('/__test__/bts_web.js');
    await init();
    const session = new WebSession();
    return requests.map((command: object) => {
      const response = JSON.parse(session.dispatch(JSON.stringify({api_version: 8,command})));
      if (response.status !== 'ok') throw new Error(response.error.code);
      const report = response.report;
      return { review: report.weekly_review, airings: report.schedule.filter((a: {station:number}) => a.station === 1), calendar: report.first_weekday };
    });
  }, native.requests);
  expect(actual).toEqual(native.responses);
});

test('set a recurring autosell limit and see the signed sales after broadcast', async ({ page }, testInfo) => {
  await page.goto('/#planning');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.getByRole('link', { name: 'Regular slot' }).click();
  const weekly = page.locator('#weekly-planner');
  await weekly.getByLabel('Weekly programme').selectOption('1');
  await weekly.getByLabel('Regular airtime').fill('18:00');
  await weekly.getByLabel('Default ad minutes').fill('15');
  await weekly.getByRole('button', { name: 'Preview weekly slots' }).click();
  await weekly.getByRole('button', { name: 'Confirm weekly schedule' }).click();
  await page.locator('#open-ad-sales').click();
  const ads = page.locator('#ad-minute-market');
  await ads.getByText('Autosell this run').click();
  await ads.getByLabel('Minimum Funds per minute').fill('1');
  await ads.getByLabel('Maximum minutes sold').fill('10');
  await ads.getByRole('button', { name: 'Save autosell limits' }).click();
  await expect(ads).toContainText('Autosell on for this run');
  await ads.screenshot({ path: testInfo.outputPath('autosell-policy.png') });
  await page.getByRole('link', { name: '▦ Schedule', exact: true }).click();
  await page.locator('#advance-selected').click();
  await page.locator('#open-review').click();
  await expect(page.locator('#selected-result')).toContainText('Minute sale settlement');
  await expect(page.locator('#selected-result')).toContainText('Autosell');
  await expect(page.locator('#selected-result')).toContainText(/· ([1-9]|10) sold ·/);
});
