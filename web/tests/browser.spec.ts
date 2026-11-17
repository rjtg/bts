import { expect, test } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';
import { readFileSync, readdirSync } from 'node:fs';

const root = resolve(import.meta.dirname, '../..');

test('static bundle: initialize, plan, quote, accept, air, settle and reset', async ({ page }) => {
  const failures: string[] = [];
  page.on('pageerror', error => failures.push(error.message));
  await page.goto('/');
  await expect(page.getByRole('status')).toContainText('Ready');
  await expect(page.locator('#cash')).toHaveText('20,000');
  await page.locator('#advance').click();
  await expect(page.getByRole('alert')).toContainText('plan_required');
  await expect(page.locator('#cash')).toHaveText('20,000');
  await page.locator('#programme').selectOption('1');
  await page.locator('#advertiser').selectOption('1');
  await page.locator('#plan').click();
  await page.locator('#quote').click();
  await expect(page.locator('#offer')).toContainText('Frozen terms');
  await page.locator('#accept').click();
  await expect(page.locator('#contracts')).toContainText('Reserved in escrow');
  await page.locator('#programme').selectOption('2');
  await page.locator('#plan').click();
  await expect(page.getByRole('alert')).toContainText('booking_locked');
  await page.locator('#advance').click();
  await expect(page.locator('#contracts')).toContainText('Settled: paid');
  await expect(page.locator('#results')).toContainText('Neighbourhood Laughs');
  await expect(page.locator('#results')).toContainText('Provisional exact expected counts');
  await expect(page.locator('#position')).toContainText('21:00');
  expect(await page.locator('#cash').textContent()).not.toBe('20,000');
  await page.locator('#reset').click();
  await expect(page.locator('#cash')).toHaveText('20,000');
  await expect(page.locator('#results')).toContainText('No broadcasts yet');
  await expect(page.locator('#contracts')).toBeEmpty();
  await expect(page.locator('#position')).toContainText('18:00');
  expect(failures).toEqual([]);
});

test('actual Wasm protocol matches native sequence; stale/reset/isolation/errors remain atomic', async ({ page }) => {
  const native = JSON.parse(execFileSync('cargo', ['run', '--locked', '--quiet', '-p', 'bts-web', '--example', 'protocol_fixture'], { cwd: root, encoding: 'utf8' }));
  // Test-only routes serve generated bindings. No inspector or test bridge ships in the page.
  await page.route('**/__test__/bts_web.js', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web.js'), contentType: 'text/javascript' }));
  await page.route('**/__test__/bts_web_bg.wasm', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web_bg.wasm'), contentType: 'application/wasm' }));
  const wasmAsset = readdirSync(resolve(root, 'web/dist/assets')).find(file => file.endsWith('.wasm'))!;
  expect(readFileSync(resolve(root, `web/dist/assets/${wasmAsset}`))).toEqual(readFileSync(resolve(root, 'web/pkg/bts_web_bg.wasm')));
  await page.goto('/');
  await expect(page.getByRole('status')).toContainText('Ready');
  const result = await page.evaluate(async () => {
    // @ts-expect-error the bindings are supplied by the test's HTTP route
    const { default: init, WebSession } = await import('/__test__/bts_web.js');
    await init();
    const a = new WebSession(); const b = new WebSession();
    const send = (s: { dispatch: (s: string) => string }, command: object) => JSON.parse(s.dispatch(JSON.stringify({ api_version: 1, command })));
    const reports = [send(a, { type: 'inspect' }), send(a, { type: 'plan', slot: 0, programme: 1, advertiser: 1 }), send(a, { type: 'request_offer' })];
    const token = reports[2].report.offer.offer_id;
    const malformed = JSON.parse(a.dispatch('{bad'));
    const beforeInvalid = send(a, { type: 'inspect' });
    const invalid = send(a, { type: 'plan', slot: 0, programme: 999, advertiser: 1 });
    const afterInvalid = send(a, { type: 'inspect' });
    reports.push(send(a, { type: 'accept_offer', offer_id: token }));
    reports.push(send(a, { type: 'advance' }));
    const bBefore = send(b, { type: 'inspect' });
    send(b, { type: 'plan', slot: 0, programme: 1, advertiser: 1 });
    send(b, { type: 'request_offer' });
    const crossSession = send(b, { type: 'accept_offer', offer_id: token });
    const reset = send(a, { type: 'reset' });
    send(a, { type: 'plan', slot: 0, programme: 1, advertiser: 1 });
    const staleToken = send(a, { type: 'request_offer' }).report.offer.offer_id;
    send(a, { type: 'plan', slot: 0, programme: 2, advertiser: 2 });
    const beforeStale = send(a, { type: 'inspect' });
    const stale = send(a, { type: 'accept_offer', offer_id: staleToken });
    const afterStale = send(a, { type: 'inspect' });
    send(a, { type: 'reset' });
    const oldAfterReset = send(a, { type: 'accept_offer', offer_id: staleToken });
    const version = JSON.parse(a.dispatch('{"api_version":999,"command":{"type":"inspect"}}'));
    const precision = BigInt('18446744073709551615').toString();
    a.free(); b.free();
    return { reports, malformed, invalid, beforeInvalid, afterInvalid, bBefore, crossSession, reset, stale, beforeStale, afterStale, oldAfterReset, version, precision };
  });
  function compare(actual: unknown, expected: unknown): void {
    if (typeof expected === 'number') { expect(actual).toBeCloseTo(expected, 8); return; }
    if (Array.isArray(expected)) { expect(Array.isArray(actual)).toBe(true); expect((actual as unknown[]).length).toBe(expected.length); expected.forEach((value, i) => compare((actual as unknown[])[i], value)); return; }
    if (expected !== null && typeof expected === 'object') {
      expect(Object.keys(actual as object).sort()).toEqual(Object.keys(expected).sort());
      for (const [key, value] of Object.entries(expected)) {
        if (key === 'offer_id') { expect(typeof (actual as Record<string, unknown>)[key]).toBe('string'); continue; }
        compare((actual as Record<string, unknown>)[key], value);
      }
      return;
    }
    expect(actual).toEqual(expected);
  }
  compare(result.reports, native);
  expect(result.malformed.error.code).toBe('malformed_command');
  expect(result.invalid.error.code).toBe('unknown_programme');
  expect(result.afterInvalid).toEqual(result.beforeInvalid);
  expect(result.stale.error.code).toBe('stale_offer');
  expect(result.afterStale).toEqual(result.beforeStale);
  expect(result.crossSession.error.code).toBe('stale_offer');
  expect(result.oldAfterReset.error.code).toBe('stale_offer');
  expect(result.version.error.code).toBe('unsupported_version');
  compare(result.reset, result.bBefore);
  expect(result.precision).toBe('18446744073709551615');
});

test('initialization failure is visible and controls stay unavailable', async ({ page }) => {
  await page.route('**/*.wasm', route => route.abort());
  await page.goto('/');
  await expect(page.getByRole('status')).toContainText('initialization failed');
  await expect(page.getByRole('alert')).toContainText('Reload to retry');
  await expect(page.locator('#controls')).toBeHidden();
});
