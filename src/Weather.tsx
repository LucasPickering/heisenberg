import { ComponentChildren } from "preact";
import { WeatherForecast } from "./state.ts";

function Weather({ weather }: { weather: WeatherForecast }): ComponentChildren {
  return (
    <div>
      {weather.periods.map((period) => (
        <div>
          {period.start_time}: {period.temperature}F {period
            .probability_of_precipitation}%
        </div>
      ))}
    </div>
  );
}

export default Weather;
