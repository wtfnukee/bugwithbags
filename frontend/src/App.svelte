<script>
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
</script>

<main>
  <h1>Stations</h1>
  <div class="stations-grid">
    {#each currentPageStations as station}
      <div class="station-card">
        <h2>{station.title.length > 20 ? station.title.substr(0, 20) + '...' : station.title}</h2>
        <div class="card-content">
          <div class="card-column">
            <p><strong class="field-name">Type:</strong> {station.station_type}</p>
            <p><strong class="field-name">Longitude:</strong> {station.longitude ?? 'N/A'}</p>
            <p><strong class="field-name">Latitude:</strong> {station.latitude ?? 'N/A'}</p>
            <p><strong class="field-name">Transport:</strong> {station.transport_type ?? 'N/A'}</p>
          </div>
          <div class="card-column">
            <p>Direction: {station.direction ?? 'N/A'}</p>
            <p>ESR Code: {station.esr_code ?? 'N/A'}</p>
            <p>Yandex Code: {station.yandex_code}</p>
            <p>Country: {station.country}</p>
          </div>
          <div class="card-column">
            <p>Country Code: {station.country_code}</p>
            <p>Region: {station.region}</p>
            <p>Region Code: {station.region_code}</p>
            <p>Settlement: {station.settlement}</p>
            <p>Settlement Code: {station.settlement_code}</p>
          </div>
        </div>
      </div>
    {/each}
  </div>
  <button on:click={() => goToPage(currentPage - 1)} disabled={currentPage <= 1}>Previous</button>
  <button on:click={() => goToPage(currentPage + 1)} disabled={currentPage * itemsPerPage >= allStations.length}>Next</button>
</main>

<style>
  .stations-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 20px;
    padding: 20px;
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