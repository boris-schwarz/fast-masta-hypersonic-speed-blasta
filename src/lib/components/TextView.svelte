<script lang="ts">
  import TextParser from "./TextParser.svelte";

  let { parsedText = $bindable() }: { parsedText: string[] } = $props();
</script>

<div class="view">
  <TextParser bind:parsedText />
</div>

<style>
  .view {
    width: 80%;
  }
</style>
