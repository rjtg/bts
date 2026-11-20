import { goToDesk } from './station-directory';
import { pickTime } from './planner-helpers';
import { expect, test } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';

const root = resolve(import.meta.dirname, '../..');

test('sell one block to two brands, trim it and settle on air', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  await page.goto('/#planning');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('.series-box > summary').click();
  await page.locator('[data-programme="1"]').click();
  await pickTime(page, 72);
  await page.locator('#place').click();
  await page.locator('#open-ad-sales').click();
  const market = page.locator('#ad-minute-market');
  await market.getByRole('spinbutton', { name: 'Minutes to list' }).fill('16');
  await market.getByRole('button', { name: 'Set ad block' }).click();
  await expect(market).toContainText('46 min runtime · until 18:46');
  await market.getByRole('button', { name: 'Find offers' }).click();
  await expect(market.locator('.minute-offer')).toHaveCount(2);
  if (process.env.BTS_MARKET_SCREENSHOTS) await market.screenshot({ path: '../contrib/ad-minute-market/offers.png' });
  for (const brand of ['Everyday Snacks', 'Precision Equipment']) {
    const offer = market.locator('.minute-offer').filter({ hasText: brand });
    await offer.locator('summary').click();
    await offer.getByRole('button', { name: `Accept ${brand} offer` }).click();
  }
  await expect(market.locator('.minute-sold')).toHaveCount(2);
  await market.getByRole('button', { name: 'Remove unsold minutes' }).click();
  await expect(market).toContainText('0 min available');
  await expect(market).toContainText('min runtime · until 18:');
  await goToDesk(page, '▦ Schedule');
  await page.locator('#advance-selected').click();
  await page.locator('#open-review').click();
  const result = page.locator('#selected-result');
  await expect(result).toContainText('Minute sale settlement');
  await expect(result).toContainText(/(\d+) sold · \1 aired · [1-9]/);
  if (process.env.BTS_MARKET_SCREENSHOTS) await result.screenshot({ path: '../contrib/ad-minute-market/settlement.png' });
});

test('minute offers and settlements match the native protocol', async ({ page }) => {
  const native = JSON.parse(execFileSync('cargo', ['run', '--locked', '--quiet', '-p', 'bts-web', '--example', 'ad_minute_market', '--', '--replay'], { cwd: root, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 }));
  await page.route('**/__test__/bts_web.js', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web.js'), contentType: 'text/javascript' }));
  await page.route('**/__test__/bts_web_bg.wasm', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web_bg.wasm'), contentType: 'application/wasm' }));
  await page.goto('/#planning');
  const actual = await page.evaluate(async requests => {
    // @ts-expect-error generated bindings are only served by test routes
    const { default: init, WebSession } = await import('/__test__/bts_web.js');
    await init();
    const session = new WebSession();
    let token = '';
    return requests.map((request: { command: { type: string; token?: string } }) => {
      if (request.command.type === 'accept_minute_offer') request.command.token = token;
      const { status, report } = JSON.parse(session.dispatch(JSON.stringify(request)));
      if (status !== 'ok') throw new Error('Replay rejected a market command.');
      if (report.minute_offers) token = report.minute_offers.token;
      return { cash: report.cash, sales: report.minute_sales, batch: report.minute_offers, results: report.results };
    });
  }, native.requests);
  // Exact comparisons on credits, identities, minutes and other discrete fields.
  const normalize = (rows: unknown) => JSON.parse(JSON.stringify(rows, (key, value) => {
    if (key === 'token') return '<session token>';
    if (typeof value === 'number' && !Number.isInteger(value)) return Number(value.toFixed(8));
    return value;
  }));
  expect(normalize(actual)).toEqual(normalize(native.results));
});

test('two paid campaigns reach the industry wire at the weekly review', async ({ page }, testInfo) => {
  test.setTimeout(90_000);
  await page.goto('/#planning');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('.series-box > summary').click();
  await page.locator('[data-programme="1"]').click();
  for (const day of [0, 1]) {
    await pickTime(page, 72, day);
    await page.locator('#place').click();
    await page.locator('#open-ad-sales').click();
    const ads = page.locator('#ad-minute-market');
    await ads.getByRole('spinbutton', { name: 'Minutes to list' }).fill('15');
    await ads.getByRole('button', { name: 'Set ad block' }).click();
    await ads.getByRole('button', { name: 'Find offers' }).click();
    const offer = ads.locator('.minute-offer').filter({ hasText: 'Everyday Snacks' });
    await offer.locator('summary').click();
    await offer.getByRole('button', { name: 'Accept Everyday Snacks offer' }).click();
    await goToDesk(page, '▦ Schedule');
    await page.locator('#advance-selected').click();
  }
  for (let day = 1; day < 7; day++) await page.locator('#advance-day').click();
  await expect(page.locator('#clock')).toContainText('Day 8');
  await goToDesk(page, '▱ Rights market');
  const wire = page.locator('#rights-news');
  await expect(wire).toContainText(/Everyday Snacks · Bidding more (competitively|cautiously)/);
  await wire.screenshot({ path: testInfo.outputPath('brand-review-wire.png') });
});
