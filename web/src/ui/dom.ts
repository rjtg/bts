/** Small shared DOM primitives. Text is always assigned as text, never parsed markup. */
export function element<T extends HTMLElement = HTMLElement>(id: string): T {
  const result = document.getElementById(id);
  if (!result) throw new Error(`Missing UI element: ${id}`);
  return result as T;
}
export const select = (id: string) => element<HTMLSelectElement>(id);
export const button = (id: string) => element<HTMLButtonElement>(id);
export function node<K extends keyof HTMLElementTagNameMap>(tag: K, text = '', className = ''): HTMLElementTagNameMap[K] {
  const result = document.createElement(tag); result.textContent = text; result.className = className; return result;
}
export function option(value: number | string, text: string): HTMLOptionElement { const result = node('option',text); result.value=String(value); return result; }
export function action(label: string, callback: () => void, className = 'quiet'): HTMLButtonElement {
  const result=node('button',label,className); result.type='button'; result.addEventListener('click',callback); return result;
}
