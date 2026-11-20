import { goToDesk, openStationDirectory } from './station-directory';
import { pickTime } from './planner-helpers';
import { expect, test, type Page } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';
const root = resolve(import.meta.dirname, '../..');
async function choose(page: Page, seat: number, name: string) {
  await page.locator(`#seat-${seat}`).click();
  if (name === 'Mina Sol') await page.getByRole('dialog').screenshot({ path: '/tmp/bts-casting-picker.png' });
  await page.getByRole('dialog').getByRole('button', { name: `Choose ${name}`, exact: true }).click();
  await expect(page.getByRole('dialog')).not.toBeVisible();
}
async function book(page: Page, id: number) {
  await page.locator(`#review-${id}`).click();
  await expect(page.locator(`#booking-review-${id}`)).toContainText('Ready to book?');
  await expect(page.locator(`#booking-review-${id}`)).toContainText('Aiming for');
  await page.locator(`#confirm-${id}`).click();
}
async function finish(page: Page, id: number) { await page.locator(`#production-${id}`).getByRole('button', { name: /^Finish / }).click(); }

test('cast, review, earn Sway, negotiate and support a show then advertise and broadcast', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  const failures: string[] = []; page.on('pageerror', e => failures.push(e.message));
  await page.goto('/#planning'); await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('#jump-now').click();await page.screenshot({path:'/tmp/bts-live-hud.png'});
  await goToDesk(page, '▣ Production');
  await page.locator('#studio-index').getByRole('button', { name: 'Sway requests', exact: true }).click();
  await page.locator('#accept-request-1').click();
  await page.locator('#studio-index').getByRole('button', { name: 'Discussion brief', exact: true }).click();
  await page.locator('#create-production').click();
  const first = page.locator('#production-1');
  await expect(first).toContainText('620 in fees'); await expect(page.locator('#cash')).toHaveText('20,000');
  await page.locator('#review-1').click(); await expect(page.locator('#booking-review-1')).toContainText('Cash remaining: 19,000');
  await expect(page.locator('#cash')).toHaveText('20,000'); // review reserves nothing
  await page.locator('#confirm-1').click(); await finish(page, 1);
  await expect(page.locator('#clock')).toContainText('Day 1 · 13:00'); await expect(page.locator('#sway-panel')).toContainText('Sway · 4'); await expect(page.locator('#resource-sway')).toHaveText('4');
  await first.getByRole('button', { name: 'Show in programme library' }).click();
  await pickTime(page, 72, 0); await page.locator('#place').click();
  await page.locator('#open-ad-sales').click();
  await page.locator('#advertiser').selectOption('2'); await page.locator('#set-advertiser').click(); await page.locator('#quote').click(); await page.locator('#accept').click(); await page.locator('#advance-next').click();
  await expect(page.locator('#selected-contract')).toContainText('Settled: paid');
  await goToDesk(page, '▣ Production'); await page.locator('#studio-index').getByRole('button', { name: 'Discussion brief', exact: true }).click(); await page.locator('#production-new').click();
  await page.locator('#production-day').selectOption('2'); await page.locator('#production-time').selectOption('40');
  await choose(page, 0, 'Mina Sol'); await choose(page, 1, 'Jules Hart'); await choose(page, 2, 'Rafi Linden');
  await page.locator('#production-name').fill('Neighbourhood Conversations'); await page.getByRole('button', { name: 'Everyday life', exact: true }).click();
  await page.locator('#production-tone').fill('1');
  await expect(page.locator('#cast-concerns')).toContainText('Mina and Jules');
  await page.locator('#create-production').click(); const second = page.locator('#production-2');
  await page.locator('#review-2').click(); await expect(page.getByRole('alert')).toContainText('exceptional_participation_required');
  await second.getByRole('button', { name: /Bring Jules Hart in/ }).click();
  await page.locator('#support-exact-2').fill('2');
  await expect(second.locator('.support-remaining')).toContainText('0 left after booking');
  await page.locator('#review-2').click(); await expect(page.locator('#booking-review-2')).toContainText('Spend 4 Sway');
  await expect(page.locator('#sway-panel')).toContainText('Sway · 4'); await page.locator('#production').screenshot({ path: '/tmp/bts-casting-review.png' }); await page.locator('#confirm-2').click();
  await expect(second).toContainText('Team support: 2 Sway'); await expect(page.locator('#sway-panel')).toContainText('Sway · 0');
  await finish(page, 2); await page.locator('#production').screenshot({ path: '/tmp/bts-casting-desk.png' });
  await second.getByRole('button', { name: 'Show in programme library' }).click(); await pickTime(page, 72, 2); await page.locator('#place').click(); await page.locator('#advance-next').click();
  await expect(page.locator('#clock')).toContainText('Day 3 · 18:45'); await page.locator('#open-review').click(); await expect(page.locator('#selected-result')).toContainText('Completed.');
  await expect(page.locator('#results-list tbody tr')).toHaveCount(2);
  await (await openStationDirectory(page)).getByRole('button', { name: 'Start over' }).click();await expect(page.locator('#cash')).toHaveText('20,000');await expect(page.locator('#production-projects')).not.toContainText('Neighbourhood Conversations');await expect(page.locator('#production-day')).toHaveValue('0');await expect(page.locator('#production-time')).toHaveValue('40');await expect(page.locator('#production-name')).toHaveValue('Workshop Conversations');
  expect(failures).toEqual([]);
});

