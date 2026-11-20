// Review-only links between authored scene states. This file has no game commands.
const action = (label, target, secondary = false) =>
  `<a class="btn${secondary ? ' alt' : ''}" href="#${target}">${label}</a>`;

const paper = (title, content) => `<section class="leaf"><h2>${title}</h2>${content}</section>`;
const note = text => `<p class="note">${text}</p>`;
const receipt = (stamp, detail) =>
  `<div class="receipt"><strong class="stamp">${stamp}</strong><p>${detail}</p></div>`;
const figures = rows => `<dl class="figures">${rows.map(([key, value]) =>
  `<dt>${key}</dt><dd>${value}</dd>`).join('')}</dl>`;

const openShow = `<span class="case-title">Series case · Neighbourhood Laughs</span>`;
const tape = (title, sub, next) =>
  `<a class="tape" href="#${next}"><span class="reels" aria-hidden="true">◉ ◉</span><span><b>${title}</b><br><small>${sub}</small></span></a>`;
const tab = (label, current = false) => `<span${current ? ' aria-current="page"' : ''}>${label}</span>`;
const tabs = (...items) => `<div class="tabs">${items.join('')}</div>`;

const airingFigures = [
  ['Ad receipts', '511 Funds'],
  ['Airing cost', '150 Funds'],
  ['Airing margin', '361 Funds'],
];
const slot = (time, state, content, next, selected = false) => {
  const status = {good: 'Good fit', ok: 'OK fit', weak: 'Weak fit', clash: 'Clash'}[state];
  return next
    ? `<a class="slot ${state}" href="#${next}"${selected ? ' aria-current="true"' : ''}><strong>${time}</strong>${status}</a>`
    : `<div class="slot ${state}"><strong>${time} · ${status}</strong>${content}</div>`;
};

