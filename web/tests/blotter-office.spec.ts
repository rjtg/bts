import { expect, test, type Page } from '@playwright/test';
import { mkdirSync } from 'node:fs';

async function openOffice(page: Page) {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto('/');
  await expect(page.locator('#status')).toContainText('WASM READY');
}

async function capture(page: Page, name: string) {
  const directory = process.env.BTS_BLOTTER_CAPTURES;
  if (!directory) return;
  mkdirSync(directory, { recursive: true });
  await page.evaluate(async () => {
    await document.fonts.ready;
    const art = new Set<string>();
    for (const item of document.querySelectorAll('#office, #office *')) {
      for (const style of [getComputedStyle(item), getComputedStyle(item, '::before')]) {
        for (const url of style.backgroundImage.matchAll(/url\("([^"]+)"\)/g)) art.add(url[1]);
      }
    }
    await Promise.all([...art].map(url => {
      const image = new Image();
      image.src = url;
      return image.decode();
    }));
  });
  await page.screenshot({ path: `${directory}/${name}.png` });
}

test('prop transition can be cancelled and repeated selection opens one workspace', async ({ page }) => {
  await openOffice(page);
  await capture(page, 'desktop-resting');
  const research = page.locator('[data-prop="research"]');
  const flight = page.locator('.office-prop-in-flight');
  await research.focus();
  await research.press('Enter');
  await expect(flight).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(flight).toHaveCount(0);
  await expect(page.locator('#audience')).toBeHidden();
  const captures = Boolean(process.env.BTS_BLOTTER_CAPTURES);
  // Dispatch both clicks in one task to check the transition's reentry guard.
  await research.evaluate((link, captureFrames) => {
    link.click();
    link.click();
    if (captureFrames) document.querySelector('.office-prop-in-flight')?.getAnimations()[0].pause();
  }, captures);
  await expect(flight).toHaveCount(1);
  if (captures) {
    for (const time of [0, 60, 120, 180, 240]) {
      await flight.evaluate((el, milliseconds) => { el.getAnimations()[0].currentTime = milliseconds; }, time);
      await capture(page, `flight-${time}`);
    }
    await flight.evaluate(el => el.getAnimations()[0].finish());
  }
  await expect(page.locator('#audience')).toBeVisible();
  await expect(page.locator('#office .office-room')).toHaveAttribute('inert', '');
  await page.locator('#audience').getByRole('button', { name: 'Young viewers' }).click();
  await capture(page, 'desktop-research');
  const draft = page.getByRole('slider', { name: 'Insight to spend' }).first();
  await draft.press('Home');
  await draft.press('ArrowRight');
  await expect(draft).toHaveValue('1');
  await page.keyboard.press('Escape');
  await expect(page.locator('#audience')).toBeHidden();
  await expect(research).toBeFocused();
  await research.press('Enter');
  await expect(draft).toHaveValue('1');
  await page.keyboard.press('Escape');
  await expect(page.locator('#audience')).toBeHidden();
  await expect(page.locator('#clock')).toContainText('00:00');
});

test('decorative props stay still and Preparation opens Production', async ({ page }) => {
  await openOffice(page);
  const notebook = page.locator('#open-notebook');
  await notebook.click();
  const pages = page.getByRole('dialog', { name: 'Desk notebook' });
  await expect(pages).toBeVisible();
  await pages.getByRole('button', { name: 'Close notebook' }).click();
  await expect(notebook).toBeFocused();
  const phone = page.locator('.office-phone');
  const restingPhone = (await phone.boundingBox())!;
  await page.mouse.move(restingPhone.x + restingPhone.width / 2, restingPhone.y + restingPhone.height / 2);
  await expect(phone).toHaveCSS('pointer-events', 'none');
  await expect(phone).toHaveCSS('content', 'normal');
  expect(await phone.boundingBox()).toEqual(restingPhone);
  const calendar = page.locator('#office-calendar');
  const hitbox = (await calendar.boundingBox())!;
  await page.mouse.move(1222, hitbox.y + hitbox.height * .65);
  expect(await calendar.evaluate(item => item.matches(':hover'))).toBe(false);
  const preparation = page.locator('[data-prop="script"]');
  await expect(page.locator('[data-prop="production"]')).toHaveCount(0);
  await expect(page.locator('#clock')).toContainText('00:00');
  await preparation.click();
  await page.getByRole('dialog').getByRole('button', { name: 'Production', exact: true }).click();
  await page.getByRole('dialog').getByRole('button', { name: 'Work orders', exact: true }).click();
  await expect(page.locator('#production')).toBeVisible();
  await page.locator('#production').getByRole('link', { name: 'Back to Office' }).click();
  await expect(preparation).toBeFocused();
});

test.describe('narrow office', () => {
  test.use({ hasTouch: true, reducedMotion: 'reduce' });
  test('touch opens work directly and restores the prop', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto('/');
    const research = page.locator('[data-prop="research"]');
    await expect(research).toBeVisible();
    await capture(page, 'narrow-resting');
    await research.tap();
    await expect(page.locator('#audience')).toBeVisible();
    await expect(page.locator('.office-prop-in-flight')).toHaveCount(0);
    await capture(page, 'narrow-research');
    await page.locator('#audience').getByRole('link', { name: 'Back to Office' }).tap();
    await expect(research).toBeFocused();
  });
});

