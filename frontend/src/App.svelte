<script>
  import { onMount } from 'svelte';

  let response = '';

  onMount(async () => {
    try {
      const res = await fetch('http://176.123.165.131:8080/');
      if (res.ok) {
        response = await res.text();
      } else {
        console.error('Error fetching data:', res.statusText);
      }
    } catch (error) {
      console.error('Network error:', error);
    }
  });
</script>

<main>
  <h1>Bug with Bags</h1>
  <p>This string came from rust backend: {response}</p>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 2em;
    margin: 0;
  }
</style>
