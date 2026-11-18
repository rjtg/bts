import { action } from './dom';
/** Single-select buttons with stable focus after a parent rerender. */
export function renderChoices<T>(root: HTMLElement, options: { value:T; label:string }[], selected:T, onChange:(value:T)=>void): void {
  root.replaceChildren(...options.map((choice,index)=>{
    const button=action(choice.label,()=>{
      onChange(choice.value);
      document.getElementById(root.id)?.querySelector<HTMLButtonElement>(`[data-choice="${index}"]`)?.focus();
    });
    button.dataset.choice=String(index); button.setAttribute('aria-pressed',String(choice.value===selected)); return button;
  }));
}
