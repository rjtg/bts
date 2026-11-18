import { expect, test } from '@playwright/test';

test('pitch, revise, cast, book, advertise and air a film', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  await page.goto('/');
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.getByRole('button', { name: 'Big-city thriller' }).click();
  await page.getByRole('button', { name: 'Save free pitch' }).click();
  const film = page.locator('.film-project');
  await expect(film).toContainText('0 credits committed');
  await film.getByRole('button', { name: 'Commission script' }).click();
  await film.getByRole('button', { name: 'Advance to script delivery' }).click();
  await expect(page.locator('#film-notices')).toContainText('Script ready');
  await expect(film).toContainText('Needs work');
  await film.getByLabel('What should the rewrite improve?').selectOption('5');
  await film.getByRole('button', { name: 'Pay for revision' }).click();
  await film.getByRole('button', { name: 'Advance to script delivery' }).click();
  await film.getByRole('button', { name: 'Accept script' }).click();
  await film.getByRole('button', { name: /Dev Rowan/ }).click();
  await film.getByRole('combobox', { name: 'Studio', exact: true }).selectOption('2');
  await film.getByRole('button', { name: 'Review film booking' }).click();
  await expect(film).toContainText('1,450 credits talent + 930 credits studio = 2,380 credits');
  if (process.env.BTS_FILM_SCREENSHOTS) {
    await film.screenshot({ path: '../contrib/original-film/booking.png' });
  }
  await film.getByRole('button', { name: 'Confirm film booking' }).click();
  await film.getByRole('button', { name: 'Advance to film delivery' }).click();
  await expect(page.locator('#film-notices')).toContainText('Film ready');
  await film.getByRole('button', { name: 'Open in programme library' }).click();
  await page.locator('#place-day').selectOption('1');
  await page.locator('#place-time').selectOption('72');
  await page.locator('#place').click();
  await page.locator('#advertiser').selectOption('1');
  await page.locator('#set-advertiser').click();
  await page.locator('#quote').click();
  await page.locator('#accept').click();
  await page.locator('#advance-selected').click();
  const result = page.locator('#selected-result');
  await expect(result).toContainText('Film investment: 2,920 credits');
  await expect(result).toContainText('Provisional audience reception');
  await expect(result).toContainText(/ad revenue: [1-9]/);
  if (process.env.BTS_FILM_SCREENSHOTS) {
    await result.screenshot({ path: '../contrib/original-film/airing.png' });
  }
});