test('conflicting bookings are rejected and cast cards explain duplicates and commitments', async ({ page }) => {
  await page.goto('/#planning'); await expect(page.locator('#status')).toContainText('WASM READY');
  await goToDesk(page, '▣ Production');
  await page.locator('#create-production').click(); await book(page, 1);
  await page.locator('#studio-index').getByRole('button', { name: 'Discussion brief', exact: true }).click();
  await page.locator('#production-name').fill('Conflicting recording'); await page.locator('#create-production').click();
  await page.locator('#review-2').click(); await expect(page.getByRole('alert')).toContainText('talent_conflict'); await expect(page.locator('#cash')).toHaveText('19,000');
  await page.locator('#production-2').getByRole('button', { name: 'Edit concept' }).click(); await page.locator('#seat-1').click();
  await expect(page.getByRole('dialog').getByRole('button', { name: 'Choose Nora Vale', exact: true })).toBeDisabled();
  await expect(page.getByRole('dialog')).toContainText('Already recording');
  await expect(page.getByRole('dialog').getByRole('button', { name: 'Choose Felix Reed', exact: true })).toBeDisabled();
  await page.keyboard.press('Escape'); await expect(page.locator('#seat-1')).toBeFocused();
  await page.locator('#production-time').selectOption('44'); await page.locator('#create-production').click();
  await expect(page.locator('#production-2')).toContainText('Day 1 · 11:00');
  await page.locator('#production-2').getByRole('button', { name: 'Discard concept' }).click(); await expect(page.locator('#production-2')).toHaveCount(0);
});

test('discussion, pooled Sway and film commands in actual Wasm match the native sequence', async ({ page }) => {
  test.setTimeout(90_000);
  const native = JSON.parse(execFileSync('cargo', ['run', '--locked', '--quiet', '-p', 'bts-web', '--example', 'production_experiment', '--', '--sequence'], { cwd: root, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 }));
  await page.route('**/__test__/bts_web.js', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web.js'), contentType: 'text/javascript' }));
  await page.route('**/__test__/bts_web_bg.wasm', route => route.fulfill({ path: resolve(root, 'web/pkg/bts_web_bg.wasm'), contentType: 'application/wasm' }));
  await page.goto('/#planning'); await expect(page.locator('#status')).toContainText('WASM READY');
  const actual = await page.evaluate(async requests => {
    // @ts-expect-error generated bindings are served by test-only routes
    const { default: init, WebSession } = await import('/__test__/bts_web.js'); await init(); const s = new WebSession(); const responses = []; let token = '';
    for (const request of requests) { if (request.command.type === 'accept_offer') request.command.offer_id = token; const response = JSON.parse(s.dispatch(JSON.stringify(request))); responses.push(response); if (response.status === 'ok' && response.report.offer) token = response.report.offer.offer_id; }
    s.free(); return responses;
  }, native.requests);
  function difference(a: unknown, b: unknown, path = 'response'): string | undefined {
    if (typeof b === 'number') return typeof a === 'number' && Math.abs(a - b) < 5e-9 ? undefined : path;
    if (Array.isArray(b)) {
      if (!Array.isArray(a) || a.length !== b.length) return path;
      for (let i = 0; i < b.length; i++) {
        const mismatch = difference(a[i], b[i], `${path}[${i}]`);
        if (mismatch) return mismatch;
      }
      return undefined;
    }
    if (b !== null && typeof b === 'object') {
      if (a === null || typeof a !== 'object' || Array.isArray(a)) return path;
      if (Object.keys(a).sort().join(',') !== Object.keys(b).sort().join(',')) return path;
      for (const [key, value] of Object.entries(b)) {
        if (key === 'offer_id' || key === 'token') continue;
        const mismatch = difference((a as Record<string, unknown>)[key], value, `${path}.${key}`);
        if (mismatch) return mismatch;
      }
      return undefined;
    }
    return a === b ? undefined : path;
  }
  expect(difference(actual, native.responses), 'first native/Wasm mismatch').toBeUndefined();
  expect(actual.filter((r: { status: string }) => r.status === 'error').map((r: { error: { code: string } }) => r.error.code)).toEqual(['production_not_ready', 'exceptional_participation_required']);
});


test('second quest, exact whole support, keyboard and narrow-screen casting controls', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/#planning'); await expect(page.locator('#status')).toContainText('WASM READY');
  await goToDesk(page, '▣ Production');
  await page.locator('#seat-2').focus(); await page.keyboard.press('Enter');
  await page.getByRole('dialog').getByRole('button', { name: 'Choose Rafi Linden', exact: true }).click();
  await page.getByRole('button', { name: 'Everyday life', exact: true }).click();
  await page.locator('#studio-index').getByRole('button', { name: 'Sway requests', exact: true }).click();
  await page.locator('#accept-request-2').click();
  await page.locator('#studio-index').getByRole('button', { name: 'Discussion brief', exact: true }).click();
  await page.getByRole('radio', { name: /Shoestring/ }).check(); await page.locator('#create-production').click();
  const first = page.locator('#production-1');
  await expect(first.locator('.quest-hint')).toContainText('matches production requirements');
  await book(page, 1); await finish(page, 1); await expect(page.locator('#sway-panel')).toContainText('Sway · 3');
  await page.locator('#studio-index').getByRole('button', { name: 'Discussion brief', exact: true }).click(); await page.locator('#production-new').click(); await page.locator('#production-name').fill('A different production'); await page.locator('#production-time').selectOption('60');
  await choose(page, 1, 'Jules Hart'); await page.locator('#create-production').click();
  await page.locator('#support-exact-2').fill('1.5'); await expect(page.locator('#review-2')).toBeDisabled();
  await page.locator('#support-exact-2').fill('4'); await expect(page.locator('#review-2')).toBeDisabled();
  await page.locator('#support-2').focus(); await page.keyboard.press('Home'); await page.keyboard.press('ArrowRight');
  await expect(page.locator('#support-exact-2')).toHaveValue('1');
  await book(page, 2); await expect(page.locator('#sway-panel')).toContainText('Sway · 2'); await finish(page, 2);
  await expect(page.locator('#production-2')).toContainText('Team support: 1 Sway');
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
});

