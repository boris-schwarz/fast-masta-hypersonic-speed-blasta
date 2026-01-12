<script lang="ts">
  import Nav from "$lib/components/Nav.svelte";
  import Header from "$lib/components/Header.svelte";
  import TextView from "$lib/components/TextView.svelte";
  import PdfView from "$lib/components/PdfView.svelte";
  import HttpView from "$lib/components/HttpView.svelte";
  import Overlay from "$lib/components/Overlay.svelte";

  let parsedText: string[] = $state([]);
  let selectedView = $state(1);

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape" && parsedText.length > 0) {
      parsedText = [];
      return;
    }

    const target = event.target as HTMLElement;
    if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") {
      return;
    }

    const num = parseInt(event.key);
    if (num >= 1 && num <= 3) {
      selectedView = num;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="wrapper">
  <Nav bind:selectedView />
  <main class="container">
    <Header />

    {#if selectedView === 1}
      <TextView bind:parsedText />
    {/if}

    {#if selectedView === 2}
      <PdfView bind:parsedText />
    {/if}

    {#if selectedView === 3}
      <HttpView bind:parsedText />
    {/if}

  </main>
</div>

<Overlay bind:content={parsedText} />

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.wrapper {
  display: flex;
  justify-content: flex-start;
  align-items: top;
  height: 100vh;
}

.view {
  width: 80%;
}

.container {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}



.row {
  display: flex;
  justify-content: center;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
