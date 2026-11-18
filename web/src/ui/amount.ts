import { node } from './dom';
/** Linked whole-number range and exact entry. Bounds are UI guidance; Rust validates actions. */
export function amountControl(options:{ id:string; exactId?:string; label:string; maximum:number; value:number; onChange:(value:number,valid:boolean)=>void }) {
  const root=node('div','','amount-control');
  const range=node('input'); range.type='range'; range.id=options.id; range.min='0'; range.max=String(options.maximum); range.step='1'; range.value=String(options.value); range.setAttribute('aria-label',options.label);
  const number=node('input'); number.type='number'; number.id=options.exactId??`${options.id}-exact`; number.min='0'; number.max=range.max; number.step='1'; number.value=String(options.value); number.setAttribute('aria-label',`Exact ${options.label.toLowerCase()}`);
  const valid=()=>Number.isInteger(number.valueAsNumber)&&number.valueAsNumber>=0&&number.valueAsNumber<=options.maximum;
  const update=(source:HTMLInputElement)=>{
    if(source===range)number.value=range.value;
    else if(Number.isInteger(number.valueAsNumber))range.value=number.value;
    options.onChange(number.valueAsNumber,valid());
  };
  range.addEventListener('input',()=>update(range)); number.addEventListener('input',()=>update(number)); root.append(range,number);
  return { element:root, valid };
}
