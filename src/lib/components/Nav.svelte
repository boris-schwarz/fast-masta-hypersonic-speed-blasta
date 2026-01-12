<script lang="ts">
  let { selectedView = $bindable() }: { selectedView: number } = $props();
</script>

<nav>
  <ul>
    <li><a href="/" class:active={selectedView === 1} onclick={() => selectedView = 1}><span class="number">1</span>Text</a></li>
    <li><a href="/" class:active={selectedView === 2} onclick={() => selectedView = 2}><span class="number">2</span>PDF</a></li>
    <li><a href="/" class:active={selectedView === 3} onclick={() => selectedView = 3}><span class="number">3</span>Website</a></li>
  </ul>
</nav>

<style>
  nav {
    width: clamp(120px, 20vw, 240px);
    height: 100%;
  }

  ul {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  li {
    flex: 1;
    display: flex;
  }

  a {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    font-weight: 500;
    color: #646cff;
    text-decoration: none;
    background-color: rgba(0, 0, 0, 0.05);
    transition: background-color 0.2s;

    &:hover {
      color: #535bf2;
      background-color: rgba(0, 0, 0, 0.1);
    }
  }

  .number {
    position: absolute;
    top: 4px;
    right: 6px;
    font-size: 0.8em;
    opacity: 0.7;
  }

  @media (prefers-color-scheme: dark) {
    a {
      background-color: rgba(255, 255, 255, 0.05);

      &:hover {
        color: #24c8db;
        background-color: rgba(255, 255, 255, 0.1);
      }
    }
  }
</style>
