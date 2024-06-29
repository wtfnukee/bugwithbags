<script>
    export let station;
  </script>
  
  <div class="station-card">
    <div class="card-content">
      <div class="card-column">
        <p><strong class="field-name">Type:</strong> {station.station_type}</p>
        <p><strong class="field-name">Longitude:</strong> {station.longitude ?? 'N/A'}</p>
        <p><strong class="field-name">Latitude:</strong> {station.latitude ?? 'N/A'}</p>
        <p><strong class="field-name">Transport:</strong> {station.transport_type ?? 'N/A'}</p>
      </div>
      <div class="card-column">
        <p><strong class="field-name">Direction:</strong> {station.direction ?? 'N/A'}</p>
        <p><strong class="field-name">ESR Code:</strong> {station.esr_code ?? 'N/A'}</p>
        <p><strong class="field-name">Yandex Code:</strong> {station.yandex_code}</p>
        <p><strong class="field-name">Country:</strong> {station.country}</p>
      </div>
      <div class="card-column">
        <p><strong class="field-name">Country Code:</strong> {station.country_code}</p>
        <p><strong class="field-name">Region:</strong> {station.region}</p>
        <p><strong class="field-name">Region Code:</strong> {station.region_code}</p>
        <p><strong class="field-name">Settlement:</strong> {station.settlement}</p>
        <p><strong class="field-name">Settlement Code:</strong> {station.settlement_code}</p>
      </div>
    </div>
  </div>
  
  <style>
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
  </style>