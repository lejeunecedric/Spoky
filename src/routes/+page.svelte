<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let name = $state('Spoky');
  let greetMsg = $state('');

  async function greet() {
    greetMsg = await invoke('greet', { name });
  }
</script>

<main class="container">
  <h1>Hello Spoky</h1>
  
  <div class="row">
    <input
      id="greet-input"
      placeholder="Enter a name..."
      bind:value={name}
    />
    <button onclick={greet}>Greet</button>
  </div>
  
  {#if greetMsg}
    <p class="greet-message">{greetMsg}</p>
  {/if}
</main>

<style>
  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  h1 {
    margin-bottom: 2rem;
    color: #333;
  }

  .row {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
  }

  input {
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 1rem;
  }

  button {
    padding: 0.5rem 1rem;
    background-color: #0078d4;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
  }

  button:hover {
    background-color: #006cbd;
  }

  .greet-message {
    margin-top: 2rem;
    font-size: 1.2rem;
    color: #006cbd;
  }
</style>
