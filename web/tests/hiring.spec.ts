import { expect, test } from '@playwright/test';

test('sign three host dates and credit the fee against edition booking', async ({ page }, testInfo) => {
  await page.goto('/#production');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.getByText('Make this a weekly show').click();
  await page.getByRole('button', { name: 'Open a three-edition binder' }).click();
  const hiring = page.locator('#hiring-desk');
  await page.locator('#studio-index').getByRole('button', { name: 'Hiring', exact: true }).click();
  await hiring.getByRole('button', { name: 'Discuss dates with Elian North' }).click();
  const dates = hiring.getByRole('checkbox');
  await expect(dates).toHaveCount(3);
  for (let i = 0; i < 3; i++) await dates.nth(i).check();
  await expect(hiring).toContainText('Pay 660 Funds now');
  await hiring.getByRole('button', { name: 'Sign host agreement' }).click();
  await expect(hiring).toContainText('660 Funds prepaid');
  await expect(page.locator('#cash')).toHaveText('19,340');
  await hiring.screenshot({ path: testInfo.outputPath('host-agreement.png') });

  await page.locator('#studio-index').getByRole('button', { name: 'Work orders', exact: true }).click();
  const first = page.locator('#production-1');
  await expect(first).toContainText('220 Funds prepaid');
  await first.getByRole('button', { name: 'Book edition · 780 Funds' }).click();
  await expect(page.locator('#cash')).toHaveText('18,560');
  await first.getByRole('button', { name: /Finish Workshop Conversations/ }).click();
  await expect(hiring.locator('#hiring-signed')).toContainText('fulfilled');
});
