// Drag a card between the « En main » and « Possessions » zones of the
// investigator page. Dropping it presses the card's own button, so the move
// is still a plain form answer checked by the server.
// Listeners sit on the document, so they keep working after htmx swaps.
(() => {
  "use strict";

  let dragged = null;

  /** The zone under the pointer, when the dragged card may land there. */
  function landing(event) {
    const zone = event.target instanceof Element ? event.target.closest("[data-zone]") : null;
    return zone && dragged && zone.dataset.zone === dragged.dataset.move ? zone : null;
  }

  function clearHighlights() {
    document.querySelectorAll("[data-zone].over").forEach((zone) => zone.classList.remove("over"));
  }

  document.addEventListener("dragstart", (event) => {
    const card = event.target instanceof Element ? event.target.closest("[data-move]") : null;
    if (!card) return;
    dragged = card;
    event.dataTransfer.effectAllowed = "move";
    event.dataTransfer.setData("text/plain", card.dataset.move);
    card.classList.add("dragging");
  });

  document.addEventListener("dragend", () => {
    if (dragged) dragged.classList.remove("dragging");
    dragged = null;
    clearHighlights();
  });

  document.addEventListener("dragover", (event) => {
    const zone = landing(event);
    if (!zone) return;
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
    zone.classList.add("over");
  });

  document.addEventListener("dragleave", (event) => {
    const zone = event.target instanceof Element ? event.target.closest("[data-zone]") : null;
    if (zone && !zone.contains(event.relatedTarget)) zone.classList.remove("over");
  });

  document.addEventListener("drop", (event) => {
    const zone = landing(event);
    if (!zone) return;
    event.preventDefault();
    clearHighlights();
    const button = dragged.querySelector("button[name=choix]");
    if (button) button.click();
  });
})();
