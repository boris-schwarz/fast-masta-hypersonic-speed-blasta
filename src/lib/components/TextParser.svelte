<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let { parsedText = $bindable(), initialText = "" }: { parsedText: string[], initialText?: string } = $props();

  let text = $state(initialText);
  let loading = $state(false);
  let hasChanges = $derived(initialText !== "" && text !== initialText);

  $effect(() => {
    if (initialText) {
      text = initialText;
    }
  });

  function reset() {
    text = initialText;
  }

  async function handleSubmit(event: Event) {
    event.preventDefault();
    loading = true;
    try {
      parsedText = await invoke<string[]>("parse_text", { text });
    } finally {
      loading = false;
    }
  }
</script>

<form class="text-form" onsubmit={handleSubmit}>
  <textarea
    placeholder="Paste text here..."
    bind:value={text}
    onkeydown={(e) => { if (e.ctrlKey && e.key === "Enter") e.currentTarget.form?.requestSubmit(); }}
  ></textarea>
  <div class="form-footer">
    <span class="hint">Ctrl+Enter to submit</span>
    {#if hasChanges}
      <button type="button" class="reset-btn" onclick={reset}>Reset to original</button>
    {/if}
    <button type="submit" disabled={loading || !text}>
      {loading ? "Loading..." : "READ!"}
    </button>
  </div>
</form>

<style>
  .text-form {
    display: flex;
    flex-direction: column;
    width: 100%;
    max-width: 800px;
  }

  .form-footer {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    width: 100%;
    gap: 1rem;
  }

  .hint {
    opacity: 0.5;
    font-size: 0.85em;
    user-select: none;
    cursor: default;
  }

  textarea {
    width: 100%;
    height: var(--textarea-height, 60vh);
    margin-bottom: 1rem;
    resize: vertical;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    outline: none;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .reset-btn {
    opacity: 0.7;
    transition: opacity 0.2s;

    &:hover {
      opacity: 1;
    }
  }

  @media (prefers-color-scheme: dark) {
    textarea {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
  }
</style>
