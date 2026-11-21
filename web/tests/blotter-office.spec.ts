import { openStationDirectory } from './station-directory';
import { expect, test, type Page } from '@playwright/test';
import { mkdirSync } from 'node:fs';

async function openOffice(page: Page) {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto('/');
  await expect(page.locator('#status')).toContainText('WASM READY');
}

async function capture(page: Page, name: string) {
  const directory = process.env.BTS_BLOTTER_CAPTURES;
  if (!directory) return;
  mkdirSync(directory, { recursive: true });
  await page.evaluate(async () => {
    await document.fonts.ready;
    const art = new Set<string>();
    for (const item of document.querySelectorAll('#office, #office *')) {
      for (const style of [getComputedStyle(item), getComputedStyle(item, '::before')]) {
        for (const url of style.backgroundImage.matchAll(/url\("([^"]+)"\)/g)) art.add(url[1]);
      }
    }
    await Promise.all([...art].map(url => {
      const image = new Image();
      image.src = url;
      return image.decode();
    }));
  });
  await page.screenshot({ path: `${directory}/${name}.png` });
}

test('prop transition can be cancelled and repeated selection opens one workspace', async ({ page }) => {
  await openOffice(page);
  await capture(page, 'desktop-resting');
  const research = page.locator('[data-prop="research"]');
  const flight = page.locator('.office-prop-in-flight');
  await research.focus();
  await research.press('Enter');
  await expect(flight).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(flight).toHaveCount(0);
  await expect(page.locator('#audience')).toBeHidden();
  const captures = Boolean(process.env.BTS_BLOTTER_CAPTURES);
  // Dispatch both clicks in one task to check the transition's reentry guard.
  await research.evaluate((link, captureFrames) => {
    link.click();
    link.click();
    if (captureFrames) document.querySelector('.office-prop-in-flight')?.getAnimations()[0].pause();
  }, captures);
  await expect(flight).toHaveCount(1);
  if (captures) {
    for (const time of [0, 60, 120, 180, 240]) {
      await flight.evaluate((el, milliseconds) => { el.getAnimations()[0].currentTime = milliseconds; }, time);
      await capture(page, `flight-${time}`);
    }
    await flight.evaluate(el => el.getAnimations()[0].finish());
  }
  await expect(page.locator('#audience')).toBeVisible();
  await expect(page.locator('#office .office-room')).toHaveAttribute('inert', '');
  await page.locator('#audience').getByRole('button', { name: 'Young viewers' }).click();
  await capture(page, 'desktop-research');
  const draft = page.getByRole('slider', { name: 'Insight to spend' }).first();
  await draft.press('Home');
  await draft.press('ArrowRight');
  await expect(draft).toHaveValue('1');
  await page.keyboard.press('Escape');
  await expect(page.locator('#audience')).toBeHidden();
  await expect(research).toBeFocused();
  await research.press('Enter');
  await expect(draft).toHaveValue('1');
  await page.keyboard.press('Escape');
  await expect(page.locator('#audience')).toBeHidden();
  await expect(page.locator('#clock')).toContainText('00:00');
});

test('decorative props stay still and Preparation opens Production', async ({ page }) => {
  await openOffice(page);
  const notebook = page.locator('#open-station-notes');
  await notebook.click();
  const pages = page.getByRole('dialog', { name: 'Station notes' });
  await expect(pages).toBeVisible();
  await pages.getByRole('button', { name: 'Close notes' }).click();
  await expect(notebook).toBeFocused();
  const phone = page.locator('.office-phone');
  const restingPhone = (await phone.boundingBox())!;
  await page.mouse.move(restingPhone.x + restingPhone.width / 2, restingPhone.y + restingPhone.height / 2);
  await expect(phone).toHaveCSS('pointer-events', 'none');
  await expect(phone).toHaveCSS('content', 'normal');
  expect(await phone.boundingBox()).toEqual(restingPhone);
  const calendar = page.locator('#office-calendar');
  const hitbox = (await calendar.boundingBox())!;
  await page.mouse.move(1222, hitbox.y + hitbox.height * .65);
  expect(await calendar.evaluate(item => item.matches(':hover'))).toBe(false);
  const preparation = page.locator('[data-prop="script"]');
  await expect(page.locator('[data-prop="production"]')).toHaveCount(0);
  await expect(page.locator('#clock')).toContainText('00:00');
  await preparation.click();
  await page.getByRole('dialog').getByRole('button', { name: 'Production', exact: true }).click();
  await page.getByRole('dialog').getByRole('button', { name: 'Work orders', exact: true }).click();
  await expect(page.locator('#production')).toBeVisible();
  await page.locator('#production').getByRole('link', { name: 'Back to Office' }).click();
  await expect(preparation).toBeFocused();
});

test.describe('narrow office', () => {
  test.use({ hasTouch: true, reducedMotion: 'reduce' });
  test('touch opens work directly and restores the prop', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto('/');
    const research = page.locator('[data-prop="research"]');
    await expect(research).toBeVisible();
    await capture(page, 'narrow-resting');
    await research.tap();
    await expect(page.locator('#audience')).toBeVisible();
    await expect(page.locator('.office-prop-in-flight')).toHaveCount(0);
    await capture(page, 'narrow-research');
    await page.locator('#audience').getByRole('link', { name: 'Back to Office' }).tap();
    await expect(research).toBeFocused();
  });
});