test('idle poses persist until their subject is drawn again and only one pose changes', async ({ page }) => {
  await page.addInitScript(() => {
    const draws = [0, .40, 0, .17, .62, .91, .34, .72, .08, .84, .56, .48, .97];
    let index = 0;
    Math.random = () => {
      const draw = index++;
      return draw % 2 ? .5 : draws[Math.floor(draw / 2) % draws.length];
    };
  });
  await page.clock.install({ time: new Date('2026-01-01T00:00:00Z') });
  await page.clock.pauseAt(new Date('2026-01-01T00:00:01Z'));
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  await openOffice(page);
  const office = page.locator('#office');
  const steam = page.locator('.office-steam i').first();
  const staff = page.locator('.office-idle-staff');
  const camera = page.locator('.studio-idle-camera');
  const poses = () => office.locator('[data-pose="1"]').evaluateAll(items => items.map(item => (item as HTMLElement).dataset.quietPose ?? item.className));
  const settle = () => office.evaluate(el => {
    for (const animation of el.getAnimations({ subtree: true })) {
      if (animation instanceof CSSTransition
        || (animation instanceof CSSAnimation && animation.animationName === 'room-leaves')) animation.finish();
    }
  });

  await page.clock.runFor(700);
  await expect(office).toHaveAttribute('data-idle-subject', 'office-idle-staff');
  await expect(staff).toHaveAttribute('data-pose', '1');
  await staff.evaluate(el => el.getAnimations().forEach(animation => animation.pause()));
  await page.clock.runFor(1800);
  await expect(office).toHaveAttribute('data-idle-subject', 'office-idle-staff');
  await settle();
  await page.clock.runFor(40);
  await expect(office).toHaveAttribute('data-idle-subject', 'office-idle-staff');
  await expect(staff).toHaveCSS('opacity', '1');
  const first = await office.getAttribute('data-idle-subject');
  const seen = new Set([first]);
  const expected = [
    'office-idle-staff', 'office-idle-reader', 'office-idle-standing', 'office-idle-colleague',
    'left-seat', 'leaning', 'through-glass', 'studio-idle-camera', 'studio-idle-second-camera',
    'studio-idle-host', 'studio-idle-guest', 'floor-assistant', 'studio-tally', 'office-lamp-glow', 'office-plant',
  ];
  for (let turn = 1; turn < expected.length; turn++) {
    const before = new Set(await poses());
    await page.clock.runFor(120);
    const subject = await office.getAttribute('data-idle-subject');
    expect(seen.has(subject)).toBe(false);
    seen.add(subject);
    expect((await poses()).filter(pose => !before.has(pose)).length).toBeLessThanOrEqual(1);
    await settle();
    if (process.env.BTS_BLOTTER_CAPTURES) {
      await capture(page, `idle-sample-${String(turn).padStart(2, '0')}`);
    }
  }
  expect(seen).toEqual(new Set(expected));
  await expect(staff).toHaveCSS('opacity', '1');
  await expect(camera).toHaveCSS('opacity', '1');
  await expect(page.locator('#clock')).toContainText('00:00');
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(steam).toHaveCSS('animation-name', 'none');
  const frozen = await office.getAttribute('data-idle-subject');
  await page.clock.runFor(1800);
  await expect(office).toHaveAttribute('data-idle-subject', frozen!);
  const choice = page.getByLabel('Room motion');
  await choice.selectOption('on');
  await expect(steam).toHaveCSS('animation-name', 'coffee-steam');
  await page.clock.runFor(1800);
  await expect(office).not.toHaveAttribute('data-idle-subject', frozen!);
  await choice.selectOption('off');
  await expect(steam).toHaveCSS('animation-name', 'none');
  await expect(page.locator('.monitor-reel')).toHaveCSS('animation-name', 'none');
  const off = await poses();
  await page.clock.runFor(2900);
  expect(await poses()).toEqual(off);
});