const scenarios = {
  rack: {
    title: 'Pull a tape', label: 'Tape rack · owned programmes',
    left: paper('Fiction · series', `
      ${openShow}
      <div class="rack">
        ${tape('Episode 1', '30 min · choose this tape', 'reception')}
        <div class="tape"><span class="reels" aria-hidden="true">◉ ◉</span><span><b>Episode 2</b><br><small>Another tape in the case</small></span></div>
      </div>`),
    right: paper('Airtime starts here', `
      <p>Pull an episode to see the station’s evidence and pick a time.</p>
      ${note('Or start with a gap. The rack keeps that date and time on the table.')}
      ${action('Start with a calendar gap', 'gap', true)}`),
  },
  gap: {
    title: 'Choose a tape for this gap', label: 'Tape rack · from Programming', back: 'calendar',
    left: paper('Saturday · Day 1 · 18:00', `
      ${note('This is the time in hand. Selecting a tape opens a preview.')}
      ${openShow}<div class="rack">${tape('Episode 1', '30 min · choose this tape', 'calendar')}</div>`),
    right: paper('What carries over', '<p>The selected date, channel and time stay attached while you browse. The ad block is adjustable on the slip.</p>'),
  },
  reception: {
    title: 'Neighbourhood Laughs · Episode 1', label:'Tape file',
    back:'rack', tabs:tabs(tab('Brief'),tab('Reception',true),tab('Members'),tab('Bottom line')),
    left: paper('Reception', `
      ${note('Best aired · Saturday evening<br>Also works · weekday evening')}
      <p>Provisional station advice from the Saturday · Day 1 survey. Competition and repeat viewing can change turnout.</p>`),
    right: paper('Find a showing', `
      ${figures([['Programme', '30 min'], ['Completed airings', 'No showing in this run yet']])}
      ${action('Find airtime', 'calendar')}`),
  },
  calendar: {
    title: 'Find airtime', label:'Programming calendar · Saturday, Day 1',
    stock:'dark', back:'reception',
    wide: `<section class="calendar" aria-label="Example calendar openings"><b>Neighbourhood Laughs · Episode 1 · Your channel</b>
      <div class="slot-grid">
        ${slot('17:30', 'weak', '', 'earlier')}
        ${slot('18:00', 'good', '', 'calendar', true)}
        ${slot('18:30', 'clash', 'Example occupied interval', null)}
        ${slot('19:30', 'ok', '', 'later')}
      </div></section>`,
    left: paper('Placement slip', `
      ${figures([['Start', '18:00'], ['Programme', '30 min'], ['Ad block', '16 min'], ['Occupied', '18:00–18:46']])}
      ${action('Place airing', 'placed')} ${action('Inspect a clash', 'blocked', true)}`),
    right: paper('Reading the board', `
      <p>These fit bands illustrate the proposed comparison. A feasible opening still needs an exact interval check after editing its ad minutes.</p>
      ${note('The clashing slot is a separate example state.')}
      ${action('Choose a different tape', 'gap', true)}`),
  },
  blocked: {
    title:'Choose a different time', label:'Programming calendar · example conflict',
    stock:'dark', back:'calendar',
    left:paper('18:30 · overlaps', `
      ${note('Neighbourhood Laughs · Episode 1 already occupies 18:00–18:46.')}
      <p>The existing airing stays where it is, and the selected tape stays in hand.</p>`),
    right:paper('Try an open slot', `
      ${action('Preview 19:30', 'later')}
      ${action('Return to calendar', 'calendar', true)}`),
  },
  earlier: {
    title:'Preview 17:30', label:'Programming calendar · sample alternate', stock:'dark', back:'calendar',
    left:paper('Placement slip', figures([['Start','17:30'],['Occupied with ads','17:30–18:16'],['Estimated fit','Weak']])),
    right:paper('Decision', '<p>Weak estimated fit can still be a deliberate choice. This sample compares openings before the first airing is placed.</p>'),
  },
  later: {
    title:'Preview 19:30', label:'Programming calendar · sample alternate', stock:'dark', back:'calendar',
    left:paper('Placement slip', figures([['Start','19:30'],['Occupied with ads','19:30–20:16'],['Estimated fit','OK']])),
    right:paper('Decision', `<p>Choose this time if the earlier opening is occupied.</p>${action('Return to the replayed 18:00 showing','calendar',true)}`),
  },
  placed: {
    title:'Neighbourhood Laughs · Episode 1', label:'Airing file · Saturday, Day 1',
    stock:'teal', back:'calendar',
    left:paper('Placement receipt', receipt('Placed','Your channel · 18:00–18:46 · 30 programme minutes and 16 ad minutes.')),
    right:paper('Offer shelf', `
      <div class="card"><strong>Everyday Snacks</strong><small>3 minutes · 511 Funds if aired</small></div>
      ${action('Sign this offer', 'signed')}`),
  },
  signed: {
    title:'Neighbourhood Laughs · Episode 1', label:'Airing file · Saturday, Day 1',
    stock:'teal', back:'placed',
    left:paper('18:00–18:46 · booked', `
      ${figures([['Programme','30 min'],['Ad inventory','16 min'],['Signed to Everyday Snacks','3 min'],['Available','13 min']])}
      ${receipt('Signed','Everyday Snacks · 3 min · 511 Funds if aired.')}`),
    right:paper('After transmission', `
      <p>Open this same file from Today, Programming or Reviews. A signed booking protects the airing’s start.</p>
      ${action('See transmission receipt', 'airing')}`),
  },
  airing: {
    title:'Neighbourhood Laughs · Episode 1', label:'Airing file · Saturday, Day 1 · 18:00',
    stock:'teal', back:'signed',
    left:paper('Before transmission', `
      ${note('About 8,800 potential viewers · survey Sat, Day 1 · 00:00')}
      <p>Dated screening estimate filed before the airing. The receipt carries provisional counts sampled when the show began.</p>
      ${receipt('Signed','Everyday Snacks · 3 minutes · 511 Funds if aired.')}`),
    right:paper('Transmission receipt', `
      <p><strong>About 8,400 tuned in</strong> · provisional opening count</p>
      ${figures([['Young viewers', 'About 3,000'], ['Households', 'About 4,900'], ['Specialists', 'About 470'], ['Ad minutes aired','3 of 3 sold'], ...airingFigures])}
      ${action('Open Ledger', 'ledger')} ${action('Tape bottom line', 'bottom-line', true)}`),
  },
  ledger: {
    title:'Station ledger', label:'Funds · entries for this showing',
    back:'airing',
    left:paper('Saturday · Day 1', figures([['Opening Funds','20,000'],['18:00 · transmission','−150'],['18:46 · ad settlement','+511'],['Closing Funds','20,361']])),
    right:paper('Related papers', `
      <p>Both entries belong to the 18:00 Neighbourhood Laughs airing.</p>
      ${action('Return to 18:00 receipt', 'airing')}
      ${action('Tape bottom line', 'bottom-line', true)}`),
  },
  'bottom-line': {
    title:'Neighbourhood Laughs · Episode 1', label:'Tape file · Bottom line', back:'airing',
    left:paper('This tape so far', figures([['Completed airings in this run','1'],['Ad receipts','511 Funds'],['Airing costs','150 Funds']])),
    right:paper('Investment', `<p>No production investment on file for this catalogue tape.</p>${action('Return to the airing','airing',true)}`),
  },
  market: {
    title:'The Corner Shop', label:'Incoming portfolio · archive offer',
    tabs:tabs(tab('Title index'),tab('Offer',true),tab('Assessment')),
    left:paper('Public evidence', `
      <div class="market-art" role="img" aria-label="Illustration for The Corner Shop"></div>
      <p>90 minutes · 100 Funds to air. An old television guide praises the cast. No recent audience figures.</p>
      ${note('Initial estimate · about 8,500 viewers at 18:00. Rough read.')}
      ${action('See assessment choice', 'assessment', true)}`),
    right:paper('Acquisition slip', `
      ${figures([['Asking price','2,456 Funds'],['Available until','Day 2 · 12:00'],['Funds available','20,000']])}
      ${action('Sign for 2,456 Funds','acquired')}`),
  },
  assessment: {
    title:'The Corner Shop', label:'Market file · assessment draft', back:'market',
    left:paper('Before commissioning','<p>The current evidence stays on the offer until the assessment arrives.</p>'),
    right:paper('Research order', `
      ${figures([['Insight to spend','3'],['Remaining Insight','9'],['Estimate available','Day 1 · 03:00']])}
      ${action('Commission assessment','assessment-result')} ${action('Back to offer','market',true)}`),
  },
  'assessment-result': {
    title:'The Corner Shop', label:'Market file · assessment delivery', back:'market',
    left:paper('Assessment arrived', receipt('Filed','Day 1 · 03:00 · estimate refined. Future turnout remains uncertain.')),
    right:paper('Offer still open', `
      ${figures([['Asking price','2,456 Funds'],['Available until','Day 2 · 12:00']])}
      ${action('Sign for 2,456 Funds','acquired')}`),
  },
  acquired: {
    title:'The Corner Shop', label:'Market file · acquisition receipt', back:'market',
    left:paper('Rights acquisition', receipt('On your shelf','2,456 Funds paid · 17,544 Funds remaining.')),
    right:paper('The tape', tape('The Corner Shop','90 min · catalogue work','acquired-tape')),
  },
  'acquired-tape': {
    title:'The Corner Shop', label:'Tape file · acquired title', back:'acquired',
    left:paper('On the tape','<p>90 minutes · 100 Funds to air.</p>'),
    right:paper('Reception',note('The current title assessment can inform where to look for a gap. The acquisition creates no airing.')),
  },
  research: {
    title:'Young viewers', label:'Research binder · audience file', stock:'olive',
    tabs:tabs('<a href="#surveys">Survey orders</a>',tab('Young viewers',true),tab('Households'),tab('Specialists')),
    left:paper('Published survey', `
      <img class="portrait" src="../../../web/public/audiences/young-viewers.webp" alt="Young viewers illustration">
      <p>Saturday · Day 1 · 00:00 · estimated size and typical age on file</p>
      <p>Example survey note · <strong>High uncertainty</strong></p>
      <p>Humour <span aria-label="four out of five stars">★★★★☆</span> · illustrated preference band</p>`),
    right:paper('Investigate this group', `
      <p><span class="dial" aria-hidden="true">3</span> <strong>3 Insight</strong> · 9 left after confirmation</p>
      ${note('The published survey keeps its date. Better evidence arrives with the next successful survey.')}
      ${action('Spend 3 Insight','investigated')} ${action('Survey orders','surveys',true)}`),
  },
  surveys: {
    title:'Survey orders', label:'Research binder · all groups', stock:'olive', back:'research',
    left:paper('One schedule for the station', figures([['Off', 'No charge'],['Daily', '60 Funds per survey'],['Every 3 days', '90 Funds per survey']])),
    right:paper('What changes','<p>A more frequent survey gives fresher evidence. The separate Insight spend changes the familiarity used the next time this group is surveyed.</p>'),
  },
  investigated: {
    title:'Young viewers', label:'Research binder · investigation receipt', stock:'olive', back:'research',
    left:paper('Published survey','<p>Saturday · Day 1 · 00:00. Still the latest available survey.</p>'),
    right:paper('Investigation', `
      ${receipt('3 Insight spent','9 Insight remaining. Survey polling is off. The result becomes visible with the next successful survey.')}
      ${action('Open survey orders','surveys')}`),
  },
  production: {
    title:'Harbour Lights', label:'Script file · Ideas / Scripts / Production', stock:'pink',
    tabs:tabs(tab('Ideas'),tab('Scripts',true),tab('Production')),
    left:paper('Show folder', `
      <div class="card"><strong>Episode 1</strong><small>Script ready · choose the next action</small></div>
      <div class="card"><strong>Episode 2</strong><small>Idle · commission when ready</small></div>
      <div class="card"><strong>Episode 3</strong><small>Idle · commission when ready</small></div>`),
    right:paper('Script ready', `
      <p>Read the delivered script for Episode 1 and check its demands. Accepting it carries the same episode into casting.</p>
      ${action('Open Episode 1', 'script')}`),
  },
  script: {
    title:'Harbour Lights · Episode 1', label:'Preparation file · script on the desk', stock:'pink', back:'production',
    left:paper('Delivered script', '<p>Writer on record · Ada Mercer</p><p>Review the delivered qualities, shortfall and staging demands on this paper.</p>'),
    right:paper('Decision', `
      <p>Accept the script to proceed to Cast. A paid revision, where available, remains a separate decision.</p>
      ${action('Accept and open Cast', 'booking')}`),
  },
  booking: {
    title:'Harbour Lights · Episode 1', label:'Production file · same episode', stock:'blue', back:'script',
    tabs:tabs(tab('Cast'),tab('Booking',true)),
    left:paper('Cast on the call sheet', `
      <ul class="cast-cards">
        <li><span class="cast-photo" style="--x:0%;--y:100%" aria-hidden="true"></span><b>Lead</b><br>Nina Bell</li>
        <li><span class="cast-photo" style="--x:33.333%;--y:100%" aria-hidden="true"></span><b>Confidant</b><br>Sam Ives</li>
        <li><span class="cast-photo" style="--x:66.667%;--y:0%" aria-hidden="true"></span><b>Director</b><br>Inez Park</li>
      </ul>`),
    right:paper('Studio and recording', `
      <div class="card studio-card"><span class="studio-photo" aria-hidden="true"></span><div><strong>Backlot Room</strong><small>Compact room for dialogue and small scenes.</small></div></div>
      ${figures([['Example recording','Day 2 · 01:30'],['Room','280 Funds'],['Talent','1,160 Funds'],['Total','1,440 Funds']])}
      ${action('Confirm example booking','work-order')}
      ${action('Inspect a conflict','cast-conflict',true)}`),
  },
  'cast-conflict': {
    title:'Recording clash', label:'Production file · example conflict', stock:'blue', back:'booking',
    left:paper('Backlot Room is reserved', note('Example conflict · Day 2 · 01:30–03:30.')),
    right:paper('Choose another time','<p>The cast selections remain in this file. No booking or spending has occurred.</p>'),
  },
  'work-order': {
    title:'Harbour Lights · Episode 1', label:'Production file · example call sheet', stock:'blue', back:'booking',
    left:paper('Work order', receipt('Booked','Backlot Room · Day 2 · 01:30–03:30. 1,440 Funds committed.')),
    right:paper('Delivery', `<p>Follow the reserved episode through post-production, then pull the finished tape.</p>${action('See delivered file','delivered')}`),
  },
  delivered: {
    title:'Harbour Lights · Episode 1', label:'Production file · example delivery', stock:'blue', back:'work-order',
    left:paper('Delivery',receipt('On your shelf','Episode 1 delivered. Episodes 2 and 3 keep their own production state.')),
    right:paper('Next decision','<p>The tape can now be selected for a future airing.</p>'),
  },
};

const root = document.querySelector('#scene');

function render() {
  const key = location.hash.slice(1);
  const scene = scenarios[key] ?? scenarios.rack;
  const back = scene.back ? action('Back', scene.back, true) : '';
  root.innerHTML = `<article class="file" data-stock="${scene.stock ?? 'kraft'}">
    <header class="file-cover"><div><span class="cover-label">${scene.label}</span><h1 tabindex="-1">${scene.title}</h1></div>${back}</header>
    ${scene.tabs ?? ''}${scene.wide ?? ''}<div class="papers">${scene.left}${scene.right}</div>
  </article>`;
  for (const link of document.querySelectorAll('.review-bar a')) {
    if (link.hash === location.hash) link.setAttribute('aria-current', 'page');
    else link.removeAttribute('aria-current');
  }
  if (key) root.querySelector('h1').focus({ preventScroll: true });
}
addEventListener('hashchange', () => { render(); scrollTo(0, 0); });
addEventListener('keydown', event => {
  if (event.key !== 'Escape') return;
  const back = scenarios[location.hash.slice(1)]?.back;
  if (back) location.hash = back;
});
render();
