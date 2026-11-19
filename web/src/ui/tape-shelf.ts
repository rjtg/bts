import { button, element } from './dom';

export function initialiseTapeShelf() {
  const narrow = matchMedia('(max-width: 700px)');
  const shelf = element<HTMLDetailsElement>('library');
  const home = shelf.parentElement!;
  const picker = element<HTMLDialogElement>('library-picker');
  const shelfButton = button('open-library');

  function layout() {
    if (narrow.matches) picker.append(shelf);
    else {
      picker.close();
      home.prepend(shelf);
    }
  }

  function open() {
    shelf.open = true;
    if (narrow.matches && !picker.open) picker.showModal();
  }

  shelfButton.addEventListener('click', open);
  button('close-library').addEventListener('click', () => picker.close());
  picker.addEventListener('close', () => {
    if (narrow.matches && !home.closest<HTMLElement>('[data-desk]')?.hidden) {
      shelfButton.focus({ preventScroll: true });
    }
  });
  narrow.addEventListener('change', layout);
  layout();
  return { open, close: () => picker.close() };
}