test('quests are accepted separately, show deadlines and expire without rewards', async ({ page }) => {
  await page.goto('/#planning'); await expect(page.locator('#status')).toContainText('WASM READY');
  await goToDesk(page, '▣ Production');
  await page.locator('#studio-index').getByRole('button', { name: 'Sway requests', exact: true }).click();
  await page.locator('#accept-request-2').click(); await expect(page.locator('#request-2')).toContainText('Open · due Day 3 · 00:00');
  await expect(page.locator('#accept-request-2')).toHaveCount(0); await expect(page.locator('#cash')).toHaveText('20,000');
  await page.locator('#advance-day').click(); await page.locator('#advance-day').click();
  await page.getByText('Completed & expired quests', { exact: true }).click();
  await expect(page.locator('#request-2')).toContainText('Expired at Day 3 · 00:00'); await expect(page.locator('#sway-panel')).toContainText('Sway · 0');
});

test('distinct rival library and staff, with studio capacity independent of cast conflicts', async ({ page }) => {
  await page.goto('/#planning');await expect(page.locator('#status')).toContainText('WASM READY');
  await goToDesk(page, '▣ Production'); await page.locator('#studio-index').getByRole('button', { name: 'Facilities', exact: true }).click(); await expect(page.locator('#station-teams')).toContainText('Elian North, Mina Sol');await expect(page.locator('#station-teams')).toContainText('Vera Quinn');
  await goToDesk(page, '▦ Schedule'); await expect(page.locator('#catalogue')).not.toContainText('After Hours');await page.locator('[data-station="2"] [data-airing]').first().click();
  await expect(page.locator('#selected-name')).toHaveText('After Hours');await expect(page.locator('#known-details')).toContainText('outside your library');
  await goToDesk(page, '▣ Production');
  await page.locator('#studio-index').getByRole('button', { name: 'Discussion brief', exact: true }).click();
  await page.locator('#create-production').click();await book(page,1);
  await page.locator('#studio-index').getByRole('button', { name: 'Discussion brief', exact: true }).click();
  await choose(page,0,'Mina Sol');await choose(page,1,'Jules Hart');await choose(page,2,'Rafi Linden');
  await page.locator('#production-name').fill('A second studio session');await page.locator('#create-production').click();await page.locator('#review-2').click();
  await expect(page.getByRole('alert')).toContainText('studio_unavailable');await expect(page.locator('#cash')).toHaveText('19,000');
  await page.locator('#production-2').getByRole('button',{name:'Edit concept'}).click();await page.locator('#production-time').selectOption('44');await page.locator('#create-production').click();
  await book(page,2);await expect(page.locator('#production-2')).toContainText('Studio 1 reserved');await expect(page.locator('#cash')).toHaveText('18,000');
});
