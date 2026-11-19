import { button, element } from './dom';

/** Browser pacing. Each callback requests one explicit Rust tick. */
export function initialisePlayback(options: {
  state: () => { now_tick: number; end_tick: number }; tick: () => void; onPlay: () => void;
}) {
  const pauseScreen = element<HTMLDialogElement>('pause-screen');
  const resume = button('continue-play');
  const transport = button('play-pause');
  const speed = button('double-speed');
  const workspace = document.querySelectorAll<HTMLElement>('body > .masthead, #status, #error, #controls');
  let playing = false;
  let fast = false;
  let timer: number | undefined;
  let hiddenHold = false;
  let restoreFocus: HTMLElement | null = null;
  const finished = () => options.state().now_tick >= options.state().end_tick;

  function controls() {
    transport.textContent = playing ? 'Ⅱ Pause' : '▶ Play';
    transport.setAttribute('aria-label', playing ? 'Pause' : 'Play');
    transport.setAttribute('aria-pressed', String(playing));
    transport.disabled = finished();
    speed.disabled = finished();
    speed.setAttribute('aria-pressed', String(fast));
    element('playback-state').textContent = finished() ? 'COMPLETE'
      : pauseScreen.open ? 'PAUSED' : playing ? (fast ? 'PLAYING ×2' : 'PLAYING')
      : hiddenHold ? 'HELD · PRESS PLAY' : 'STANDBY';
  }

  function pause() {
    playing = false;
    if (timer !== undefined) window.clearTimeout(timer);
    timer = undefined;
    controls();
  }

  function schedule() {
    if (timer !== undefined) window.clearTimeout(timer);
    if (!playing) return;
    timer = window.setTimeout(() => {
      timer = undefined;
      if (document.hidden || finished()) { pause(); return; }
      options.tick();
      if (playing) schedule();
    }, fast ? 1000 : 2000);
  }

  function play() {
    if (document.hidden || finished()) return;
    hiddenHold = false;
    playing = true;
    options.onPlay();
    controls();
    schedule();
  }

  transport.addEventListener('click', () => {
    if (finished()) return;
    if (!playing) { play(); return; }
    restoreFocus = document.activeElement instanceof HTMLElement ? document.activeElement : transport;
    pause();
    pauseScreen.showModal();
    for (const area of workspace) area.inert = true;
    controls();
    resume.focus();
  });
  pauseScreen.addEventListener('cancel', event => event.preventDefault());
  resume.addEventListener('click', () => {
    if (document.hidden) return;
    for (const area of workspace) area.inert = false;
    pauseScreen.close();
    restoreFocus?.focus({ preventScroll: true });
    restoreFocus = null;
    play();
  });
  speed.addEventListener('click', () => { fast = !fast; controls(); schedule(); });
  document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
      hiddenHold = playing || hiddenHold;
      pause();
    }
  });
  document.addEventListener('focusin', event => {
    const target = event.target as HTMLElement;
    const drafting = target.matches('input, select') || target.closest('dialog:not(#pause-screen), .casting-seats, .production-card');
    if (drafting && playing) pause();
  });
  document.addEventListener('pointerdown', event => {
    if (playing && (event.target as HTMLElement).closest('form, dialog:not(#pause-screen), .production-card, #offer')) pause();
  });
  controls();
  return {
    pause, isPlaying: () => playing,
    reset() { hiddenHold = false; fast = false; pause(); },
    render() { if (finished()) pause(); else controls(); },
  };
}
