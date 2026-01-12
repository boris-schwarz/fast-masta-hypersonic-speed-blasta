<script>
  let { children } = $props();
</script>

{@render children()}

<style>
  :global(*),
  :global(*::before),
  :global(*::after) {
    box-sizing: border-box;
  }

  :global(body, h1, h2, h3, h4, h5, h6, p, figure, blockquote, dl, dd) {
    margin: 0;
  }

  :global(ul, ol) {
    margin: 0;
    padding: 0;
    list-style: none;
  }

  :global(html) {
    -webkit-text-size-adjust: 100%;
  }

  :global(body) {
    min-height: 100vh;
    line-height: 1.5;
  }

  :global(img, picture, video, canvas, svg) {
    display: block;
    max-width: 100%;
  }

  :global(input, button, textarea, select) {
    font: inherit;
  }

  :global(a) {
    color: #646cff;
    text-decoration: none;
  }
  :global(a:hover) {
    color: #535bf2;
  }
</style>
