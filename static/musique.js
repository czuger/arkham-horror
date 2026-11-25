// Background music: one mood per situation, read from #contenu's
// data-ambiance (combat, other_world or default), always cross-faded.
// The two players are made here and never put in the page, so no htmx swap
// can stop them; the button sits outside #contenu.
// Listeners sit on the document, so they keep working after htmx swaps.
(() => {
  "use strict";

  /** Cross-fade length of each mood, in seconds: entering a combat is quick. */
  const FADE_SECONDS = { default: 3, other_world: 4, combat: 1.5 };
  /** Where the play/pause choice is kept between full reloads. */
  const STORAGE_KEY = "arkham.musique";

  /** Track file names by mood, from `musique/pistes`. */
  let tracks = {};
  const tracksLoaded = fetch(new URL("musique/pistes", document.baseURI))
    .then((response) => (response.ok ? response.json() : {}))
    .then((list) => {
      tracks = list;
    })
    .catch((error) => console.warn("musique : liste des pistes illisible", error));

  /** The mood of the page shown, followed even while silent. */
  let wanted = "default";
  /** Whether the player wants music (shown by the button). */
  let on = readChoice();
  let context = null;
  let decks = [];
  /** The deck playing now (or paused), the other one fades out or waits. */
  let current = null;

  /** The mood actually played for `mood`: the default one when it has no track. */
  function playable(mood) {
    return (tracks[mood] || []).length > 0 ? mood : "default";
  }

  /** The fade length for `mood`, at most a third of a track lasting `duration`. */
  function fadeFor(mood, duration) {
    const fade = FADE_SECONDS[mood] || FADE_SECONDS.default;
    return Number.isFinite(duration) && duration > 0 ? Math.min(fade, duration / 3) : fade;
  }

  /** Makes the audio graph: two players, each through its own volume. */
  function setUp() {
    if (context) return;
    const AudioContext = window.AudioContext || window.webkitAudioContext;
    context = new AudioContext();
    decks = [makeDeck(), makeDeck()];
  }

  function makeDeck() {
    const audio = new Audio();
    audio.preload = "auto";
    const gain = context.createGain();
    gain.gain.value = 0;
    context.createMediaElementSource(audio).connect(gain).connect(context.destination);
    const deck = { audio, gain, token: 0, mood: null, track: null, leaving: false };
    audio.addEventListener("timeupdate", () => nearEnd(deck));
    audio.addEventListener("ended", () => {
      if (deck === current && on && !deck.leaving) crossFade(deck.mood);
    });
    return deck;
  }

  /** Ramps a deck's volume to `target` from where it is now, over `seconds`. */
  function ramp(deck, target, seconds) {
    const volume = deck.gain.gain;
    const now = context.currentTime;
    volume.cancelScheduledValues(now);
    volume.setValueAtTime(volume.value, now);
    volume.linearRampToValueAtTime(target, now + seconds);
  }

  /** Fades a deck in; a pause planned by an earlier fade-out is called off. */
  function fadeIn(deck, seconds) {
    deck.token += 1;
    deck.audio.play().catch((error) => console.warn("musique : lecture refusée", error));
    ramp(deck, 1, seconds);
  }

  /** Fades a deck out, then pauses it unless it was started again meanwhile. */
  function fadeOut(deck, seconds) {
    ramp(deck, 0, seconds);
    deck.token += 1;
    const token = deck.token;
    setTimeout(() => {
      if (deck.token === token) deck.audio.pause();
    }, seconds * 1000 + 100);
  }

  /** A random track of `mood`, not `avoid` when there is another one. */
  function pick(mood, avoid) {
    const list = tracks[mood] || [];
    const others = list.length > 1 ? list.filter((name) => name !== avoid) : list;
    return others[Math.floor(Math.random() * others.length)];
  }

  function trackUrl(mood, name) {
    return new URL(`musique/${mood}/${encodeURIComponent(name)}`, document.baseURI).href;
  }

  /** Fades the current track out while a random track of `mood` fades in. */
  function crossFade(mood, seconds) {
    const old = current;
    const name = pick(mood, old && old.mood === mood ? old.track : null);
    if (!name) return;
    const fade = seconds || fadeFor(mood);
    const next = decks.find((deck) => deck !== old);
    next.token += 1;
    next.audio.src = trackUrl(mood, name);
    next.mood = mood;
    next.track = name;
    next.leaving = false;
    next.gain.gain.cancelScheduledValues(context.currentTime);
    next.gain.gain.setValueAtTime(0, context.currentTime);
    fadeIn(next, fade);
    if (old) {
      old.leaving = true;
      fadeOut(old, fade);
    }
    current = next;
  }

  /** Near the end of the current track, cross-fades to the next one. */
  function nearEnd(deck) {
    if (deck !== current || !on || deck.leaving) return;
    const duration = deck.audio.duration;
    if (!Number.isFinite(duration) || duration <= 0) return;
    const fade = fadeFor(deck.mood, duration);
    if (duration - deck.audio.currentTime <= fade) {
      deck.leaving = true;
      crossFade(deck.mood, fade);
    }
  }

  /** Follows a new mood: nothing changes when it is the one playing. */
  function changerAmbiance(mood) {
    if (!Object.hasOwn(FADE_SECONDS, mood)) return;
    wanted = mood;
    if (!on || !context) return;
    const target = playable(mood);
    if (current && current.mood === target) return;
    crossFade(target);
  }

  /** Starts the music; must run during a click or a key press. */
  async function start() {
    setUp();
    await context.resume();
    await tracksLoaded;
    if (!on) return;
    const target = playable(wanted);
    if (current && current.mood === target && current.audio.src) {
      current.leaving = false;
      fadeIn(current, fadeFor(target, current.audio.duration));
    } else {
      crossFade(target);
    }
    unlock();
  }

  /** Lets the waiting deck play later without a click (needed on iOS). */
  function unlock() {
    const waiting = decks.find((deck) => deck !== current);
    if (!waiting || waiting.audio.src) return;
    const name = pick(playable(wanted));
    if (!name) return;
    waiting.audio.src = trackUrl(playable(wanted), name);
    const token = waiting.token;
    waiting.audio
      .play()
      .then(() => {
        if (waiting.token === token) waiting.audio.pause();
      })
      .catch((error) => console.warn("musique : lecteur bloqué", error));
  }

  function stop() {
    decks.forEach((deck) => {
      if (!deck.audio.paused) fadeOut(deck, fadeFor(deck.mood, deck.audio.duration));
    });
  }

  async function toggle() {
    on = !on;
    saveChoice();
    show();
    if (on) {
      await start();
    } else if (context) {
      stop();
    }
  }

  /** Shows the player's choice on the button. */
  function show() {
    const button = document.getElementById("musique");
    if (!button) return;
    button.setAttribute("aria-pressed", String(on));
    button.title = on ? "Couper la musique" : "Jouer la musique d'ambiance";
  }

  function readChoice() {
    try {
      return window.localStorage.getItem(STORAGE_KEY) === "on";
    } catch {
      return false;
    }
  }

  function saveChoice() {
    try {
      window.localStorage.setItem(STORAGE_KEY, on ? "on" : "off");
    } catch {
      // Storage refused (private window): the choice lasts until reload.
    }
  }

  /** Reads the mood of the page part shown. */
  function readPage() {
    const content = document.getElementById("contenu");
    if (content && content.dataset.ambiance) changerAmbiance(content.dataset.ambiance);
    show();
  }

  document.addEventListener("click", (event) => {
    if (event.target instanceof Element && event.target.closest("#musique")) {
      toggle().catch((error) => console.warn("musique :", error));
    }
  });
  document.addEventListener("htmx:after:settle", readPage);
  document.addEventListener("changerAmbiance", (event) => {
    changerAmbiance(event.detail && event.detail.ambiance);
  });
  // After a full reload, a browser plays nothing before a click: music
  // chosen earlier starts at the first click or key press.
  if (on) {
    const resume = () => {
      document.removeEventListener("pointerdown", resume, true);
      document.removeEventListener("keydown", resume, true);
      start().catch((error) => console.warn("musique :", error));
    };
    document.addEventListener("pointerdown", resume, true);
    document.addEventListener("keydown", resume, true);
  }
  readPage();
})();
