import { node } from './dom';
// Bounded scenario art mapping; no identity/skill/eligibility is inferred from an image.
const artwork:Record<number,string>={1:'elian-north',2:'mina-sol',3:'nora-vale',4:'felix-reed',5:'jules-hart',6:'rafi-linden'};
export function personPortrait(id:number,name:string):HTMLImageElement|null {
  if(!artwork[id])return null;
  const img=node('img','','talent-portrait');img.src=`${import.meta.env.BASE_URL}portraits/${artwork[id]}.png`;img.alt=`Portrait of ${name}`;img.width=96;img.height=96;return img;
}
