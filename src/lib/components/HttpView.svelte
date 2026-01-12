<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import TextParser from "./TextParser.svelte";

  let { parsedText = $bindable() }: { parsedText: string[] } = $props();

  let url = $state("");
  let loading = $state(false);
  let error = $state("");
  let fetchedText = $state("");

  async function handleSubmit(event: Event) {
    event.preventDefault();
    if (!url) return;

    loading = true;
    error = "";
    try {
      fetchedText = await invoke<string>("fetch_url", { url });
    } catch (e) {
      error = e as string;
      fetchedText = "";
    } finally {
      loading = false;
    }
  }
</script>

<div class="view">
  <form class="url-form" onsubmit={handleSubmit}>
    <input
      type="url"
      placeholder="Enter URL..."
      bind:value={url}
      disabled={loading}
    />
    <button type="submit" disabled={loading || !url}>
      {loading ? "Fetching..." : "Fetch"}
    </button>
  </form>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if fetchedText}
    <div class="text-parser-wrapper">
      <TextParser bind:parsedText initialText={fetchedText} />
    </div>
  {/if}
</div>

<style>
  .view {
    width: 80%;
  }

  .url-form {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .url-form input {
    flex: 1;
    padding: 0.6em 1.2em;
    font-size: 1em;
    border: 1px solid transparent;
    background-color: #ffffff;
    color: #0f0f0f;
  }


  .url-form button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .error {
    color: #cc0000;
    margin-bottom: 1rem;
    padding: 0.5rem;
    background-color: rgba(204, 0, 0, 0.1);
    border-radius: 4px;
  }

  .text-parser-wrapper {
    --textarea-height: 40vh;
  }

  @media (prefers-color-scheme: dark) {
    .url-form input {
      background-color: #0f0f0f98;
      color: #ffffff;
    }

    .error {
      color: #ff6666;
      background-color: rgba(255, 102, 102, 0.1);
    }
  }
</style>
