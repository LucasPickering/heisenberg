import React, { useMemo } from "react";
import { AxisOptions, Chart, UserSerie } from "react-charts";
import { WeatherForecast } from "./state.ts";

// How far in advance to show on the chart
const TIME_SPAN_HOURS: number = 24;
const PRIMARY_AXIS: AxisOptions<Datum> = {
  getValue: datum => datum.date,
};
const SECONDARY_AXES: AxisOptions<Datum>[] = [
  { getValue: datum => datum.temperature, min: 0, max: 100 },
  { getValue: datum => datum.pop, min: 0, max: 100 },
];

function Weather({ weather }: { weather: WeatherForecast }): React.ReactNode {
  if (weather.periods.length === 0) {
    // TODO better loading icon?
    return "Weather loading...";
  }

  const data = useMemo(() => getSeries(weather), [weather]);
  console.log(data);
  return (
    <div>
      <Chart
        options={{
          primaryAxis: PRIMARY_AXIS,
          secondaryAxes: SECONDARY_AXES,
          data,
        }}
      />
    </div>
  );
}

interface Datum {
  date: Date;
  temperature: number;
  pop: number;
}

function getSeries(weather: WeatherForecast): UserSerie<Datum>[] {
  // Filter weather periods to just the upcoming spans we care about
  const now = new Date();
  const later = new Date(now.getTime());
  later.setHours(later.getHours() + TIME_SPAN_HOURS);
  const periods = weather.periods.filter((period) =>
    now.toISOString() < period.end_time
    && period.start_time < later.toISOString()
  );

  return [
    {
      label: "Temperature",
      data: periods.map((period) => ({
        date: new Date(period.start_time),
        temperature: period.temperature,
        pop: period.probability_of_precipitation,
      })),
    },
  ];
}

export default Weather;
