import { expect, test } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';
import { readFileSync, readdirSync } from 'node:fs';
const root = resolve(import.meta.dirname, '../..');

test('plan a varied evening, move/remove broadcasts, book a selected future airing, run and review', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  const failures: string[] = []; page.on('pageerror', error => failures.push(error.message));
  await page.goto('/'); await expect(page.locator('#status')).toContainText('WASM READY');
  await expect(page.locator('#cash')).toHaveText('20,000');
  await expect(page.locator('#timeline [data-station]')).toHaveCount(2);
  await expect(page.locator('.timeline-cell')).toHaveCount(96);
  await expect(page.locator('#next-airing')).toContainText('Nothing else planned');
  // 30-minute comedy at 18:00; explicit dimensions come from Rust's returned interval.
  await page.locator('[data-programme="1"]').click(); await page.locator('#place-time').selectOption('72'); await page.locator('#place').click();
  const first = await page.locator('#selected-id').textContent(); const firstId = Number(first!.match(/#(\d+)/)![1]);
  const firstBlock = page.locator(`[data-airing="${firstId}"]`);
  await expect(firstBlock).toHaveAttribute('data-start', '72'); await expect(firstBlock).toHaveAttribute('data-end', '74');
  await expect(page.locator('#next-airing')).toContainText(`#${firstId}`);
  // A 45-minute science programme follows; placement time advances to the returned end.
  await page.locator('[data-programme="2"]').click(); await expect(page.locator('#place-time')).toHaveValue('74'); await page.locator('#place').click();
  const secondId = Number((await page.locator('#selected-id').textContent())!.match(/#(\d+)/)![1]);
  await expect(page.locator('#selected-window')).toContainText('18:30–19:15 · 45 min');
  await expect(page.locator('#known-details')).toContainText('Information depth');
  await expect(page.locator('#known-details')).not.toContainText('Editorial rigour');
  await page.locator('#move-time').selectOption('73'); await page.locator('#move').click();
  await expect(page.getByRole('alert')).toContainText('overlap');
  await expect(page.locator(`[data-airing="${secondId}"]`)).toHaveAttribute('data-start', '74');
  await page.locator('#move-time').selectOption('78'); await page.locator('#move').click();
  await expect(page.locator(`[data-airing="${secondId}"]`)).toHaveAttribute('data-start', '78');
  await page.locator('#remove').click(); await expect(page.locator(`[data-airing="${secondId}"]`)).toHaveCount(0);
  await page.locator('#place-time').selectOption('74'); await page.locator('#place').click();
  const replacementId = Number((await page.locator('#selected-id').textContent())!.match(/#(\d+)/)![1]); expect(replacementId).not.toBe(secondId);
  // Film leaves a visible 45-minute off-air gap and occupies 90 minutes.
  await page.locator('[data-programme="5"]').click(); await page.locator('#place-time').selectOption('80'); await page.locator('#place').click();
  const movieId = Number((await page.locator('#selected-id').textContent())!.match(/#(\d+)/)![1]);
  await expect(page.locator('#selected-window')).toContainText('20:00–21:30 · 90 min');
  expect((await page.locator(`[data-airing="${movieId}"]`).boundingBox())!.width).toBeGreaterThan((await firstBlock.boundingBox())!.width * 2.8);
  await expect(page.locator('#next-airing')).toContainText(`#${firstId}`);
  await expect(page.locator('#selected-id')).toContainText(`#${movieId}`);
  await page.locator('#advertiser').selectOption('1'); await page.locator('#set-advertiser').click(); await page.locator('#quote').click();
  await expect(page.locator('#offer')).toContainText(`Offer for airing #${movieId}`);
  await expect(page.locator('#offer')).toContainText('Forecast from published audience estimates');
  await firstBlock.click();
  await expect(page.locator('#offer')).toContainText('pending offer belongs to another airing');
  await expect(page.locator('#accept')).toHaveCount(0);
  await page.locator(`[data-airing="${movieId}"]`).click();
  await expect(page.locator('#offer')).toContainText(`Offer for airing #${movieId}`);
  await expect(page.locator('#next-airing')).toContainText(`#${firstId}`);
  await page.screenshot({ path: '/tmp/bts-planning-workspace.png', fullPage: true });
  await page.locator('#accept').click(); await expect(page.locator('#selected-contract')).toContainText('Reserved in advertiser escrow');
  await expect(page.locator('#move')).toBeDisabled(); await expect(page.locator('#remove')).toBeDisabled(); await expect(page.locator('#advertiser')).toBeDisabled();
  // Global advancement runs the next airing, not the selected movie.
  await page.locator('#advance-next').click(); await expect(page.locator('#clock')).toHaveText('Day 1 · 18:30');
  await expect(page.locator('#selected-id')).toContainText(`#${movieId}`);
  await expect(page.locator('#next-airing')).toContainText(`#${replacementId}`);
  await expect(page.locator('#selected-contract')).not.toContainText('Settled:');
  await page.locator('#advance-next').click(); await expect(page.locator('#clock')).toHaveText('Day 1 · 19:15');
  await page.locator('#advance-next').click(); await expect(page.locator('#clock')).toHaveText('Day 1 · 21:30');
  await expect(page.locator('#selected-contract')).toContainText('Settled: paid');
  await expect(page.locator('#selected-result')).toContainText(`Broadcast result · airing #${movieId}`);
  await expect(page.locator('#selected-result')).toContainText('Provisional expected counts sampled at Day 1 · 20:00');
  await expect(page.locator('#finance-summary')).toContainText('300');
  await expect(page.locator('#results-list tbody tr')).toHaveCount(3);
  await page.locator('#results-list').getByRole('button', { name: `#${firstId} · Neighbourhood Laughs`, exact: true }).click();
  await expect(page.locator('#selected-result')).toContainText(`Broadcast result · airing #${firstId}`);
  await expect(page.locator('#selected-result')).toContainText('Cost 150');
  await page.locator('#advance-day').click(); await expect(page.locator('#clock')).toHaveText('Day 2 · 00:00');
  await expect(page.locator('#finance-summary')).toContainText('300');
  await page.locator('#reset').click(); await expect(page.locator('#cash')).toHaveText('20,000'); await expect(page.locator('#clock')).toHaveText('Day 1 · 00:00');
  await expect(page.locator('#results-list')).toContainText('No completed broadcasts'); await expect(page.locator('#selection-empty')).toBeVisible();
  expect(failures).toEqual([]);
});

test('day navigation, gap placement, on-air progression and mobile controls', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/'); await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('#day').selectOption('1'); await page.locator('#jump-evening').click();
  await page.getByRole('button', { name: 'Choose placement Day 2 · 18:00', exact: true }).click();
  await expect(page.locator('#place-day')).toHaveValue('1'); await expect(page.locator('#place-time')).toHaveValue('72');
  await page.locator('#place').click(); await expect(page.locator('#selected-window')).toContainText('Day 2 · 18:00–18:30');
  await page.locator('#place-day').selectOption('0'); await page.locator('#place-time').selectOption('0'); await page.locator('#place').click();
  await page.locator('#advance-tick').click(); await expect(page.locator('#clock')).toHaveText('Day 1 · 00:15');
  await expect(page.locator('#selected-status')).toContainText('ON AIR'); await expect(page.locator('#selected-result')).toContainText('available at scheduled completion');
  await expect(page.locator('#move')).toBeDisabled();
  await page.locator('#advance-next').click(); await expect(page.locator('#clock')).toHaveText('Day 1 · 00:30');
  await expect(page.locator('#selected-result')).toContainText('Completed.');
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
});

test('actual Wasm protocol matches native planning, booking, tick accounting and results', async ({ page }) => {
  const native = JSON.parse(execFileSync('cargo', ['run', '--locked', '--quiet', '-p', 'bts-web', '--example', 'protocol_fixture'], { cwd: root, encoding: 'utf8' }));
  await page.route('**/__test__/bts_web.js', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web.js'), contentType: 'text/javascript' }));
  await page.route('**/__test__/bts_web_bg.wasm', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web_bg.wasm'), contentType: 'application/wasm' }));
  const wasmAsset = readdirSync(resolve(root, 'web/dist/assets')).find(file => file.endsWith('.wasm'))!;
  expect(readFileSync(resolve(root, `web/dist/assets/${wasmAsset}`))).toEqual(readFileSync(resolve(root, 'web/pkg/bts_web_bg.wasm')));
  await page.goto('/'); await expect(page.locator('#status')).toContainText('WASM READY');
  const result = await page.evaluate(async () => {
    // @ts-expect-error test-only routes serve generated bindings, not a product inspector
    const { default: init, WebSession } = await import('/__test__/bts_web.js'); await init();
    const a = new WebSession(); const b = new WebSession();
    const send = (s: { dispatch: (s: string) => string }, command: object) => JSON.parse(s.dispatch(JSON.stringify({ api_version: 8, command })));
    const reports = [send(a, { type: 'inspect' }), send(a, { type: 'place', start_tick: 72, programme: 1, advertiser: 1 })];
    const id = reports[1].report.schedule.find((a: { station: number }) => a.station === 1).airing_id;
    reports.push(send(a, { type: 'request_offer', airing_id: id })); const token = reports[2].report.offer.offer_id;
    const beforeInvalid = send(a, { type: 'inspect' });
    const invalid = send(a, { type: 'place', start_tick: 73, programme: 1, advertiser: null });
    const afterInvalid = send(a, { type: 'inspect' });
    const malformed = JSON.parse(a.dispatch('{bad'));
    const wide = JSON.parse(a.dispatch('{"api_version":8,"command":{"type":"remove","airing_id":4294967296}}'));
    reports.push(send(a, { type: 'accept_offer', airing_id: id, offer_id: token }));
    reports.push(send(a, { type: 'advance', to_tick: 74 }));
    const beforeLocked = send(a, { type: 'inspect' });
    const locked = send(a, { type: 'move', airing_id: id, start_tick: 80 }); const afterLocked = send(a, { type: 'inspect' });
    send(b, { type: 'place', start_tick: 72, programme: 1, advertiser: 1 }); send(b, { type: 'request_offer', airing_id: id });
    const crossSession = send(b, { type: 'accept_offer', airing_id: id, offer_id: token });
    const bState = send(b, { type: 'inspect' });
    const reset = send(a, { type: 'reset' });
    const placed = send(a, { type: 'place', start_tick: 72, programme: 1, advertiser: 1 });
    const newid = placed.report.schedule.find((a: { station: number }) => a.station === 1).airing_id;
    const staleToken = send(a, { type: 'request_offer', airing_id: newid }).report.offer.offer_id;
    send(a, { type: 'move', airing_id: newid, start_tick: 76 });
    const staleBefore = send(a, { type: 'inspect' }); const stale = send(a, { type: 'accept_offer', airing_id: newid, offer_id: staleToken }); const staleAfter = send(a, { type: 'inspect' });
    const old = send(a, { type: 'remove', airing_id: id });
    const version = JSON.parse(a.dispatch('{"api_version":1,"command":{"type":"inspect"}}'));
    a.free(); b.free();
    return { reports, invalid, beforeInvalid, afterInvalid, malformed, wide, locked, beforeLocked, afterLocked, crossSession, bState, reset, newid, id, stale, staleBefore, staleAfter, old, version };
  });
  function compare(actual: unknown, expected: unknown): void {
    if (typeof expected === 'number') { expect(actual).toBeCloseTo(expected, 8); return; }
    if (Array.isArray(expected)) { expect(Array.isArray(actual)).toBe(true); expect((actual as unknown[]).length).toBe(expected.length); expected.forEach((value, i) => compare((actual as unknown[])[i], value)); return; }
    if (expected !== null && typeof expected === 'object') {
      expect(Object.keys(actual as object).sort()).toEqual(Object.keys(expected).sort());
      for (const [key, value] of Object.entries(expected)) { if (key === 'offer_id' || key === 'token') { expect(typeof (actual as Record<string, unknown>)[key]).toBe('string'); continue; } compare((actual as Record<string, unknown>)[key], value); } return;
    }
    expect(actual).toEqual(expected);
  }
  compare(result.reports, native);
  expect(result.invalid.error.code).toBe('overlap'); expect(result.afterInvalid).toEqual(result.beforeInvalid);
  expect(result.malformed.error.code).toBe('malformed_command'); expect(result.wide.error.code).toBe('malformed_command');
  expect(result.locked.error.code).toBe('already_started'); expect(result.beforeLocked).toEqual(result.afterLocked);
  expect(result.crossSession.error.code).toBe('stale_offer'); expect(result.bState.report.now_tick).toBe(0); expect(result.bState.report.contracts).toEqual([]);
  expect(result.reset.report.cash).toBe('20000'); expect(result.reset.report.now_tick).toBe(0); expect(result.reset.report.results).toEqual([]);
  expect(result.newid).not.toBe(result.id); expect(result.stale.error.code).toBe('stale_offer'); expect(result.staleBefore).toEqual(result.staleAfter);
  expect(result.old.error.code).toBe('unknown_airing'); expect(result.version.error.code).toBe('unsupported_version');
});

test('initialization failure is visible and planning controls stay unavailable', async ({ page }) => {
  await page.route('**/*.wasm', route => route.abort()); await page.goto('/');
  await expect(page.locator('#status')).toContainText('initialization failed'); await expect(page.getByRole('alert')).toContainText('Reload to retry'); await expect(page.locator('#controls')).toBeHidden();
});
