import { expect, test, type Page } from '@playwright/test';
import { mkdirSync } from 'node:fs';

async function capture(page: Page, name: string) {
  const dir = process.env.BTS_OFFICE_INTERACTION_CAPTURES;
  if (!dir) return;
  mkdirSync(dir, { recursive: true });
  await page.screenshot({ path: `${dir}/${name}.png`, animations: 'disabled' });
}

test('work first with keyboard, editable preview, ad sale and review', async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await expect(page.locator('#status')).toContainText('WASM READY');
  const shelf = page.locator('#pull-tapes');
  await shelf.focus();
  await expect(shelf).toBeFocused();
  await page.keyboard.press('Enter');
  const wheel = page.locator('.desk-radial.film-can');
  await expect(wheel.getByRole('button', { name: 'Fiction', exact: true })).toBeFocused();
  await expect(wheel.locator('.can-side-sticker')).toHaveText('Owned works');
  await page.screenshot({ path: testInfo.outputPath('tape-shelf-can.png'), animations: 'disabled' });
  await capture(page, 'desktop-owned-works');
  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('Enter');
  await expect(wheel.getByRole('button', { name: 'News', exact: true })).toBeFocused();
  await wheel.getByRole('button', { name: 'Back', exact: true }).press('Enter');
  await expect(wheel.getByRole('button', { name: 'Nonfiction', exact: true })).toBeFocused();
  await page.keyboard.press('Escape');
  await expect(shelf).toBeFocused();
  await shelf.focus();
  await expect(shelf).toBeFocused();
  await page.keyboard.press('Enter');
  await page.keyboard.press('Enter');
  await wheel.getByRole('button', { name: 'Series', exact: true }).press('Enter');
  await page.locator('.series-box summary').first().click();
  await page.locator('[data-programme="1"]').click();
  const tape = page.getByRole('region', { name: 'Tape dossier' });
  await expect(page.locator('#office-workspace .shelf-bay')).toBeHidden();
  await expect(tape.getByRole('button', { name: 'Brief', exact: true })).toBeFocused();
  await tape.getByRole('button', { name: 'Schedule this tape' }).click();
  await wheel.getByRole('button', { name: 'One-shot' }).click();
  await expect(wheel.locator('.can-side-sticker')).toHaveText('One-shot');
  await page.screenshot({ path: testInfo.outputPath('one-shot-can.png'), animations: 'disabled' });
  await wheel.getByRole('button', { name: 'Next gap' }).click();
  const slip = page.getByRole('dialog', { name: 'Airtime booking' });
  await slip.getByLabel('Start time').focus();
  await slip.getByLabel('Start time').press('Enter');
  const clock = page.getByRole('dialog', { name: 'Airing date and time' });
  const minutes = clock.getByRole('spinbutton', { name: 'Minutes', exact: true });
  await expect(minutes).toHaveAttribute('step', '1');
  await minutes.fill('4');
  await clock.getByRole('button', { name: 'Use slot' }).click();
  await slip.getByLabel('Ad minutes').fill('16');
  await expect(slip).toContainText('46 min');
  await expect(page.locator('.today-lane .today-block')).toHaveCount(0);
  await capture(page, 'desktop-placement');
  await slip.getByRole('button', { name: 'Confirm placement' }).click();
  const airing = page.getByRole('region', { name: 'Airing dossier' });
  await expect(airing).toContainText('18:04–18:50');
  await airing.getByRole('button', { name: 'Find offers' }).click();
  const offer = airing.locator('.minute-offer').first();
  await offer.locator('summary').click();
  await offer.getByRole('button', { name: /^Accept/ }).click();
  await expect(airing.locator('.minute-sold')).toHaveCount(1);
  await airing.locator('#advance-selected').click();
  await expect(airing).toContainText('Minute sale settlement');
});

test('regular recommendation opens an editable weekly review', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.locator('#pull-tapes').click();
  const wheel = page.locator('.desk-radial.film-can');
  await wheel.getByRole('button', { name: 'Browse all tapes' }).click();
  await page.locator('.series-box summary').first().click();
  await page.locator('[data-programme="1"]').click();
  await page.getByRole('button', { name: 'Schedule this tape' }).click();
  await wheel.getByRole('button', { name: 'Regular', exact: true }).click();
  await expect(wheel.locator('.can-side-sticker')).toHaveText('Regular');
  await wheel.getByRole('button', { name: 'Recommended run' }).click();
  const schedule = page.locator('#weekly-planner');
  await expect(schedule.getByLabel('Weekly programme')).toHaveValue('1');
  await schedule.getByLabel('Default ad minutes').fill('7');
  await schedule.getByRole('button', { name: 'Preview weekly slots' }).click();
  await expect(schedule.locator('.weekly-date')).toHaveCount(3);
  await schedule.getByRole('link', { name: 'Back to Office' }).click();
  await page.getByRole('button', { name: 'Schedule this tape' }).click();
  await wheel.getByRole('button', { name: 'Regular', exact: true }).click();
  await wheel.getByRole('button', { name: 'Choose days and time' }).click();
  await expect(schedule.getByRole('button', { name: 'Confirm weekly schedule' })).toHaveCount(0);
  await expect(page.locator('.today-lane .today-block')).toHaveCount(0);
});

