<script>
  import StationCard from './StationCard.svelte';
  import { onMount } from 'svelte';
  import localForage from 'localforage';

  let allStations = [];
  let currentPageStations = [];
  let currentPage = 1;
  const itemsPerPage = 10;
  let totalPages = 1;

  onMount(async () => {
    await fetchStations(currentPage);
  });

  async function fetchStations(page) {
    try {
      const cachedData = await localForage.getItem(`stationsData_${page}`);
      if (cachedData) {
        allStations = cachedData.stations;
        totalPages = cachedData.total_pages;
        updateCurrentPageStations();
        console.log('Using cached data from IndexedDB');
      } else {
        const res = await fetch(`http://176.123.165.131:8080/stations?page=${page}&page_size=${itemsPerPage}`);
        if (res.ok) {
          const data = await res.json();
          allStations = data.stations;
          totalPages = data.total_pages;
          await localForage.setItem(`stationsData_${page}`, data); // Use IndexedDB for larger data
          updateCurrentPageStations();
          console.log('Fetched and cached new data in IndexedDB');
        } else {
          console.error('Error fetching data:', res.statusText);
        }
      }
    } catch (error) {
      console.error('Error with data fetching/caching:', error);
    }
  }

  function updateCurrentPageStations() {
    currentPageStations = allStations;
  }

  async function goToPage(page) {
    currentPage = page;
    await fetchStations(page);
  }

  function changePage(offset) {
    if (currentPage + offset > 0 && currentPage + offset <= totalPages) {
      goToPage(currentPage + offset);
    }
  }
</script>

<div class="stations-grid">
  {#each currentPageStations as station}
    <StationCard {station} />
  {/each}
</div>

<div class="pagination-buttons">
  <button on:click={() => changePage(-1)} disabled={currentPage <= 1}>Previous</button>
  <button on:click={() => changePage(1)} disabled={currentPage >= totalPages}>Next</button>
</div>

<style>
  .stations-grid {
    display: grid;
    gap: 20px;
    padding: 20px;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  }

  .pagination-buttons {
    display: flex;
    justify-content: center;
    margin-top: 20px;
  }

  button {
    padding: 10px 20px;
    margin: 0 10px;
    font-size: 1rem;
    border: none;
    background-color: #ff3e00;
    color: white;
    border-radius: 5px;
    cursor: pointer;
  }

  button:disabled {
    background-color: #ccc;
    cursor: not-allowed;
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
