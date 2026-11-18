import { node } from './dom';
const definitions = [
  { key:'cash',name:'Cash',valueId:'cash',help:'Cash · spendable integer credits after commitments.',art:'<path d="M2 6h20v12H2zM5 3h14M5 21h14"/><circle cx="12" cy="12" r="3"/>' },
  { key:'sway',name:'Sway',valueId:'resource-sway',help:'Sway · pooled cooperation earned from completed quests.',art:'<path d="m2 10 5-5 5 2 5-2 5 5-7 9h-5l-8-9Zm5-5 5 7 4-3M7 15l3-3m0 6 3-3"/>' },
  { key:'glamour',name:'Glamour',help:'Glamour · planned award-driven prestige; not yet available.',art:'<path d="m12 2 3 7 7 3-7 3-3 7-3-7-7-3 7-3Z"/>' },
  { key:'credibility',name:'Credibility',help:'Credibility · planned reporting authority; not yet available.',art:'<path d="m12 2 8 3v7c0 5-8 10-8 10S4 17 4 12V5l8-3Zm-5 9 3 3 7-7"/>' },
  { key:'backing',name:'Backing',help:'Backing · planned support for an editorial position; not yet available.',art:'<circle cx="12" cy="6" r="3"/><path d="M7 21v-5c0-6 10-6 10 0v5M3 21v-5m18 5v-5M3 7v5m18-5v5"/>' },
];
/** One declarative HUD, with keyboard tooltips. Unimplemented resources never get fake balances. */
export function createResourceHud(root:HTMLElement) {
  const values=new Map<string,HTMLElement>();
  for(const resource of definitions) {
    const item=node('div','',`resource ${resource.key}${resource.valueId?'':' unavailable'}`);item.tabIndex=0;item.setAttribute('aria-label',`${resource.name}${resource.valueId?'':' unavailable'}`);item.setAttribute('aria-describedby',`${resource.key}-tip`);
    const icon=document.createElementNS('http://www.w3.org/2000/svg','svg');icon.setAttribute('viewBox','0 0 24 24');icon.setAttribute('aria-hidden','true');
    icon.innerHTML=resource.art; // Static code-authored SVG only; never user or simulation text.
    const value=node('strong','—');if(resource.valueId)value.id=resource.valueId;values.set(resource.key,value);
    const tip=node('span',resource.help,'resource-tooltip');tip.id=`${resource.key}-tip`;tip.setAttribute('role','tooltip');item.append(icon,value,tip);root.append(item);
  }
  return { render(known:{cash:string;sway:string}) { for(const [key,value] of Object.entries(known))values.get(key)!.textContent=value; } };
}
