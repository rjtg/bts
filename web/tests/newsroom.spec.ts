import { choosePhoto } from './casting-helpers';
import { expect, test } from '@playwright/test';

test('run the daily newsroom and replace a reporter for unstarted editions', async ({ page }, testInfo) => {
  test.setTimeout(90_000);
  await page.goto('/#production');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('#studio-index').getByRole('button', { name: 'Newsroom' }).click();
  const board = page.locator('#newsroom-board');
  await expect(board).toHaveAttribute('open', '');
  await choosePhoto(page, board.locator('#newsroom-studio'), 'Studio 1');
  await board.getByRole('button', { name: 'Start five daily editions' }).click();
  await expect(board.locator('.news-edition-board article')).toHaveCount(5);
  const first = board.locator('#news-edition-1');
  const second = board.locator('#news-edition-2');
  await first.getByRole('button', { name: 'Advance to tape delivery' }).click();
  await expect(first).toContainText('delivered');
  await expect(first).toContainText('Iris Bell');
  await choosePhoto(page, board.locator('#newsroom-reporter-photo'), 'Tomas Avery');
  await board.getByRole('button', { name: 'Apply staff to unstarted editions' }).click();
  await expect(first).toContainText('Iris Bell');
  await expect(second).toContainText('Tomas Avery');
  await board.screenshot({ path: testInfo.outputPath('newsroom-board.png') });
  await first.getByRole('button', { name: 'Open bulletin airing' }).click();
  await page.locator('#advance-selected').click();
  await page.locator('#open-review').click();
  await expect(page.locator('#selected-result')).toContainText('Evening Bulletin');
  await expect(page.locator('#selected-result')).toContainText('Completed.');
});
