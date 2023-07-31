<script lang="ts">
  import './App.css';
  import svelteLogo from './assets/svelte.svg';
  import viteLogo from './assets/vite.svg';
  import azleLogo from './assets/azle_logo.svg';
  import azleShadow from './assets/azle_shadow.png';
  import Counter from './components/Counter.svelte';
</script>

<main>
    <div>
        <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
          <img src={viteLogo} class="logo" alt="Vite Logo" />
        </a>
        <a href="https://svelte.dev" target="_blank" rel="noreferrer">
          <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
        </a>
        <a
          href="https://github.com/demergent-labs/azle"
          target="_blank"
        >
          <span class="logo-stack">
            <img
              src={azleShadow}
              class="logo azle-shadow"
              alt="azle logo"
            />
            <img src={azleLogo} class="logo azle" alt="Azle logo" />
          </span>
        </a>
      </div>
      <h1>Vite + Svelte + Azle </h1>
    
     <Counter />
    
    
      <p class="read-the-docs">
        Click on the Vite, Svelte, and Azle logos to learn more
      </p>
</main>

<style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</style>
