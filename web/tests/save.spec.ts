import { expect, test } from '@playwright/test';
import { pickTime } from './planner-helpers';

test('download a station file, reject a bad file and restore the schedule and clock', async ({ page }, testInfo) => {
  await page.goto('/#planning');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('[data-programme="2"]').click();
  await pickTime(page, 72);
  await page.locator('#place').click();
  await page.locator('#advance-tick').click();
  await expect(page.locator('#clock')).toContainText('00:15');

  const files = page.locator('#station-files');
  await files.locator('summary').click();
  const ready = page.waitForEvent('download');
  await files.getByRole('button', { name: 'Download save' }).click();
  const download = await ready;
  const filename = testInfo.outputPath('station-checkpoint.json');
  await download.saveAs(filename);
  const load = page.locator('#load-station');
  await load.setInputFiles({ name: 'invalid.json', mimeType: 'application/json', buffer: Buffer.from('{}') });
  await expect(page.locator('#error')).toContainText('Load failed');
  await expect(page.locator('#clock')).toContainText('00:15');

  await page.locator('#reset').click();
  await expect(page.locator('#clock')).toContainText('00:00');
  await load.setInputFiles(filename);
  await expect(page.locator('#status')).toContainText('Station save restored');
  await expect(page.locator('#clock')).toContainText('00:15');
  await expect(page.locator('#error')).toBeHidden();
  await page.getByRole('link', { name: '▦ Schedule', exact: true }).click();
  await expect(page.locator('[data-station="1"] [data-start-minute="1080"]')).toContainText('Workshop Science');
  await page.screenshot({ path: testInfo.outputPath('restored-station.png') });
});
