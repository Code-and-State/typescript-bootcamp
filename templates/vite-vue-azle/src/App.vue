<script setup lang="ts">
import './App.css';
import Counter from './components/Counter.vue';
</script>

<template>
  <div>
    <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
      <img src="./assets/vite.svg" class="logo" alt="Vite Logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank" rel="noreferrer">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue Logo" />
    </a>
    <a href="https://github.com/demergent-labs/azle" target="_blank">
      <span class="logo-stack">
        <img src="./assets/azle_shadow.png" class="logo azle-shadow" alt="azle logo" />
        <img src="./assets/azle_logo.svg" class="logo azle" alt="Azle logo" />
      </span>
    </a>

    <h1>Vite + Vue + Azle</h1>

    <Counter />

    <p class="read-the-docs">
      Click on the Vite, Svelte, and Azle logos to learn more
    </p>
  </div>
</template>

<style scoped>
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