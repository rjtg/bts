const root = document.querySelector('#review');
const query = new URLSearchParams(location.search);
const art = '../../../web/public/';
const paths = {
  Cassette:'M3 5h18v14H3z M6 8h12v6H6z M9 11h.1m5.9 0h.1 M7 19l2-3h6l2 3',
  Calendar:'M4 6h16v15H4z M4 10h16 M8 3v5m8-5v5 M8 14h.1m4 0h.1m4 0h.1M8 17h.1m4 0h.1',
  Script:'M5 3h10l4 4v14H5z M15 3v5h4 M8 12h8m-8 4h8',
  Clapperboard:'M3 11h18v10H3z M3 11l-1-5 17-4 1 5z M6 5l3 4m3-6 3 4 M7 16h10',
  Film:'M3 4h18v16H3z M7 4v16m10 0V4 M3 9h4m-4 6h4m10-6h4m-4 6h4',
  Series:'M4 7h12v14H4z M8 3h12v14h-3 M7 11h6m-6 4h6',
  News:'M3 5h18v14H3z M6 8h5v5H6z M14 8h4m-4 4h4M6 16h12',
  Magazine:'M4 4h15v17H4z M7 4v17 M10 8h6v6h-6zm0 9h6',
  Clock:'M12 3a9 9 0 1 1 0 18 9 9 0 0 1 0-18z M12 7v5l4 2',
  Star:'M12 3l3 6 7 1-5 5 1 6-6-3-6 3 1-6-5-5 7-1z',
  Repeat:'M4 9a7 7 0 0 1 12-4l4 3m0-5v5h-5M20 15a7 7 0 0 1-12 4l-4-3m0 5v-5h5',
  People:'M8 11a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm8 0a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M2 20c0-8 12-8 12 0zm12 0h8c0-5-4-7-8-6',
  Studio:'M3 6h13v12H3z M16 10l5-3v10l-5-3 M6 10h5m-5 3h3',
};
const svg = name => `<svg viewBox="0 0 24 24" aria-hidden="true"><path d="${paths[name]}" fill="none" stroke="currentColor" stroke-width="1.65" stroke-linejoin="round" stroke-linecap="round"/></svg>`;
const option = (label, icon, to, note = '') => ({ label, icon, to, note });
const tapeArt = art + 'console-art/cassette.png';
const title = 'Neighbourhood Laughs';
const date = 'Saturday · Day 1';
const exactGap = '18:07';
const roots = ['desk','tape','planner','prep','production'];
let current = query.get('step') || 'desk';
let fromTime = ['gap-owned','gap-fiction','gap-collection','gap-slip','gap-cancel'].includes(current);
let slipTime = fromTime ? exactGap : '18:00';
let ads = 16;
let episode = 1;
let host = '';
const owned = [option('Fiction','Film','fiction'), option('Nonfiction','News','nonfiction')];
const fiction = [option('Movies','Film','collection-movies'),option('Series','Series','collection')];
const schedule = [option('One-shot','Calendar','one'),option('Regular','Repeat','regular')];
const choices = {
  owned:['Owned works','Cassette','Shelf',owned,'desk'],
  fiction:['Fiction','Film','Owned works',fiction,'owned'],
  nonfiction:['Nonfiction','Magazine','Owned works',[option('News','News','collection-news'),option('Magazines','Magazine','collection-magazine')],'owned'],
  'gap-owned':['Owned works','Cassette','Calendar / Choose programme',[option('Fiction','Film','gap-fiction'),option('Nonfiction','News','gap-nonfiction')],'planner'],
  'gap-fiction':['Fiction','Film','Calendar / Owned works',[option('Movies','Film',null,'No fit'),option('Series','Series','gap-collection')],'gap-owned'],
  'gap-nonfiction':['Nonfiction','Magazine','Calendar / Owned works',[option('News','News','gap-collection-news'),option('Magazines','Magazine','gap-collection-magazine')],'gap-owned'],
  schedule:['Schedule','Calendar','Tape file',schedule,'tape'],
  one:['One-shot','Calendar','Tape file / Schedule',[option('Next gap','Clock','next-slip'),option('Recommended slot','Star','best-slip'),option('Choose on planner','Calendar','free-planner')],'schedule'],
  regular:['Regular','Repeat','Tape file / Schedule',[option('Recommended run','Star','weekly'),option('Choose days and time','Calendar','weekly-free')],'schedule'],
  formats:['New project','Script','Preparation',[option('Film','Film','film'),option('Scripted series','Series','series'),option('Discussion show','Clapperboard','discussion'),option('News bulletin','News','news')],'prep'],
  studio:['Production','Clapperboard','Workshop Conversations',[option('Choose host','People','roster'),option('Studio booking','Studio','studio-slip')],'production'],
};
const exits = {
  owned:'desk', fiction:'desk', nonfiction:'desk', 'gap-owned':'gap-cancel', 'gap-fiction':'gap-cancel', 'gap-nonfiction':'gap-cancel',
  schedule:'tape', one:'tape', regular:'tape', formats:'prep', studio:'production',
};
const focusOrigins = { desk:'shelf', tape:'schedule-trigger', 'gap-cancel':'choose-gap', prep:'new-project', production:'production-trigger' };
const radians = deg => deg * Math.PI / 180;
const point = (radius, deg) => [200 + radius * Math.sin(radians(deg)), 200 - radius * Math.cos(radians(deg))];
function wedge(a,b) {
  const [s,e,i,j] = [point(174,a),point(174,b),point(69,b),point(69,a)];
  return `M${s} A174 174 0 0 1 ${e} L${i} A69 69 0 0 0 ${j}Z`;
}
function control(label,to,style='',id='') {
  return `<button type="button" class="control ${style}"${to ? ` data-go="${to}"` : ''}${id ? ` id="${id}"` : ''}>${label}</button>`;
}
function wheel(key,treatment='sectors',comparison=false) {
  const [name,icon,path,items,back] = choices[key];
  const width = 360 / items.length;
  const plate = items.map((item,i) => `<path class="sector-surface ${i === 0 && comparison ? 'preview' : ''} ${item.to ? '' : 'muted'}" d="${wedge(i*width-width/2+3,i*width+width/2-3)}"/>`).join('');
  const indexes = [...Array(36)].map((_,i) => `<path d="M${point(i%3 === 0 ? 189 : 184,i*10)} L${point(193,i*10)}"/>`).join('');
  const labels = items.map((item,i) => {
    const [x,y] = point(124,i*width);
    return `<button type="button" class="wheel-key" style="--x:${x/4}%;--y:${y/4}%" ${item.to ? `data-go="${item.to}"` : 'disabled'}>${svg(item.icon)}<span>${item.label}</span>${item.note ? `<small>${item.note}</small>` : ''}</button>`;
  }).join('');
  const isGap = key.startsWith('gap-');
  const shelf = ['owned','fiction','nonfiction'].includes(key);
  const at = shelf ? '--at-x:-3%;--at-y:22%' : isGap ? '--at-x:51%;--at-y:32%' : key === 'formats' ? '--at-x:25%;--at-y:33%' : '--at-x:49%;--at-y:34%';
  return `<section class="wheel ${treatment}" style="${at}" aria-label="${name} choices">
    <div class="path-strip">${path} <span aria-hidden="true">›</span> <b>${name}</b></div>
    <div class="wheel-view"><svg class="wheel-plate" viewBox="0 0 400 400" aria-hidden="true"><circle cx="200" cy="200" r="194" fill="#272a22" stroke="#a78a5d" stroke-width="3"/><g fill="none" stroke="#a8916b">${indexes}</g>${plate}</svg>
      <div class="wheel-core">${svg(icon)}<strong>${name}</strong>${isGap ? '<small>Sat · D1 · 18:07</small>' : ''}</div>${labels}</div>
    <footer class="wheel-footer">${back && key !== 'owned' ? control('← Back',back) : ''}
      ${key === 'owned' || key === 'gap-owned' ? control('Browse all tapes',isGap ? 'gap-all' : 'all') : ''}
      ${control('Cancel',exits[key],' cancel')}</footer>
  </section>`;
}
function panel(name,content,kind='') {
  return `<section class="work ${kind}" aria-label="${name}"><header><h2>${name}</h2></header><div class="paper">${content}</div></section>`;
}
const bound = (x,y) => `--x:${x}%;--y:${y}%`;
function onDesk(label,to,id,x,y) {
  return `<button type="button" class="control deck-button" style="${bound(x,y)}" id="${id}" data-go="${to}">${label}</button>`;
}
const playContext = `<span class="context-ticket">${date} · ${exactGap}</span>`;
const tapeName = () => `${title} · Episode ${episode}`;
function tapePanel() {
  return panel('Tape file',`<div class="tabs"><span>Brief</span><span>Reception</span><span>Members</span><span>Bottom line</span></div>
    <div class="split"><div><h3>${title}</h3><p>Episode ${episode} · 30 min · Fiction</p><img class="work-tape" src="${tapeArt}" alt="Cassette for Neighbourhood Laughs"></div>
    <div><h3>Airtime</h3><p>Next gap · Today at 18:00</p><p>Suggested · Monday at 18:00</p><div class="buttons">${control('Schedule this tape','schedule','primary','schedule-trigger')}${control('Put away','desk')}</div></div></div>`,'tape');
}
function calendar(free=false, selected=true) {
  return panel('Programming · Day',`<h3>${date}</h3><div class="ruler"><span>18:00</span>${selected ? '<span class="gap">18:07 · selected</span>' : control('Select 18:07 gap','planner','primary')}<span>18:30</span><span>19:00</span></div>
  <div class="buttons">${free
    ? control('Preview selected time','free-slip','primary') + control('Return to tape','tape')
    : (selected ? control('Choose programme','gap-owned','primary','choose-gap') : '') + control('Back to office','desk')}</div>`,'schedule');
}
function collection(key) {
  const gap = key.startsWith('gap-');
  const set = key.replace('gap-','');
  const name = set === 'collection-movies' ? 'Movies' : set === 'collection-news' ? 'News' : set === 'collection-magazine' ? 'Magazines' : set === 'all' ? 'All tapes' : 'Series';
  const isSeries = ['Series','All tapes'].includes(name);
  const all = key.endsWith('-all') || key === 'all';
  const back = all ? gap ? 'gap-owned' : 'owned' : name === 'News' || name === 'Magazines' ? gap ? 'gap-nonfiction' : 'nonfiction' : gap ? 'gap-fiction' : 'fiction';
  const row = (name,detail,to,extra='') => `<button type="button" data-go="${to}" ${extra}><img src="${tapeArt}" alt=""><strong>${name}</strong><span>${detail}</span></button>`;
  const exit = gap ? 'gap-cancel' : 'desk';
  const cancel = control('Cancel',exit,' cancel');
  if (key.endsWith('episodes')) return panel(title,`${gap ? playContext : ''}<div class="collection">${[1,2,3].map(i => row(`Episode ${i}`, i === 2 ? '45 min' : '30 min',gap ? 'gap-slip' : 'tape',`data-episode="${i}"`)).join('')}</div><div class="buttons">${control('← Back',gap ? 'gap-collection' : 'collection')}${cancel}</div>`);
  return panel('Tape collection / ' + name,`${gap ? playContext : ''}
    <div class="collection">${isSeries ? row(title,'3 episodes',gap ? 'gap-episodes' : 'episodes') + row('The Big Premiere','Series','other-title') : ''}
    ${name === 'Movies' || all ? row('The Corner Shop','90 min','other-title') : ''}
    ${name === 'Magazines' || all ? row('Workshop Science','45 min','other-title') : ''}
    ${name === 'News' ? '<p>No finished news tapes on this shelf.</p>' : ''}</div><div class="buttons">${control('← Back',back)}${cancel}</div>`);
}
function formatPanel(key) {
  const formats = {film:'Film pitch',series:'Scripted series brief',discussion:'Discussion-show concept',news:'Daily newsroom'};
  return panel(formats[key],`<h3>${key === 'discussion' ? 'Workshop Conversations' : key === 'film' ? 'A Light in the Window' : 'New project brief'}</h3>
    <label>Working title <input value="${key === 'discussion' ? 'Workshop Conversations' : key === 'film' ? 'A Light in the Window' : ''}"></label><div class="buttons">${control('← Preparation','prep')}${key === 'discussion' ? control('Open Production board','production','primary') : ''}</div>`,'pink');
}
function booking(key) {
  const gap = key === 'gap-slip';
  const regular = key.startsWith('weekly');
  const day = key === 'best-slip' ? 'Monday · Day 3' : date;
  if (regular) {
    const suggested = key === 'weekly';
    return panel('Regular booking preview',`<h3>${tapeName()}</h3>
      <label>Weekday <select id="weekday"><option ${suggested ? 'selected' : ''}>Monday</option><option ${suggested ? '' : 'selected'}>Saturday</option><option>Sunday</option></select></label>
      <label>Start time <input id="slot" type="time" value="18:00"></label><label>Ad minutes <input class="ads" type="number" min="0" max="30" value="${ads}"></label>
      <div id="occurrences"></div><div class="buttons">${control('Refresh dates','refresh','primary')}${control('Cancel','tape',' cancel')}</div>`,'slip');
  }
  return panel('Airtime booking · Preview',`<h3>${tapeName()}</h3>${gap ? playContext : ''}
    ${gap ? '' : `<p>${day}</p>`}<label>Start time <input id="slot" type="time" value="${slipTime}"></label>
    <label>Ad minutes <input class="ads" type="number" min="0" max="30" value="${ads}"></label>
    <p id="occupied"></p><div class="buttons">${control('Confirm placement','confirmed','primary')}${control('Cancel',gap || key === 'free-slip' ? 'gap-cancel' : 'tape',' cancel')}</div>`,'slip');
}
function stage(name, markup) {
  return `<div class="stage"><span class="scene-label">${name}</span>${markup}</div>`;
}
function showStory() {
  const wheelKey = choices[current];
  let markup;
  let label = 'Station Office';
  if (wheelKey) {
    const bg = current.startsWith('gap-') ? calendar() : ['schedule','one','regular'].includes(current) ? tapePanel()
      : current === 'formats' ? panel('Preparation',control('New project','formats','primary','new-project'),'pink')
      : current === 'studio' ? panel('Workshop Conversations','<h3>Production board</h3>','blue') : '';
    // The background remains part of the scene while one work surface has focus.
    markup = (bg ? `<div inert aria-hidden="true">${bg}</div>` : '') + wheel(current);
    label = 'Contextual choices';
  } else if (current.includes('collection') || ['episodes','gap-episodes','all','gap-all'].includes(current)) {
    markup = collection(current);
    label = 'Tape collection';
  } else if (current === 'tape') {
    markup = tapePanel(); label = 'Selected programme';
  } else if (['planner','planner-pick','gap-cancel','free-planner'].includes(current)) {
    markup = calendar(current === 'free-planner', current !== 'planner-pick'); label = 'Programming';
  } else if (current.includes('slip') && current !== 'studio-slip' || current.startsWith('weekly')) {
    const wrap = current === 'gap-slip' || current === 'free-slip' ? calendar() : tapePanel();
    markup = `<div inert aria-hidden="true">${wrap}</div>${booking(current)}`; label = 'Booking preview';
  } else if (current === 'prep') {
    markup = panel('Preparation','<h3>Writing desk</h3>' + control('New project','formats','primary','new-project'),'pink');
  } else if (['film','series','discussion','news'].includes(current)) {
    markup = formatPanel(current);
  } else if (current === 'production') {
    markup = panel('Production board',`<h3>Workshop Conversations</h3><p>Host: ${host || 'Choose a host'}</p><div class="buttons">${control('Cast or book','studio','primary','production-trigger')}</div>`,'blue');
  } else if (current === 'roster') {
    markup = panel('Eligible hosts',`<h3>Workshop Conversations · Host seat</h3><div class="roster">${[['Elian North','elian-north','Science presenter · 220 Funds'],['Mina Sol','mina-sol','Entertainment host · 120 Funds']].map(([name,asset,detail]) => `<button type="button" data-go="production" data-host="${name}"><img src="${art}portraits/${asset}.png" alt=""><strong>${name}</strong><br>${detail}</button>`).join('')}</div><div class="buttons">${control('Cancel','production',' cancel')}</div>`,'blue');
  } else if (current === 'studio-slip') {
    markup = panel('Studio booking',`<h3>Workshop Conversations</h3><p>Host: ${host || 'Choose a host'}</p><label>Recording day <select><option>Day 1</option><option>Day 2</option></select></label><label>Room <select><option>Station studio</option></select></label><div class="buttons">${control('Return to project','production')}</div>`,'blue');
  } else if (current === 'confirmed') {
    markup = panel('Airing file',`<h3>${tapeName()}</h3><p>${slipTime} · ${ads} ad minutes</p><p>Placement confirmed in this storyboard.</p><div class="buttons">${control('Back to calendar','planner')}</div>`);
  } else if (current === 'other-title') {
    markup = panel('Tape file','<p>Use Neighbourhood Laughs for the worked example.</p><div class="buttons">' + control('Back to collection',fromTime ? 'gap-collection' : 'collection') + '</div>');
  } else {
    markup = onDesk('Open shelf','owned','shelf',3,35) + onDesk('Open calendar','planner-pick','calendar-trigger',75,69)
      + onDesk('Preparation','prep','prep-trigger',21,70) + onDesk('Production','production','studio-trigger',60,65);
  }
  return stage(label,markup);
}
function render(moveFocus=false, cancel=false) {
  if (query.get('view') === 'compare') {
    root.innerHTML = `<p class="view-intro">Same office, same shelf anchor, same text and wheel diameter. Each proposed control is composed for a 1280 × 900 scene.</p>
      <div class="compare" inert><figure>${stage('A · Radial sectors',wheel('owned','sectors',true))}<figcaption><strong>A · Radial sectors</strong> / Selected</figcaption></figure>
      <figure>${stage('B · Station rotary',wheel('owned','rotary',true))}<figcaption><strong>B · Station rotary</strong> / Alternate</figcaption></figure></div>
      <p class="story-note">Both keep the return keys outside the ring. The sectors connect each label to its direction and leave the center free for the cassette. The rotary knob makes the control feel more physical and adds a second reading task: following the pointer from the knob to the label.</p>`;
    return;
  }
  if (query.get('view') === 'icons') {
    root.innerHTML = `<p class="view-intro">Use a large icon in the center and a smaller one paired with each action label. Tapes and people use their name and available art in lists.</p>
      <div class="sheet">${Object.keys(paths).map((key,index) => `<article class="icon-card">${svg(key)}<span>${key}</span><small>${index < 4 ? 'Context' : 'Category or action'}</small></article>`).join('')}</div>
      <h2 style="margin-top:22px">Control states</h2><div class="states">${[['normal','Series','Solid edge'],['focused','Series','Light outline'],['pressed','Series','Recessed face'],['unavailable','Movies','Dashed edge · No fit']].map(([state,label,detail]) => `<div class="sample ${state}">${svg(label === 'Movies' ? 'Film' : 'Series')}<strong>${label}</strong><small>${state} · ${detail}</small></div>`).join('')}</div>`;
    return;
  }
  root.innerHTML = `<div class="story"><nav class="story-nav" aria-label="Jump to a journey">${roots.map((key,i) => `<button type="button" data-go="${key}" aria-pressed="${key === current}">${['Office','Tape scheduling','Time first','Preparation','Production'][i]}</button>`).join('')}</nav>
    ${showStory()}<div class="story-caption"><strong>Proposed interaction mockup</strong><span>Follow the controls in the scene. Arrow keys change wheel choice. Escape cancels.</span></div></div>`;
  const target = cancel && focusOrigins[current] ? document.getElementById(focusOrigins[current])
    : root.querySelector('.wheel-key:not(:disabled), .work .control.primary, .work .collection button, .work .roster button, .deck-button');
  if (moveFocus) target?.focus({preventScroll:true});
  root.querySelector('#slot')?.addEventListener('input',updatePreview);
  root.querySelector('.ads')?.addEventListener('input',updatePreview);
  updatePreview();
  updateDates();
}
function updatePreview() {
  const slot = root.querySelector('#slot');
  const quantity = root.querySelector('.ads');
  const line = root.querySelector('#occupied');
  if (!line || !slot?.value || !quantity?.validity.valid) return;
  const [h,m] = slot.value.split(':').map(Number);
  const runtime = episode === 2 ? 45 : 30;
  const end = h*60 + m + runtime + quantity.valueAsNumber;
  line.textContent = `${runtime} min programme + ${quantity.value} min ads · ${slot.value}–${String(Math.floor(end/60)).padStart(2,'0')}:${String(end%60).padStart(2,'0')}`;
}
function updateDates() {
  const list = root.querySelector('#occurrences');
  if (!list) return;
  const weekday = root.querySelector('#weekday').value;
  const time = root.querySelector('#slot').value || '18:00';
  const first = {Saturday:1,Sunday:2,Monday:3}[weekday];
  list.innerHTML = [0,7,14].map(offset => `<p>${weekday} · Day ${first+offset} · ${time}</p>`).join('');
}
root.addEventListener('click',event => {
  const button = event.target.closest('button[data-go]');
  if (!button) return;
  const next = button.dataset.go;
  if (next === 'refresh') { updateDates(); return; }
  if (button.dataset.episode) episode = Number(button.dataset.episode);
  if (button.dataset.host) host = button.dataset.host;
  if (next === 'gap-owned') fromTime = true;
  if (next === 'owned') fromTime = false;
  const timeInput = root.querySelector('#slot');
  if (next === 'confirmed' && timeInput?.value) slipTime = timeInput.value;
  if (root.querySelector('.ads')?.validity.valid) ads = root.querySelector('.ads').valueAsNumber;
  if (next.endsWith('-slip')) slipTime = ['gap-slip','free-slip'].includes(next) ? exactGap : '18:00';
  current = next;
  render(true,button.classList.contains('cancel'));
});
root.addEventListener('keydown',event => {
  const menu = event.target.closest('.wheel');
  if (!menu) return;
  if (event.key === 'Escape') {
    event.preventDefault(); current = exits[current]; render(true,true); return;
  }
  if (!['ArrowLeft','ArrowUp','ArrowRight','ArrowDown'].includes(event.key)) return;
  event.preventDefault();
  const items = [...menu.querySelectorAll('.wheel-key:not(:disabled)')];
  const index = items.indexOf(document.activeElement);
  const delta = ['ArrowRight','ArrowDown'].includes(event.key) ? 1 : -1;
  items[(index < 0 ? 0 : (index + delta + items.length) % items.length)]?.focus();
});
const page = query.get('view') || 'story';
document.querySelector(`nav a[href="?view=${['story','compare','icons'].includes(page) ? page : 'story'}"]`)?.setAttribute('aria-current','page');
if (query.get('focus') === 'return') current = 'gap-cancel';
render(query.get('focus') === 'return',query.get('focus') === 'return');
