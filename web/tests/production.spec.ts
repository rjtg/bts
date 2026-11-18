import { expect, test } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';
const root = resolve(import.meta.dirname, '../..');

test('produce, advertise and broadcast an original, earn pooled Sway and spend across people on participation and cooperation', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  const failures: string[] = []; page.on('pageerror', error => failures.push(error.message));
  await page.goto('/'); await expect(page.getByRole('status')).toContainText('WASM READY');
  await page.getByRole('link', { name: '▣ Production', exact: true }).click();
  await page.locator('#production-name').fill('Workshop Conversations'); await page.locator('#create-production').click();
  const first = page.locator('#production-1');
  await expect(first).toContainText('DRAFT'); await expect(first).toContainText('620 in fees'); await expect(page.locator('#cash')).toHaveText('20,000');
  await page.locator('#request-choice-1').selectOption('1'); await first.getByRole('button', { name: 'Book production #1', exact: true }).click();
  await expect(first).toContainText('BOOKED'); await expect(page.locator('#cash')).toHaveText('19,000'); await expect(page.locator('#sway-panel')).toContainText('not fulfilled yet');
  await first.getByRole('button', { name: /Run through production #1/ }).click();
  await expect(page.locator('#clock')).toHaveText('Day 1 · 13:00'); await expect(first).toContainText('COMPLETED');
  await expect(page.locator('#sway-panel')).toContainText('Fulfilled at Day 1 · 11:00'); await expect(page.locator('#sway-panel')).toContainText('Pooled Sway · 4'); await expect(page.locator('#cash')).toHaveText('19,000');
  await first.getByRole('button', { name: 'Show in programme library' }).click();
  await expect(page.locator('#place-name')).toHaveText('Workshop Conversations'); await expect(page.locator('[data-programme="8"]')).toContainText('Your original');
  await page.locator('#place-day').selectOption('0'); await page.locator('#place-time').selectOption('72'); await page.locator('#place').click();
  await expect(page.locator('#selected-name')).toHaveText('Workshop Conversations');
  await page.locator('#advertiser').selectOption('2'); await page.locator('#set-advertiser').click(); await page.locator('#quote').click(); await page.locator('#accept').click(); await page.locator('#advance-next').click();
  await expect(page.locator('#clock')).toHaveText('Day 1 · 18:45'); await expect(page.locator('#selected-result')).toContainText('Completed.'); await expect(page.locator('#selected-contract')).toContainText('Settled: paid');
  await page.getByRole('link', { name: '▣ Production', exact: true }).click(); await page.locator('#production-new').click(); await page.locator('#production-name').fill('Neighbourhood Conversations');
  await page.locator('#production-host').selectOption('2'); await page.locator('#production-guest-one').selectOption('5'); await page.locator('#production-guest-two').selectOption('6'); await page.locator('#production-subject').selectOption('everyday_life'); await page.locator('#production-tone').selectOption('conversational');
  await page.locator('#production-day').selectOption('2'); await page.locator('#production-time').selectOption('40'); await page.locator('#create-production').click();
  const second = page.locator('#production-2'); const before = await page.locator('#cash').textContent();
  await second.getByRole('button', { name: 'Book production #2', exact: true }).click(); await expect(page.getByRole('alert')).toContainText('exceptional_participation_required'); await expect(page.locator('#cash')).toHaveText(before!);
  await page.locator('#exceptional-2').selectOption('5'); await page.locator('#smooth-2').check(); await second.getByRole('button', { name: 'Book production #2', exact: true }).click(); await expect(page.locator('#sway-panel')).toContainText('Pooled Sway · 0');
  await expect(second).toContainText('Exceptional participation secured: Jules Hart'); await expect(second).toContainText('Cooperation smoothing applied'); await expect(page.locator('#sway-history')).toContainText('Jules Hart');
  await second.getByRole('button', { name: /Run through production #2/ }).click(); await expect(page.locator('#clock')).toHaveText('Day 3 · 13:00'); await expect(second).toContainText('COMPLETED');
  await page.locator('#production').screenshot({ path: '/tmp/bts-pooled-sway.png' });
  await second.getByRole('button', { name: 'Show in programme library' }).click(); await page.locator('#place-day').selectOption('2'); await page.locator('#place-time').selectOption('72'); await page.locator('#place').click(); await page.locator('#advance-next').click();
  await expect(page.locator('#clock')).toHaveText('Day 3 · 18:45'); await expect(page.locator('#selected-name')).toHaveText('Neighbourhood Conversations'); await expect(page.locator('#selected-result')).toContainText('Completed.'); await expect(page.locator('#results-list tbody tr')).toHaveCount(2);
  await expect(page.locator('#finance-summary')).toContainText('2,000'); await expect(page.locator('#ledger')).toContainText('Recording expense (from escrow)'); expect(failures).toEqual([]);
});

test('conflicting talent is rejected; free drafts can be revised and discarded', async ({ page }) => {
  await page.goto('/'); await expect(page.getByRole('status')).toContainText('WASM READY');
  await page.locator('#create-production').click(); await page.locator('#production-1').getByRole('button', { name: 'Book production #1', exact: true }).click();
  await page.locator('#production-name').fill('Conflicting recording'); await page.locator('#create-production').click();
  const second = page.locator('#production-2'); await second.getByRole('button', { name: 'Book production #2', exact: true }).click(); await expect(page.getByRole('alert')).toContainText('talent_conflict'); await expect(page.locator('#cash')).toHaveText('19,000');
  await second.getByRole('button', { name: 'Edit draft', exact: true }).click(); await page.locator('#production-time').selectOption('44'); await page.locator('#create-production').click(); await expect(second).toContainText('Day 1 · 11:00'); await second.getByRole('button', { name: 'Discard draft', exact: true }).click(); await expect(second).toHaveCount(0); await expect(page.locator('#cash')).toHaveText('19,000');
});

test('production and pooled Sway commands in actual Wasm match the native sequence', async ({ page }) => {
  test.setTimeout(60_000);
  const native = JSON.parse(execFileSync('cargo', ['run', '--locked', '--quiet', '-p', 'bts-web', '--example', 'production_experiment', '--', '--sequence'], { cwd: root, encoding: 'utf8' }));
  await page.route('**/__test__/bts_web.js', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web.js'), contentType: 'text/javascript' }));
  await page.route('**/__test__/bts_web_bg.wasm', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web_bg.wasm'), contentType: 'application/wasm' }));
  await page.goto('/'); await expect(page.getByRole('status')).toContainText('WASM READY');
  const actual = await page.evaluate(async requests => {
    // @ts-expect-error generated bindings are served by test-only routes
    const { default: init, WebSession } = await import('/__test__/bts_web.js'); await init(); const s = new WebSession(); const responses = []; let token = '';
    for (const request of requests) { if (request.command.type === 'accept_offer') request.command.offer_id = token; const response = JSON.parse(s.dispatch(JSON.stringify(request))); responses.push(response); if (response.status === 'ok' && response.report.offer) token = response.report.offer.offer_id; }
    s.free(); return responses;
  }, native.requests);
  function compare(a: unknown, b: unknown): void {
    if (typeof b === 'number') { expect(a).toBeCloseTo(b, 8); return; }
    if (Array.isArray(b)) { expect(Array.isArray(a)).toBe(true); expect((a as unknown[]).length).toBe(b.length); b.forEach((value, i) => compare((a as unknown[])[i], value)); return; }
    if (b !== null && typeof b === 'object') { expect(Object.keys(a as object).sort()).toEqual(Object.keys(b).sort()); for (const [key, value] of Object.entries(b)) { if (key === 'offer_id') continue; compare((a as Record<string, unknown>)[key], value); } return; }
    expect(a).toEqual(b);
  }
  compare(actual, native.responses);
  expect(actual.filter((r: { status: string }) => r.status === 'error').map((r: { error: { code: string } }) => r.error.code)).toEqual(['production_not_ready', 'exceptional_participation_required']);
});

test('a second finite request earns at completion and its pooled reward buys cooperation elsewhere', async ({ page }) => {
  await page.goto('/'); await expect(page.getByRole('status')).toContainText('WASM READY');
  await page.locator('#production-guest-two').selectOption('6'); await page.locator('#production-subject').selectOption('everyday_life');
  await page.locator('#create-production').click(); await page.locator('#request-choice-1').selectOption('2');
  await page.locator('#smooth-1').check(); await page.locator('#production-1').getByRole('button', { name: 'Book production #1', exact: true }).click();
  await expect(page.getByRole('alert')).toContainText('insufficient_sway'); await expect(page.locator('#cash')).toHaveText('20,000');
  await page.locator('#smooth-1').uncheck(); await page.locator('#production-1').getByRole('button', { name: 'Book production #1', exact: true }).click();
  await expect(page.locator('#sway-panel')).toContainText('Pooled Sway · 0');
  await page.locator('#production-1').getByRole('button', { name: /Run through production #1/ }).click();
  await expect(page.locator('#request-2')).toContainText('Fulfilled at Day 1 · 13:00'); await expect(page.locator('#sway-panel')).toContainText('Pooled Sway · 3');
  await page.locator('#production-new').click(); await page.locator('#production-name').fill('A different production'); await page.locator('#production-time').selectOption('60');
  await page.locator('#production-guest-one').selectOption('4'); await page.locator('#production-guest-two').selectOption('5'); await page.locator('#create-production').click();
  await page.locator('#smooth-2').check(); await page.locator('#production-2').getByRole('button', { name: 'Book production #2', exact: true }).click();
  await expect(page.locator('#sway-panel')).toContainText('Pooled Sway · 1'); await expect(page.locator('#production-2')).toContainText('Cooperation smoothing applied');
  await page.locator('#production-2').getByRole('button', { name: /Run through production #2/ }).click(); await expect(page.locator('#sway-panel')).toContainText('Pooled Sway · 1');
});
