// Skill adjustment page: counts the stops moved against the Concentration,
// shows the skill values picked, and blocks a form that asks too much.
// Listeners sit on the document, so they keep working after htmx swaps.
(() => {
  "use strict";

  /** Recounts the stops moved on the adjustment form, if any. */
  function refresh() {
    const form = document.getElementById("adjustment");
    if (!form) return;
    let moved = 0;
    form.querySelectorAll("input[type=radio]:checked").forEach((radio) => {
      moved += Math.abs(Number(radio.value) - Number(radio.dataset.current));
      const shown = form.querySelector(`output[data-track="${radio.name}"]`);
      if (shown) shown.textContent = radio.dataset.label;
    });
    const used = document.getElementById("focus-used");
    const tooMuch = moved > Number(form.dataset.focus);
    used.textContent = String(moved);
    used.parentElement.classList.toggle("too-much", tooMuch);
    document.getElementById("adjust-submit").disabled = tooMuch;
  }

  document.addEventListener("change", (event) => {
    if (event.target instanceof Element && event.target.closest("#adjustment")) refresh();
  });
  // The form is reset after the event: recount once it is done.
  document.addEventListener("reset", () => setTimeout(refresh, 0));
  document.addEventListener("DOMContentLoaded", refresh);
  document.addEventListener("htmx:load", refresh);
})();
