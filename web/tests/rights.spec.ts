import { pickTime } from './planner-helpers';
import { expect, test } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';

const root = resolve(import.meta.dirname, '../..');

test('assess, acquire, air, resell and see a rival acquisition', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  await page.goto('/#planning');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.getByRole('link', { name: /Rights market/ }).click();
  const market = page.locator('#rights-market');
  const corner = market.locator('article').filter({ has: page.getByRole('heading', { name: 'The Corner Shop' }) });
  const observatory = market.locator('article').filter({ has: page.getByRole('heading', { name: 'Beyond the Observatory' }) });
  await market.getByRole('button', { name: /The Corner Shop/ }).first().click();
  const baseline = await corner.locator('.rights-forecast').textContent();
  await corner.getByRole('spinbutton', { name: 'Insight for assessment' }).fill('6');
  await corner.getByRole('button', { name: 'Commission assessment' }).click();
  await corner.getByRole('button', { name: 'Advance to assessment' }).click();
  await expect(corner.locator('.rights-forecast')).not.toHaveText(baseline!);
  await expect(market.locator('#rights-news')).toContainText('Assessment ready');
  if (process.env.BTS_RIGHTS_SCREENSHOTS) await market.screenshot({ path: '../contrib/rights-market/market.png' });
  await corner.getByRole('button', { name: /Buy rights/ }).click();
  await expect(corner).toContainText('In your library');
  await corner.getByRole('button', { name: 'Open in programme library' }).click();
  await pickTime(page, 72, 0);
  await page.locator('#place').click();
  await page.getByRole('link', { name: /Rights market/ }).click();
  await corner.getByRole('button', { name: 'List for resale' }).click();
  await expect(corner.getByRole('button', { name: /Sell rights/ })).toBeDisabled();
  await corner.getByRole('button', { name: /Review blocking airing/ }).click();
  await page.locator('#open-ad-sales').click();
  const ads = page.locator('#ad-minute-market');
  await ads.getByRole('button', { name: 'Set ad block' }).click();
  await ads.getByRole('button', { name: 'Find offers' }).click();
  await ads.locator('.minute-offer').first().locator('summary').click();
  await ads.getByRole('button', { name: /^Accept/ }).click();
  await page.getByRole('link', { name: '▦ Schedule', exact: true }).click();
  await page.locator('#advance-selected').click();
  await page.locator('#open-review').click();
  await expect(page.locator('#selected-result')).toContainText('Minute sale settlement');
  await page.getByRole('link', { name: /Rights market/ }).click();
  await expect(corner.getByRole('button', { name: /Sell rights/ })).toBeEnabled();
  await corner.getByRole('button', { name: /Sell rights/ }).click();
  await expect(page.locator('#results-list')).toContainText('The Corner Shop');
  await expect(corner).toContainText('On the shelf at Channel Two');
  await expect(page.locator('[data-programme="60000"]')).toHaveCount(0);
  await expect(page.locator('#ledger')).toContainText('Rights resale');
  for (let day = 0; day < 3; day++) await page.locator('#advance-day').click();
  await expect(market).toContainText('Night Chase');
  await market.getByRole('navigation', { name: 'Title index' }).getByRole('button', { name: /Beyond the Observatory/ }).click();
  await expect(observatory).toContainText('On the shelf at Channel Two');
  await expect(market.locator('#rights-news')).toContainText('Rival acquired the rights');
  if (process.env.BTS_RIGHTS_SCREENSHOTS) await market.screenshot({ path: '../contrib/rights-market/industry-wire.png' });
});

test('rights acquisition, assessment and resale match native execution', async ({ page }) => {
  const native = JSON.parse(execFileSync('cargo', ['run', '--locked', '--quiet', '-p', 'bts-web', '--example', 'rights_market', '--', '--replay'], { cwd: root, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 }));
  await page.route('**/__test__/bts_web.js', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web.js'), contentType: 'text/javascript' }));
  await page.route('**/__test__/bts_web_bg.wasm', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web_bg.wasm'), contentType: 'application/wasm' }));
  await page.goto('/#planning');
  const actual = await page.evaluate(async requests => {
    // @ts-expect-error generated bindings are only served by test routes
    const { default: init, WebSession } = await import('/__test__/bts_web.js');
    await init();
    const session = new WebSession();
    const inspect = (command: object) => JSON.parse(session.dispatch(JSON.stringify({ api_version: 8, command })));
    let previous = inspect({ type: 'inspect' }).report;
    return requests.map((request: { command: { type: string; programme?: number; token?: string } }) => {
      const command = request.command;
      if (command.type === 'buy_rights' || command.type === 'accept_resale') {
        const title = previous.rights_market.titles.find((t: { programme: number }) => t.programme === command.programme);
        command.token = (command.type === 'buy_rights' ? title.acquisition : title.resale).token;
      }
      const { status, report } = inspect(command);
      if (status !== 'ok') throw new Error('Rights replay rejected a command.');
      previous = report;
      return { market: report.rights_market, cash: report.cash, finances: report.finances, catalogue: report.catalogue, schedule: report.schedule };
    });
  }, native.requests);
  const normalize = (rows: unknown) => JSON.parse(JSON.stringify(rows, (key, value) => key === 'token' ? '<session token>' : value));
  expect(normalize(actual)).toEqual(normalize(native.results));
});
