import { element } from './dom';
import { roomMotionEnabled } from './office-idle';

const animatedReels = ['drama', 'weather', 'variety'];
const atlasCount = 16;
const testCard = animatedReels.length + atlasCount;
const reelCount = testCard + 1;

export function initialiseOfficeMonitors() {
  const office = element('office');
  const screens = Array.from(office.querySelectorAll<HTMLElement>('.monitor-reel i'));
  const offsets = [0, 8, 15];
  let batch = 0;
  const show = () => screens.forEach((screen, index) => {
    const reel = (offsets[index] + batch) % reelCount;
    const atlas = reel - animatedReels.length;
    screen.dataset.reel = String(reel);
    screen.dataset.format = reel < animatedReels.length ? 'frames' : reel === testCard ? 'card' : 'atlas';
    if (atlas < 0) screen.style.setProperty('--monitor-frames', `url('/office-art/monitor-${animatedReels[reel]}.webp')`);
    else if (reel !== testCard) {
      screen.style.setProperty('--reel-x', `${(atlas % 4) * 100 / 3}%`);
      screen.style.setProperty('--reel-y', `${Math.floor(atlas / 4) * 100 / 3}%`);
    }
  });
  show();
  window.setInterval(() => {
    if (!roomMotionEnabled(office) || !screens[0].getClientRects().length) return;
    // Seven is coprime to the pool size. Every reel reaches every screen.
    batch += 7;
    show();
  }, 9500);
}
