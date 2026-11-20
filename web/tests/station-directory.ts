import type { Page } from '@playwright/test';

export async function openStationDirectory(page: Page) {
  const directory = page.getByRole('navigation', { name: 'Station directory' });
  if (!await directory.isVisible()) await page.getByRole('button', { name: 'Directory', exact: true }).click();
  return directory;
}

export async function goToDesk(page: Page, name: string | RegExp) {
  const directory = await openStationDirectory(page);
  await directory.getByRole('link', { name, exact: typeof name === 'string' }).click();
}
