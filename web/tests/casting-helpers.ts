import type { Locator, Page } from '@playwright/test';

export async function choosePhoto(page: Page, spot: Locator, name: string) {
  await spot.click();
  await page.getByRole('dialog').getByRole('button', { name: `Choose ${name}`, exact: true }).click();
}
