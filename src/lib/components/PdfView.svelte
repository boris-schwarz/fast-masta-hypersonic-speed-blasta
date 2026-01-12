<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { onMount } from "svelte";
    import TextParser from "./TextParser.svelte";

  let { parsedText = $bindable() }: { parsedText: string[] } = $props();

  let selectedFile: File | null = $state(null);
  let droppedFilePath: string | null = $state(null);
  let dragging = $state(false);
  let loading = $state(false);
  let extractedText = $state("");

  async function loadFromPath(path: string) {
    loading = true;
    try {
      extractedText = await invoke<string>("extract_pdf_text_from_path", { path });
    } finally {
      loading = false;
    }
  }

  async function loadFromFile(file: File) {
    loading = true;
    try {
      const arrayBuffer = await file.arrayBuffer();
      const bytes = Array.from(new Uint8Array(arrayBuffer));
      extractedText = await invoke<string>("extract_pdf_text", { bytes });
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    const webview = getCurrentWebviewWindow();

    const unlisten = Promise.all([
      webview.onDragDropEvent((event) => {
        if (event.payload.type === "over") {
          dragging = true;
        } else if (event.payload.type === "leave") {
          dragging = false;
        } else if (event.payload.type === "drop") {
          dragging = false;
          const paths = event.payload.paths;
          if (paths.length > 0 && paths[0].endsWith(".pdf")) {
            droppedFilePath = paths[0];
            selectedFile = null;
            loadFromPath(paths[0]);
          }
        }
      })
    ]);

    return () => {
      unlisten.then((listeners) => listeners.forEach((fn) => fn()));
    };
  });

  function handleFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (file) {
      selectedFile = file;
      droppedFilePath = null;
      loadFromFile(file);
    }
  }

  function getDisplayName(): string {
    if (dragging) return "Drop PDF here...";
    if (droppedFilePath) return droppedFilePath.split(/[/\\]/).pop() || droppedFilePath;
    if (selectedFile) return selectedFile.name;
    return "Choose or drop PDF file...";
  }
</script>

<div class="view">
  <label class="file-input" class:dragging class:loading class:expanded={!extractedText}>
    <input
      type="file"
      accept=".pdf"
      onchange={handleFileChange}
      disabled={loading}
    />
    <span class="file-label">{loading ? "Loading..." : getDisplayName()}</span>
  </label>
  {#if extractedText}
    <div class="text-parser-wrapper">
      <TextParser bind:parsedText initialText={extractedText} />
    </div>
  {/if}
</div>

<style>
  .view {
    width: 80%;
  }

  .text-parser-wrapper {
    --textarea-height: 40vh;
  }

  .file-input {
    margin-bottom: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    min-height: 10vh;
    border: 2px dashed rgba(0, 0, 0, 0.2);
    border-radius: 8px;
    cursor: pointer;
    transition: border-color 0.2s, background-color 0.2s;

    &:hover {
      border-color: rgba(0, 0, 0, 0.4);
    }

    &.dragging {
      border-color: #646cff;
      background-color: rgba(100, 108, 255, 0.1);
    }

    input {
      display: none;
    }
  }

  .file-label {
    font-size: 1.25rem;
    opacity: 0.7;
  }

  .file-input.loading {
    opacity: 0.5;
    cursor: wait;
  }

  .file-input.expanded {
    min-height: 60vh;
  }

  @media (prefers-color-scheme: dark) {
    .file-input {
      border-color: rgba(255, 255, 255, 0.2);

      &:hover {
        border-color: rgba(255, 255, 255, 0.4);
      }
    }
  }
</style>
