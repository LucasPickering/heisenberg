import { useEffect, useState } from "preact/hooks";
import Transit from "./Transit.tsx";
import Weather from "./Weather.tsx";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { ComponentChildren } from "preact";
import { Mode, TransitPredictions, WeatherForecast } from "./state.ts";

function App(): ComponentChildren {
  const [mode, setMode] = useState<Mode>(Mode.Weather);
  const [transit, setTransit] = useState<TransitPredictions>();
  const [weather, setWeather] = useState<WeatherForecast>();

  useEffect(() => {
    const unlisten = Promise.all(
      [
        listen<TransitPredictions>(
          Mode.Transit,
          event => setTransit(event.payload),
        ),
        listen<WeatherForecast>(
          Mode.Weather,
          event => setWeather(event.payload),
        ),
      ],
    );
    return () => unlisten.then(funcs => funcs.forEach(f => f()));
  }, []);

  return (
    <main class="container">
      <header class="header">
        <ModeTab mode={Mode.Weather} label="Weather" onClick={setMode} />
        <ModeTab mode={Mode.Transit} label="Transit" onClick={setMode} />
      </header>
      {(() => {
        switch (mode) {
          case Mode.Transit:
            return transit && <Transit transit={transit} />;
          case Mode.Weather:
            return weather && <Weather weather={weather} />;
        }
      })()}
    </main>
  );
}

function ModeTab(
  { mode, label, onClick }: {
    mode: Mode;
    label: string;
    onClick: (mode: Mode) => void;
  },
): ComponentChildren {
  return (
    <button
      class="headerButton"
      type="button"
      onClick={() => onClick(mode)}
    >
      {label}
    </button>
  );
}

export default App;
