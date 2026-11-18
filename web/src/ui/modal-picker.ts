/** A native keyboard/touch modal with explicit return focus; item contents belong to the caller. */
export function modalPicker<T>(options:{ dialog:HTMLDialogElement; title:HTMLElement; list:HTMLElement; closeButton:HTMLElement; renderItem:(item:T)=>HTMLElement; restoreFocus:()=>void }) {
  const close=()=>options.dialog.close();
  options.closeButton.addEventListener('click',close); options.dialog.addEventListener('close',options.restoreFocus);
  return { close, open(title:string,items:T[]) {
    options.title.textContent=title; options.list.replaceChildren(...items.map(options.renderItem)); options.dialog.showModal(); options.closeButton.focus();
  } };
}
