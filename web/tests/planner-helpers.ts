import type { Page } from '@playwright/test';

/** Use the timeline as a player would. Dates and quarter marks are shortcuts;
 * exact adjacent gaps are also reachable through the desk notes. */
export async function pickTime(page: Page, tick: number, day?: number) {
  if (day !== undefined) await page.locator('#day').selectOption(String(day));
  const selectedDay = Number(await page.locator('#day').inputValue());
  const slot = page.locator(`#timeline .timeline-cell[data-tick="${selectedDay * 96 + tick}"]`);
  await slot.waitFor({ state: 'visible' });
  await slot.press('Enter');
}

export async function setAdLoad(page: Page, percent: number) {
  await page.locator('#ad-load').evaluate((control: HTMLInputElement, value) => {
    control.value = String(value);
    control.dispatchEvent(new Event('change', { bubbles: true }));
  }, percent);
}
