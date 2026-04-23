import { render } from "preact";
import { LocationProvider, Route, Router } from "preact-iso";

import { AsteroidPage } from "./pages/asteroid.js";
import { Map } from "./pages/map.js";
import { AsteroidCreationForm } from "./pages/tmp-create-asteroid.js";
import "./style.css";

export function App() {
  return (
    <LocationProvider>
      <main>
        <aside></aside>
        {/* <button onClick={() => console.log("aa")}>Button</button>
        <button class="terminal-button" onClick={() => console.log("aa")}>
          Button
        </button>
        <a class="terminal-button" href="#">
          Link
        </a> */}
        <Router>
          <Route path="/" component={AsteroidCreationForm} />
          <Route path="/map" component={Map} />
          <Route path="/asteroid/:asteroidId" component={AsteroidPage} />
          {/* <Route default component={NotFound} /> */}
        </Router>
      </main>
    </LocationProvider>
  );
}

render(<App />, document.getElementById("app"));