test('idle actors choose independent layered poses while the station clock holds', async ({ page }, testInfo) => {
  await page.addInitScript(() => {
    let seed = 94389;
    Math.random = () => ((seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0) / 4294967296);
  });
  await page.clock.install({ time: new Date('2026-01-01T00:00:00Z') });
  await page.clock.pauseAt(new Date('2026-01-01T00:00:01Z'));
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  await openOffice(page);
  const actors = page.locator('#office [data-idle-actor]');
  const steam = page.locator('.office-steam i').first();
  await expect(actors).toHaveCount(12);
  const poses = () => actors.evaluateAll(items => items.map(item => {
    const { poseLarge, poseMedium, poseSmall } = (item as HTMLElement).dataset;
    return [poseLarge, poseMedium, poseSmall];
  }));
  const changes = [0, 0, 0];
  const seen = new Set<number>();
  let simultaneous = false;
  let before = await poses();
  for (let sample = 0; sample < 50; sample++) {
    await page.clock.runFor(1100);
    const after = await poses();
    let movers = 0;
    for (const [actor, pose] of after.entries()) {
      if (pose.join() === before[actor].join()) continue;
      movers++;
      seen.add(actor);
      pose.forEach((value, tier) => { if (value !== before[actor][tier]) changes[tier]++; });
    }
    simultaneous ||= movers > 1;
    before = after;
  }
  expect(seen.size).toBe(12);
  expect(simultaneous).toBe(true);
  expect(changes[2]).toBeGreaterThan(changes[1]);
  expect(changes[1]).toBeGreaterThan(changes[0]);
  expect(changes[0]).toBeGreaterThan(0);
  await expect(page.locator('#clock')).toContainText('00:00');
  await page.screenshot({ path: testInfo.outputPath('independent-poses.png'), animations: 'disabled' });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(steam).toHaveCSS('animation-name', 'none');
  const frozen = await poses();
  await page.clock.runFor(4000);
  expect(await poses()).toEqual(frozen);
  const choice = (await openStationDirectory(page)).getByLabel('Room motion');
  await choice.selectOption('on');
  await expect(steam).toHaveCSS('animation-name', 'coffee-steam');
  await page.clock.runFor(4000);
  expect(await poses()).not.toEqual(frozen);
  await choice.selectOption('off');
  await expect(steam).toHaveCSS('animation-name', 'none');
  await expect(page.locator('.monitor-reel')).toHaveCSS('animation-name', 'none');
  const off = await poses();
  await page.clock.runFor(4000);
  expect(await poses()).toEqual(off);
});

test('all three CRTs share a fitted reel pool and retain their programme in reduced motion', async ({ page }, testInfo) => {
  await page.clock.install();
  await page.clock.pauseAt(new Date(Date.now() + 1000));
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  await page.setViewportSize({ width: 1280, height: 900 });
  await openOffice(page);
  const screens = page.locator('.monitor-reel i');
  const reels = () => screens.evaluateAll(items => items.map(item => ({
    id: (item as HTMLElement).dataset.reel!,
    animation: getComputedStyle(item, '::before').animationName,
  })));
  const perSet = [new Set<string>(), new Set<string>(), new Set<string>()];
  for (let batch = 0; batch < 20; batch++) {
    const active = await reels();
    expect(new Set(active.map(screen => screen.id)).size).toBe(3);
    expect(active.filter(screen => screen.animation !== 'none').length).toBeGreaterThanOrEqual(2);
    active.forEach((screen, index) => perSet[index].add(screen.id));
    if (batch === 3) await page.screenshot({ path: testInfo.outputPath('rotating-crts.png'), animations: 'disabled' });
    await page.clock.runFor(9500);
  }
  perSet.forEach(set => expect(set.size).toBe(20));
  const still = (await reels()).map(screen => screen.id);
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.clock.runFor(12000);
  expect(await reels()).toEqual(still.map(id => ({ id, animation: 'none' })));
  await expect(page.locator('#clock')).toContainText('00:00');
});
