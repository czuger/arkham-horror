// Links and forms only swap the page part #contenu (htmx boost, see
// base.html), so the music never stops. A response without #contenu (an
// error message, a file) would wipe that part: it is handled here instead.

(() => {
  "use strict";

  document.addEventListener("htmx:before:swap", (event) => {
    const ctx = event.detail && event.detail.ctx;
    if (!ctx || !ctx.response || typeof ctx.text !== "string") return;
    const kind = ctx.response.headers.get("content-type") || "";
    if (!kind.startsWith("text/html")) {
      event.preventDefault();
      if (ctx.response.status < 400) {
        window.location.assign(ctx.response.raw.url);
      } else {
        showError(ctx.text);
      }
      return;
    }
    if (!ctx.text.includes('id="contenu"')) {
      event.preventDefault();
      showError(ctx.text);
    }
  });

  // Puts the server's error message in place of the page content.
  function showError(text) {
    const content = document.getElementById("contenu");
    if (!content) return;
    const message = document.createElement("p");
    message.className = "error";
    message.textContent = text.trim() || "Erreur inattendue.";
    const back = document.createElement("a");
    back.href = window.location.href;
    back.textContent = "Recharger la page";
    content.replaceChildren(message, back);
  }
})();
