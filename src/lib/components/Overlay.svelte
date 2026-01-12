<script lang="ts">
  import Player from "./Player.svelte";

  let { content = $bindable() }: { content: string[] } = $props();

  let player: ReturnType<typeof Player>;

  function close() {
    content = [];
    player?.reset();
  }
</script>

{#if content.length > 0}
  <div class="overlay" onclick={close}>
    <div class="overlay-content" onclick={(e) => e.stopPropagation()}>
      <button class="overlay-close" onclick={close} title="Close (Esc)">&times;</button>
      <Player bind:this={player} {content} />
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .overlay-content {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: #ffffff;
    padding: 2rem;
    width: 98%;
    height: 98%;
    overflow: auto;
    opacity: 0.95;
    overflow: hidden;
  }

  .overlay-close {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    font-size: 1.5rem;
    line-height: 1;
    padding: 0.25rem 0.5rem;
    box-shadow: none;
  }

  @media (prefers-color-scheme: dark) {
    .overlay-content {
      background-color: #3f3f3f;
    }
  }
</style>
