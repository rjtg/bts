import { pickTime } from './planner-helpers';
import { expect, test, type Page } from '@playwright/test';
async function ready(page: Page) {
  await page.goto('/#planning'); await expect(page.locator('#status')).toContainText('WASM READY');
}
async function freezeClock(page: Page) {
  await page.clock.install({ time: new Date('2026-11-18T00:00:00Z') });
  await page.clock.pauseAt('2026-11-18T00:00:01Z');
}
test('play, pause, double speed and background pause send bounded explicit ticks', async ({ page }) => {
  await ready(page); await freezeClock(page);
  await expect(page.locator('#playback-state')).toHaveText('STANDBY');
  await page.getByRole('button', { name: 'Play', exact: true }).click();
  await page.clock.runFor(1999); await expect(page.locator('#clock')).toContainText('Day 1 · 00:00');
  await page.clock.runFor(1); await expect(page.locator('#clock')).toContainText('Day 1 · 00:15');
  await expect(page.locator('.current-time-marker')).toHaveAttribute('aria-label', 'Current simulated time: Day 1 · 00:15');
  await page.locator('#double-speed').click(); await expect(page.locator('#playback-state')).toHaveText('PLAYING ×2');
  await page.clock.runFor(1000); await expect(page.locator('#clock')).toContainText('Day 1 · 00:30');
  await page.getByRole('button', { name: 'Pause', exact: true }).click();
  await page.clock.runFor(5000); await expect(page.locator('#clock')).toContainText('Day 1 · 00:30');
  await expect(page.getByRole('button', { name: 'Continue' })).toBeFocused();
  await page.getByRole('button', { name: 'Continue' }).click();
  await expect(page.locator('#play-pause')).toBeFocused();
  await page.locator('#library-search').focus();
  await expect(page.locator('#playback-state')).toHaveText('STANDBY'); await page.clock.runFor(2000); await expect(page.locator('#clock')).toContainText('Day 1 · 00:30');
  await page.locator('#reset').click(); await expect(page.locator('#double-speed')).toHaveAttribute('aria-pressed','false');
  await page.getByRole('button', { name: 'Play', exact: true }).click();
  await page.evaluate(() => { Object.defineProperty(document,'hidden',{ configurable:true,get:()=>true }); document.dispatchEvent(new Event('visibilitychange')); });
  await expect(page.locator('#playback-state')).toHaveText('HELD · PRESS PLAY'); await page.clock.runFor(5000); await expect(page.locator('#clock')).toContainText('Day 1 · 00:00');
});
test('automatic tick processing charges a whole programme once, just like manual ticks', async ({ page }) => {
  await ready(page); await freezeClock(page);
  await pickTime(page, 0); await page.locator('#place').click();
  await page.getByRole('button',{name:'Play',exact:true}).click(); await page.clock.runFor(8000);
  await page.getByRole('button',{name:'Pause',exact:true}).click();
  await expect(page.locator('#clock')).toContainText('Day 1 · 01:00'); await expect(page.locator('#cash')).toHaveText('19,850');
  await page.getByRole('button', { name: 'Continue' }).click();
  await page.locator('#open-review').click();
  await expect(page.locator('#selected-result')).toContainText('Completed. Cost 150');
  const viewers=await page.locator('#selected-result tbody').innerText();
  await page.locator('#reset').click(); await page.getByRole('link', { name: '▦ Schedule', exact: true }).click();
  await pickTime(page, 0); await page.locator('#place').click();
  for(let i=0;i<4;i++)await page.locator('#advance-tick').click();
  await page.locator('#open-review').click();
  await expect(page.locator('#cash')).toHaveText('19,850'); expect(await page.locator('#selected-result tbody').innerText()).toBe(viewers);
});
test('resource icons have keyboard tooltips and future resources are unavailable', async ({ page }) => {
  await page.setViewportSize({ width:390,height:844 }); await ready(page);
  await expect(page.getByRole('group',{name:'Station resources'}).locator('svg')).toHaveCount(6);
  await page.getByLabel('Sway',{exact:true}).focus(); await expect(page.locator('#sway-tip')).toBeVisible(); await expect(page.locator('#resource-sway')).toHaveText('0');
  await page.getByLabel('Glamour unavailable',{exact:true}).focus(); await expect(page.locator('#glamour-tip')).toContainText('not yet available');
  await page.locator('#next-day').click(); await expect(page.locator('.current-time-marker')).toHaveCount(0);
  await page.locator('#jump-now').click(); await expect(page.locator('.current-time-marker')).toHaveCount(1);
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=window.innerWidth)).toBe(true);
});
