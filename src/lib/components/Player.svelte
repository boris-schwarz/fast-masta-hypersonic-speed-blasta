<script lang="ts">
  let { content }: { content: string[] } = $props();

  let currentIndex = $state(0);
  let playing = $state(false);
  let wordsPerMinute = $state(300);
  let selectedFont = $state("system-ui");
  let fontSize = $state(4);

  const fontFamilies = [
    { name: "System", value: "system-ui" },
    { name: "Sans-serif", value: "Verdana, Arial, Helvetica, sans-serif" },
    { name: "Serif", value: "Georgia, 'Times New Roman', serif" },
    { name: "Monospace", value: "Consolas, monospace" },
    { name: "OpenDyslexic", value: "OpenDyslexic, sans-serif" },
  ];

  // intervalMs = 60000ms / WPM
  let intervalMs = $derived(60000 / wordsPerMinute);

  $effect(() => {
    if (!playing) return;

    const interval = setInterval(() => {
      if (currentIndex < content.length - 1) {
        currentIndex++;
      } else {
        playing = false;
      }
    }, intervalMs);

    return () => clearInterval(interval);
  });

  function previous() {
    if (currentIndex > 0) {
      currentIndex--;
    }
  }

  function next() {
    if (currentIndex < content.length - 1) {
      currentIndex++;
    }
  }

  function togglePlay() {
    if (!playing && currentIndex >= content.length - 1) {
      currentIndex = 0;
    }
    playing = !playing;
  }

  function restart() {
    currentIndex = 0;
    playing = false;
  }

  function jumpToEnd() {
    currentIndex = content.length - 1;
    playing = false;
  }

  export function reset() {
    currentIndex = 0;
    playing = false;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.code === "Space") {
      event.preventDefault();
      togglePlay();
    } else if (event.code === "ArrowLeft") {
      previous();
    } else if (event.code === "ArrowRight") {
      next();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="player-container">
  <div class="word-area">
    <div class="word-container" onclick={next} style="font-family: {selectedFont}; font-size: {fontSize}rem">
      <p>{@html content[currentIndex]}</p>
    </div>
  </div>
  <div class="controls-wrapper">
    <div class="controls">
      <button class="control-btn" onclick={restart} title="Restart">⏮</button>
      <button class="control-btn" onclick={previous} title="Previous (←)">◂</button>
      <button class="control-btn" onclick={togglePlay} title="Play/Pause (Space)">
        {playing ? "⏸" : "▶"}
      </button>
      <button class="control-btn" onclick={next} title="Next (→)">▸</button>
      <button class="control-btn" onclick={jumpToEnd} title="Jump to end">⏭</button>
    </div>
    <div class="speed-control">
      <label for="speed">{wordsPerMinute} WPM</label>
      <input type="range" id="speed" min="100" max="900" step="25" bind:value={wordsPerMinute} />
    </div>
    <div class="font-control">
      <label for="font">Font</label>
      <select id="font" bind:value={selectedFont}>
        {#each fontFamilies as font}
          <option value={font.value}>{font.name}</option>
        {/each}
      </select>
    </div>
    <div class="size-control">
      <label for="size">{fontSize}rem</label>
      <input type="range" id="size" min="2" max="8" step="0.5" bind:value={fontSize} />
    </div>
  </div>
</div>

<style>
  .player-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
  }

  .word-area {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .controls-wrapper {
    opacity: 0.4;
    transition: opacity 0.2s ease;
    padding: 1rem 0;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .controls-wrapper:hover {
    opacity: 1;
  }

  .word-container {
    position: relative;
    min-width: 80%;
    border-top: 1px solid rgba(0, 0, 0, 0.4);
    border-bottom: 1px solid rgba(0, 0, 0, 0.4);
    padding: 3rem 0;
    cursor: default;
    user-select: none;

    &::before,
    &::after {
      content: "";
      position: absolute;
      left: 50%;
      width: 1px;
      background-color: rgba(0, 0, 0, 0.4);
    }

    &::before {
      top: 0;
      height: 2.5rem;
    }

    &::after {
      bottom: 0;
      height: 2.5rem;
    }

    :global(p) {
      display: flex;
      justify-content: center;
    }

    :global(.before),
    :global(.after) {
      flex: 1;
      min-width: 0;
    }

    :global(.before) {
      text-align: right;
    }

    :global(.after) {
      text-align: left;
    }

    :global(.highlight) {
      color: #cc5500;
    }
  }

  .controls {
    display: flex;
    justify-content: center;
    gap: 1rem;
  }

  .control-btn {
    width: 36px;
    height: 36px;
    padding: 0;
    font-size: 1rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .speed-control {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-top: 1rem;
  }

  .speed-control label {
    min-width: 5rem;
  }

  .speed-control input[type="range"] {
    width: 150px;
  }

  .font-control {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-top: 1rem;
  }

  .font-control label {
    min-width: 5rem;
  }

  .font-control select {
    padding: 0.25rem 0.5rem;
    font-size: 1rem;
  }

  .size-control {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-top: 1rem;
  }

  .size-control label {
    min-width: 5rem;
  }

  .size-control input[type="range"] {
    width: 150px;
  }

  @media (prefers-color-scheme: dark) {
    .word-container {
      border-top-color: rgba(255, 255, 255, 0.4);
      border-bottom-color: rgba(255, 255, 255, 0.4);

      &::before,
      &::after {
        background-color: rgba(255, 255, 255, 0.4);
      }
    }
  }
</style>
