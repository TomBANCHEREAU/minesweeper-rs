import React from "preact/compat";
import { TitledFrame } from "../components/titled-frame";
import "./map.css";

export const Map = React.memo(function Map() {
  return (
    <TitledFrame
      title="Carte"
      style="width: 100vw;height: calc(100vh - 2px)"
      class="map"
    >
      <TitledFrame title="0 Vos vaisseaux" class="list">
        <ol>
          <ShipListItem></ShipListItem>
          <ShipListItem></ShipListItem>
          <ShipListItem></ShipListItem>
        </ol>
      </TitledFrame>
    </TitledFrame>
  );
});

const ShipListItem = React.memo(function ShipListItem() {
  // 🟢 Prêt a miner
  // 🟡 En déplacement
  // 🔵 Garage
  // 🔴 En panne
  return (
    <li class="ship-list-item">
      <a href="#">
        <div>
          <h3>Nom</h3>
          <p>🔵 En déplacement</p>
        </div>
        <div>
          <p style={"flex-grow:1"}>Voyage en direction de XXX</p>
          <p>🕓 20min</p>
        </div>
      </a>
    </li>
  );
});
