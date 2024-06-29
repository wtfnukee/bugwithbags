<script>
  import StationCard from './StationCard.svelte';
  import { onMount } from 'svelte';
  let allStations = [];
  let currentPageStations = [];
  let currentPage = 1;
  const itemsPerPage = 10;

  onMount(async () => {
    try {
      const res = await fetch('http://176.123.165.131:8080/stations');
      if (res.ok) {
        const data = await res.json();
        allStations = data["stations"];
        console.log(data);
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
  function changePage(offset) {
    goToPage(currentPage + offset);
  }
</script>

{#each currentPageStations as station}
  <StationCard {station} />
{/each}

<button on:click={() => changePage(-1)} disabled={currentPage <= 1}>Previous</button>
<button on:click={() => changePage(1)} disabled={currentPage * itemsPerPage >= allStations.length}>Next</button>

<style>
  .stations-grid {
    display: grid;
    gap: 20px;
    padding: 20px;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  }

  /* Example Media Queries */
  @media (max-width: 600px) {
    .stations-grid {
      grid-template-columns: repeat(1, 1fr); /* 1 card per row for small screens */
    }
  }

  @media (min-width: 601px) and (max-width: 900px) {
    .stations-grid {
      grid-template-columns: repeat(2, 1fr); /* 2 cards per row for medium screens */
    }
  }

  @media (min-width: 901px) and (max-width: 1200px) {
    .stations-grid {
      grid-template-columns: repeat(3, 1fr); /* 3 cards per row for large screens */
    }
  }

  @media (min-width: 1201px) {
    .stations-grid {
      grid-template-columns: repeat(4, 1fr); /* 4 cards per row for extra large screens */
    }
  }
  .station-card {
    border: 1px solid #ccc;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    display: flex;
    flex-direction: column;
  }
  .card-content {
    display: flex;
    justify-content: space-between;
  }
  .card-column {
    flex: 1;
    padding: 0 10px;
  }

  .field-name {
  font-weight: bold;
  color: #ff3e00;
}

  h2 {
    font-size: 1.2rem;
    margin-bottom: 10px;
  }
  p {
    font-size: 0.9rem;
    margin: 5px 0;
  }
</style>