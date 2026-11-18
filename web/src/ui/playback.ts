import { button, element } from './dom';
/** Browser pacing only. Each callback requests one explicit Rust tick; no catch-up or world state here. */
export function initialisePlayback(options:{ state:()=>{now_tick:number;end_tick:number}; tick:()=>void; onPlay:()=>void }) {
  let playing=false; let fast=false; let timer:number|undefined;
  function controls():void {
    button('play-pause').textContent=playing?'Ⅱ Pause':'▶ Play'; button('play-pause').setAttribute('aria-label',playing?'Pause':'Play');button('play-pause').setAttribute('aria-pressed',String(playing));
    button('play-pause').disabled=options.state().now_tick>=options.state().end_tick;button('double-speed').setAttribute('aria-pressed',String(fast));
    element('playback-state').textContent=playing?fast?'PLAYING ×2':'PLAYING':'PAUSED';
  }
  function pause():void { playing=false;if(timer!==undefined)window.clearTimeout(timer);timer=undefined;controls(); }
  function schedule():void {
    if(timer!==undefined)window.clearTimeout(timer); if(!playing)return;
    timer=window.setTimeout(()=>{
      timer=undefined;if(document.hidden||options.state().now_tick>=options.state().end_tick){pause();return;}
      options.tick();if(playing)schedule();
    },fast?1000:2000);
  }
  button('play-pause').addEventListener('click',()=>{
    if(playing){pause();return;}if(document.hidden||options.state().now_tick>=options.state().end_tick)return;
    playing=true;options.onPlay();controls();schedule();
  });
  button('double-speed').addEventListener('click',()=>{fast=!fast;controls();schedule();});
  document.addEventListener('visibilitychange',()=>{if(document.hidden)pause();});
  document.addEventListener('focusin',event=>{const t=event.target as HTMLElement;if(t.matches('input,select')||t.closest('dialog,.casting-seats,.production-card'))pause();});
  document.addEventListener('pointerdown',event=>{if((event.target as HTMLElement).closest('form,dialog,.production-card,#offer'))pause();});
  controls();
  return { pause, isPlaying:()=>playing, reset(){fast=false;pause();}, render(){if(options.state().now_tick>=options.state().end_tick)pause();else controls();} };
}
