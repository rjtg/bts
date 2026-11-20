import { expect, test } from '@playwright/test';

test('book a weekly discussion run and change edition two after reviewing edition one', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto('/#production');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('#production-name').fill('Studio Roundtable');
  await page.getByText('Make this a weekly show').click();
  await page.getByLabel('Airtime on recording day').fill('18:01');
  await page.getByRole('button', { name: 'Open a three-edition binder' }).click();
  const binder = page.locator('.discussion-binder');
  await expect(binder.locator('.production-card')).toHaveCount(3);
  await expect(binder).toContainText('Day 15 at 18:01');
  await expect(page.locator('#cash')).toHaveText('20,000');

  const first = binder.locator('#production-1');
  const second = binder.locator('#production-2');
  const third = binder.locator('#production-3');
  await first.getByRole('button', { name: /Book edition/ }).click();
  await first.getByRole('button', { name: /Finish Studio Roundtable/ }).click();
  await first.getByRole('button', { name: 'Open airing file' }).click();
  await page.locator('#advance-selected').click();
  await page.locator('#open-review').click();
  await expect(page.locator('#selected-result')).toContainText('Completed.');

  await page.getByRole('link', { name: '▣ Production', exact: true }).click();
  const replaceGuest = async (edition: typeof second) => {
    await edition.getByRole('button', { name: 'Edit edition' }).click();
    await page.locator('#seat-1').click();
    await page.getByRole('button', { name: 'Choose Rafi Linden' }).click();
  };
  await replaceGuest(second);
  await page.getByRole('button', { name: 'Entertainment & culture', exact: true }).click();
  await page.locator('#production-tone').fill('2');
  await page.getByRole('button', { name: 'Update concept' }).click();
  await expect(first).toContainText('Science & invention');
  await expect(second).toContainText('Entertainment & culture · lively');
  if (process.env.BTS_DISCUSSION_CAPTURE) await binder.screenshot({ path: '../contrib/season-roadmap/discussion-binder.png' });

  await second.getByRole('button', { name: /Book edition/ }).click();
  await replaceGuest(third);
  await page.getByRole('button', { name: 'Update concept' }).click();
  await third.getByRole('button', { name: /Book edition/ }).click();
  for (const edition of [second, third]) {
    await edition.getByRole('button', { name: /Finish Studio Roundtable/ }).click();
    await edition.getByRole('button', { name: 'Open airing file' }).click();
    await page.locator('#advance-selected').click();
    await page.locator('#open-review').click();
    await expect(page.locator('#selected-result')).toContainText('Completed.');
    await page.getByRole('link', { name: '▣ Production', exact: true }).click();
  }
});
