import { ComponentChildren } from "preact";
import { Line } from "preact-chartjs-2";
import { WeatherForecast } from "./state.ts";

// How far in advance to show on the chart
const TIME_SPAN_HOURS: number = 24;

function Weather({ weather }: { weather: WeatherForecast }): ComponentChildren {
  if (weather.periods.length === 0) {
    // TODO better loading icon?
    return "Weather loading...";
  }

  console.log(weather);
  return (
    <div>
      <Line data={getChartData(weather)} />
      {weather.periods.map((period) => (
        <div>
          {period.start_time}: {period.temperature}F {period
            .probability_of_precipitation}%
        </div>
      ))}
    </div>
  );
}

function getChartData(weather: WeatherForecast): unknown {
  // Filter weather periods to just the upcoming spans we care about
  const now = new Date();
  const later = new Date(now.getTime());
  later.setHours(later.getHours() + TIME_SPAN_HOURS);
  const periods = weather.periods.filter((period) =>
    now.toISOString() < period.end_time
    && period.start_time < later.toISOString()
  );

  return {
    datasets: [
      {
        label: "Temperature",
        data: periods.map((period) => period.temperature),
      },
    ],
  };
}

export default Weather;
