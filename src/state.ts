export enum Mode {
  Transit = "transit",
  Weather = "weather",
}

export interface TransitPredictions {
  lines: LinePredictions[];
}

export interface LinePredictions {
  name: string;
  stops: StopPredictions[];
}

export interface StopPredictions {
  name: string;
  predictions: number[];
}

export interface WeatherForecast {
  periods: WeatherPeriod[];
}

export interface WeatherPeriod {
  start_time: string;
  end_time: string;
  temperature: number;
  probability_of_precipitation: number;
}
