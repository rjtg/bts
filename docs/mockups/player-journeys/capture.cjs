const assert = require('node:assert/strict');
const { execFileSync } = require('node:child_process');
const { mkdirSync, mkdtempSync } = require('node:fs');
const { tmpdir } = require('node:os');
const { join } = require('node:path');
const { pathToFileURL } = require('node:url');
const { chromium, expect } = require('../../../web/node_modules/@playwright/test');

const project = __dirname;
const scratch = mkdtempSync(join(tmpdir(), 'bts-journey-shots-'));
const board = pathToFileURL(join(project, 'index.html')).href;

async function shot(page, folder, name) {
  const dir = join(project, folder);
  mkdirSync(dir, { recursive: true });
  const png = join(scratch, name + '.png');
  await page.screenshot({ path: png, animations: 'disabled', caret: 'hide' });
  execFileSync('magick', [png, '-quality', '85', join(dir, name + '.webp')]);
}

async function reviewBoards(browser) {
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, reducedMotion: 'reduce' });
  const errors = [];
  page.on('pageerror', error => errors.push(error));
  const go = async (key, heading) => {
    await page.goto(board + '#' + key);
    await expect(page.getByRole('heading', { name: heading, exact: true })).toBeVisible();
  };
  const step = async (label, key) => {
    await page.getByRole('link', { name: label, exact: true }).click();
    await expect(page).toHaveURL(new RegExp('#' + key + '$'));
  };
  await go('rack', 'Pull a tape');
  const firstTape = page.getByRole('link', { name: /^Episode 1/ });
  await firstTape.focus();
  await firstTape.press('Enter');
  await expect(page).toHaveURL(/#reception$/);
  await step('Find airtime', 'calendar');
  await step('Inspect a clash', 'blocked');
  await page.keyboard.press('Escape');
  await expect(page).toHaveURL(/#calendar$/);
  await step('Choose a different tape', 'gap');
  await page.getByRole('link', { name: /^Episode 1/ }).click();
  await expect(page).toHaveURL(/#calendar$/);
  await step('Place airing', 'placed');
  await step('Sign this offer', 'signed');
  await step('See transmission receipt', 'airing');
  await step('Open Ledger', 'ledger');
  await step('Return to 18:00 receipt', 'airing');
  await step('Tape bottom line', 'bottom-line');

  await go('market', 'The Corner Shop');
  await step('See assessment choice', 'assessment');
  await step('Commission assessment', 'assessment-result');
  await step('Sign for 2,456 Funds', 'acquired');
  await page.getByRole('link', { name: /^The Corner Shop/ }).click();
  await expect(page).toHaveURL(/#acquired-tape$/);

  await go('research', 'Young viewers');
  await step('Spend 3 Insight', 'investigated');
  await step('Open survey orders', 'surveys');

  await go('production', 'Harbour Lights');
  await step('Open Episode 1', 'script');
  await step('Accept and open Cast', 'booking');
  await step('Inspect a conflict', 'cast-conflict');
  await page.keyboard.press('Escape');
  await expect(page).toHaveURL(/#booking$/);
  await step('Confirm example booking', 'work-order');
  await step('See delivered file', 'delivered');

  for (const key of ['rack', 'reception', 'calendar', 'blocked', 'market', 'acquired',
    'research', 'investigated', 'production', 'booking', 'airing', 'ledger']) {
    await page.goto(board + '#' + key);
    await expect(page.locator('.file')).toBeVisible();
    await shot(page, 'mockups', key);
  }
  const narrow = await browser.newPage({ viewport: { width: 390, height: 844 }, hasTouch: true,
    isMobile: true, reducedMotion: 'reduce' });
  await narrow.goto(board + '#calendar');
  await narrow.getByRole('link', { name: 'Choose a different tape' }).tap();
  await expect(narrow).toHaveURL(/#gap$/);
  await narrow.getByRole('link', { name: /^Episode 1/ }).tap();
  await expect(narrow).toHaveURL(/#calendar$/);
  for (const key of ['calendar', 'airing']) {
    await narrow.goto(board + '#' + key);
    await expect(narrow.locator('.file')).toBeVisible();
    assert(await narrow.evaluate(() => document.documentElement.scrollWidth <= innerWidth));
    await shot(narrow, 'mockups', 'narrow-' + key);
  }
  assert.deepEqual(errors, []);
  console.log('Verified mockup links, keyboard, narrow touch and horizontal fit.');
}

async function captureLedger(browser, url) {
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, reducedMotion: 'reduce' });
  await page.goto(url);
  await expect(page.locator('#status')).toContainText('WASM READY');
  await page.locator('#pull-tapes').click();
  await page.getByRole('button', { name: 'Browse all tapes' }).click();
  await page.locator('.series-box summary').first().click();
  await page.locator('[data-programme="1"]').click();
  const tape = page.getByRole('region', { name: 'Tape dossier' });
  await tape.getByRole('button', { name: 'Schedule this tape' }).click();
  await page.getByRole('button', { name: 'One-shot', exact: true }).click();
  await page.getByRole('button', { name: 'Next gap', exact: true }).click();
  const slip = page.getByRole('dialog', { name: 'Airtime booking' });
  await slip.getByLabel('Ad minutes').fill('16');
  await slip.getByRole('button', { name: 'Confirm placement' }).click();
  const airing = page.getByRole('region', { name: 'Airing dossier' });
  await expect(airing).toContainText('18:00–18:46');
  await airing.getByRole('button', { name: 'Find offers' }).click();
  const offer = airing.locator('.minute-offer').filter({ hasText: 'Everyday Snacks' });
  await offer.locator('summary').click();
  await offer.getByRole('button', { name: 'Accept Everyday Snacks offer' }).click();
  await airing.locator('#advance-selected').click();
  await expect(airing).toContainText('Minute sale settlement');
  await airing.getByRole('button', { name: 'Close file' }).click();
  if (await page.locator('#back-office').isVisible()) await page.locator('#back-office').click();
  const prop = page.locator('[data-prop="ledger"]');
  await prop.click();
  await expect(page.locator('#ledger')).toContainText('511');
  await shot(page, 'baseline', 'desktop-ledger');
  await page.locator('#finances').getByRole('link', { name: 'Back to Office' }).click();
  await expect(prop).toBeFocused();
  console.log('Captured the current ledger after a settled airing and verified its return to office.');
}

(async () => {
  const browser = await chromium.launch();
  try {
    const live = process.argv[2];
    if (live) await captureLedger(browser, live);
    await reviewBoards(browser);
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
