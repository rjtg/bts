import { expect, test } from '@playwright/test';

test('audience research takes effect in the next paid group poll', async ({ page }) => {
  await page.goto('/');
  const research = page.locator('#audience-research');
  await expect(research).toContainText('Insight available: 12');
  const young = research.locator('article').filter({ has: page.getByRole('heading', { name: 'Young viewers' }) });
  await young.getByLabel('Polling frequency').selectOption('daily');
  await young.getByLabel('Insight to spend').fill('8');
  await young.getByRole('button', { name: 'Improve next estimate' }).click();
  await expect(research).toContainText('Insight available: 4');
  await expect(young).toContainText('Familiarity at poll: Low');
  await expect(page.locator('#cash')).toHaveText('20,000');
  await page.locator('#advance-day').click();
  await expect(young).toContainText('Familiarity at poll: High');
  await expect(young).toContainText('Day 2 · 00:00');
  await expect(page.locator('#cash')).toHaveText('19,940');
  await expect(research.locator('article').filter({ hasText: 'Households' })).toContainText('24h old');
});
