<script>
  import { onMount } from 'svelte';
  let allStations = [];
  let currentPageStations = [];
  let currentPage = 1;
  const itemsPerPage = 10; // Adjust based on your needs

  onMount(async () => {
    try {
      const res = await fetch('http://176.123.165.131:8080/');
      if (res.ok) {
        const data = await res.json();
        allStations = data; // Assuming the JSON is an array of stations
        updateCurrentPageStations();
      } else {
        console.error('Error fetching data:', res.statusText);
      }
    } catch (error) {
      console.error('Network error:', error);
    }
  });

  function updateCurrentPageStations() {
    const startIndex = (currentPage - 1) * itemsPerPage;
    const endIndex = startIndex + itemsPerPage;
    currentPageStations = allStations.slice(startIndex, endIndex);
  }

  function goToPage(page) {
    currentPage = page;
    updateCurrentPageStations();
  }
</script>

<main>
  <h1>Stations</h1>
  {#each currentPageStations as station}
    <p>{station.title}</p>
  {/each}
  <button on:click={() => goToPage(currentPage - 1)} disabled={currentPage <= 1}>Previous</button>
  <button on:click={() => goToPage(currentPage + 1)} disabled={currentPage * itemsPerPage >= allStations.length}>Next</button>
</main>