test('Reception carries a tape into a labelled, editable calendar comparison', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.locator('#pull-tapes').click();
  await page.getByRole('button', { name: 'Browse all tapes' }).click();
  await page.locator('.series-box summary').first().click();
  await page.locator('[data-programme="1"]').click();
  const record = page.getByRole('region', { name: 'Tape dossier' });
  await record.getByRole('button', { name: 'Reception', exact: true }).click();
  await expect(record).toContainText('Best aired');
  await expect(record).toContainText('Survey evidence');
  await record.getByRole('button', { name: 'Find airtime' }).press('Enter');
  const board = page.locator('#tape-fit');
  await expect(board).toContainText('Neighbourhood Laughs');
  await expect(board).toBeFocused();
  await capture(page, 'desktop-tape-fit');
  await board.getByRole('button', { name: /^18:00 · (Good|OK|Weak)$/ }).click();
  const slip = page.getByRole('dialog', { name: 'Airtime booking' });
  await expect(slip.getByLabel('Start time')).toHaveValue('18:00');
  await expect(page.locator('.today-lane .today-block')).toHaveCount(0);
  await slip.getByRole('button', { name: 'Confirm placement' }).click();
  await page.getByRole('region', { name: 'Airing dossier' }).getByRole('button', { name: 'Close file' }).click();
  await board.getByRole('button', { name: '18:00 · Clash' }).click();
  await expect(slip).toContainText('Conflicts with');
  await slip.getByRole('button', { name: 'Cancel' }).click();
  await expect(page.locator('.today-lane .today-block')).toHaveCount(1);
});

test('clicking outside a film can dismisses it without activating the office', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto('/');
  await expect(page.locator('#status')).toContainText('WASM READY');
  const shelf = page.locator('#pull-tapes');
  await shelf.click();
  const wheel = page.locator('.desk-radial.film-can');
  await wheel.locator('.radial-center').click();
  await expect(wheel).toBeVisible();

  const calendar = await page.locator('#office-calendar').boundingBox();
  await page.mouse.click(calendar!.x + calendar!.width / 2, calendar!.y + calendar!.height / 2);
  await expect(wheel).toHaveCount(0);
  await expect(shelf).toBeFocused();
  await expect(page.locator('#office')).toBeVisible();
  await expect(page.locator('#planning')).toBeHidden();
});

test.describe('time first on touch', () => {
  test.use({ hasTouch: true, viewport: { width: 390, height: 844 }, reducedMotion: 'reduce' });
  test('retains the exact time across catalogue browsing and rejects a conflict', async ({ page }) => {
    await page.goto('/');
    await page.locator('#office-calendar').tap();
    const time = page.locator('#choose-placement-time');
    const choose = page.getByRole('button', { name: 'Choose programme for this time' });
    const slip = page.getByRole('dialog', { name: 'Airtime booking' });
    async function selectTape(start: string, id: number) {
      await time.fill(start);
      await choose.tap();
      const wheel = page.locator('.desk-radial.film-can');
      await expect(wheel.locator('.can-side-sticker')).toHaveText(`Tapes for ${start}`);
      await wheel.getByRole('button', { name: 'Browse all tapes' }).tap();
      await page.locator('.series-box summary').first().tap();
      await page.locator(`[data-programme="${id}"]`).tap();
    }
    await selectTape('18:07', 1);
    await expect(slip.getByLabel('Start time')).toHaveValue('18:07');
    await expect(page.locator('.today-lane .today-block')).toHaveCount(0);
    await capture(page, 'narrow-placement');
    await slip.getByRole('button', { name: 'Confirm placement' }).tap();
    await page.getByRole('region', { name: 'Airing dossier' }).getByRole('button', { name: 'Close file' }).tap();
    await selectTape('18:00', 2);
    await expect(slip).toContainText('Conflicts with');
    await slip.getByRole('button', { name: 'Confirm placement' }).tap();
    await expect(slip.locator('.placement-problem')).toContainText(/overlap/i);
    await expect(page.locator('.today-lane .today-block')).toHaveCount(1);
    await expect(page.locator('.today-lane .today-block')).toHaveAttribute('title', '18:07 · Neighbourhood Laughs');
    await slip.getByRole('button', { name: 'Cancel' }).tap();
    await expect(slip).toHaveCount(0);
    await expect(time).toHaveValue('18:00');
  });
});

test('office props and directory share one workspace per task', async ({ page }, testInfo) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  for (const [prop, job] of [['script', 'production'],
    ['rights', 'rights-market'], ['research', 'audience'], ['ledger', 'finances'], ['reviews', 'results']]) {
    const source = page.locator(`#desk-file-piles [data-prop="${prop}"]`);
    await source.click();
    if (job === 'production') await page.getByRole('button', { name: 'Return to desk papers' }).click();
    const work = page.locator(`#${job}.office-job`);
    await expect(work).toBeVisible();
    await expect(page.locator('#office')).toBeVisible();
    const deskPaper = ['audience', 'production', 'results', 'rights-market', 'finances'].includes(job);
    await expect(page.locator(deskPaper ? '#office .office-room' : '#office')).toHaveAttribute('inert', '');
    if (deskPaper) await expect(page.locator(`#office-paper-mount > #${job}`)).toBeVisible();
    if (prop === 'reviews') {
      await expect(work.getByRole('region', { name: 'Airing dossier' })).toBeVisible();
      await expect(work.locator('#results-list')).toContainText('No completed broadcasts');
    }
    if (prop === 'rights') {
      await work.getByRole('navigation', { name: 'Title index' }).getByRole('button', { name: /The Corner Shop/ }).click();
      await expect(work.getByRole('region', { name: 'Market dossier' })).toBeVisible();
      await page.screenshot({ path: testInfo.outputPath('rights-portfolio.png'), animations: 'disabled' });
      await capture(page, 'desktop-portfolio');
    }
    await work.getByRole('link', { name: 'Back to Office' }).click();
    await expect(source).toBeFocused();
  }
});
