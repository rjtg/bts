const assert = require('node:assert/strict');
const { pathToFileURL } = require('node:url');
const { resolve } = require('node:path');
const { chromium } = require('../../../web/node_modules/playwright');

const review = pathToFileURL(resolve(__dirname, 'review.html')).href;

async function run() {
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({ viewport: { width: 1440, height: 1020 }, reducedMotion: 'reduce' });
    const errors = [];
    page.on('pageerror', e => errors.push(e.message));
    const work = (name, active = page) => active.locator('.stage').getByRole('region', { name, exact: true });
    const click = (name, active = page) => active.locator('.stage').getByRole('button', { name, exact: true }).click();
    const screenshot = async (page, name, selector) => {
      await page.locator(selector).screenshot({ path: resolve(__dirname, name + '.png'), animations: 'disabled' });
    };
    await page.goto(review + '?view=compare');
    const comparable = await page.locator('.compare .stage').evaluateAll(nodes => nodes.map(el => [el.clientWidth, el.clientHeight]));
    assert.equal(comparable.length, 2);
    assert.deepEqual(comparable[0], comparable[1]);
    assert.equal(await page.locator('.stage').evaluateAll(nodes => nodes.every(el => getComputedStyle(el).backgroundImage.includes('desktop-resting.png'))), true);
    await screenshot(page, 'treatment-comparison', '#review');

    await page.goto(review + '?view=icons');
    assert.equal(await page.locator('.sample').count(), 4);
    await screenshot(page, 'icon-and-state-sheet', '#review');

    await page.goto(review);
    await click('Open shelf');
    assert.equal(await page.locator(':focus').innerText(), 'Fiction');
    await page.keyboard.press('ArrowRight');
    assert.equal(await page.locator(':focus').innerText(), 'Nonfiction');
    await page.keyboard.press('Enter');
    await click('← Back');
    await page.keyboard.press('Escape');
    assert.equal(await page.locator(':focus').getAttribute('id'), 'shelf');
    await click('Open shelf');
    await click('Fiction');
    await click('Series');
    await click('Neighbourhood Laughs 3 episodes');
    await click('Episode 1 30 min');
    await work('Tape file').waitFor();
    await click('Schedule this tape');
    await screenshot(page, 'tape-scheduling', '.stage');
    await click('One-shot');
    await click('Next gap');
    await work('Airtime booking · Preview').getByLabel('Start time').fill('18:04');
    assert.match(await page.locator('#occupied').innerText(), /18:04–18:50/);
    await click('Cancel');
    assert.equal(await page.locator(':focus').getAttribute('id'), 'schedule-trigger');
    await click('Schedule this tape');
    await click('One-shot');
    await click('Recommended slot');
    assert.match(await work('Airtime booking · Preview').innerText(), /Monday · Day 3/);
    await click('Cancel');
    await click('Schedule this tape');
    await click('One-shot');
    await click('Choose on planner');
    await click('Preview selected time');
    assert.equal(await work('Airtime booking · Preview').getByLabel('Start time').inputValue(), '18:07');
    await page.goto(review + '?step=tape');
    await click('Schedule this tape');
    await click('Regular');
    await click('Recommended run');
    assert.match(await page.locator('#occurrences').innerText(), /Monday · Day 17/);
    await work('Regular booking preview').getByLabel('Weekday').selectOption('Saturday');
    await click('Refresh dates');
    assert.match(await page.locator('#occurrences').innerText(), /Saturday · Day 15/);
    await click('Cancel');
    await click('Schedule this tape');
    await click('Regular');
    await click('Choose days and time');
    assert.equal(await work('Regular booking preview').getByLabel('Weekday').inputValue(), 'Saturday');

    await page.goto(review + '?step=prep');
    await click('New project');
    assert.equal(await work('New project choices').getByRole('button').count(), 6);
    await click('Discussion show');
    await click('Open Production board');
    await click('Cast or book');
    await click('Choose host');
    await screenshot(page, 'production-roster', '.stage');
    await click('Elian North Science presenter · 220 Funds');
    assert.match(await work('Production board').innerText(), /Host: Elian North/);
    await click('Cast or book');
    await click('Studio booking');
    assert.match(await work('Studio booking').innerText(), /Host: Elian North/);

    const touch = await browser.newPage({ viewport: { width:390, height:844 }, hasTouch:true });
    touch.on('pageerror', e => errors.push(e.message));
    await touch.goto(review);
    const tap = name => touch.locator('.stage').getByRole('button', { name, exact:true }).tap();
    await tap('Open calendar');
    await tap('Select 18:07 gap');
    await tap('Choose programme');
    await tap('Fiction');
    assert.equal(await touch.getByRole('button', { name: 'Movies No fit', exact:true }).isDisabled(), true);
    await screenshot(touch, 'narrow-choices', '.stage');
    await tap('Cancel');
    assert.equal(await touch.locator(':focus').getAttribute('id'), 'choose-gap');
    assert.match(await work('Programming · Day', touch).innerText(), /18:07 · selected/);
    await tap('Choose programme');
    await tap('Browse all tapes');
    await tap('Neighbourhood Laughs 3 episodes');
    await tap('Episode 1 30 min');
    assert.equal(await work('Airtime booking · Preview', touch).getByLabel('Start time').inputValue(), '18:07');
    await screenshot(touch, 'exact-time-handoff', '.stage');
    assert.equal(await touch.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true);
    await tap('Confirm placement');
    assert.match(await work('Airing file', touch).innerText(), /18:07/);
    assert.deepEqual(errors, []);
    console.log('Compared both treatments and replayed the shelf, booking, casting and exact-time touch flows.');
  } finally {
    await browser.close();
  }
}
run().catch(error => { console.error(error); process.exitCode = 1; });
