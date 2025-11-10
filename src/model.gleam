import gleam/erlang/process
import gleam/otp/actor
import gleam/time/timestamp.{type Timestamp}

/// The actor that owns the global model
pub type ModelActor =
  actor.Started(process.Subject(GlobalMessage))

/// TODO
pub type GlobalModel {
  GlobalModel(transit: Transit, weather: Weather, last_updated: Timestamp)
}

/// Current transit data
pub type Transit {
  Transit
}

/// Current weather data
pub type Weather {
  Weather
}

/// Message from a data fetch actor back to the main actor that handles the
/// model
pub type GlobalMessage {
  /// TODO
  GetModel(reply_with: process.Subject(GlobalModel))
  UpdateTransit(Transit)
  UpdateWeather(Weather)
}

/// Get the initial global state
pub fn global_init() -> GlobalModel {
  GlobalModel(
    transit: Transit,
    weather: Weather,
    last_updated: timestamp.system_time(),
  )
}

/// Update the global state according to the given messgae
pub fn global_update(
  model: GlobalModel,
  message: GlobalMessage,
) -> actor.Next(GlobalModel, GlobalMessage) {
  // All updates require updating the timestamp as well
  let now = timestamp.system_time()
  case message {
    GetModel(reply_with) -> {
      // Someone is asking for the current model value
      process.send(reply_with, model)
      actor.continue(model)
    }
    UpdateTransit(transit) -> {
      let model = GlobalModel(..model, transit: transit, last_updated: now)
      actor.continue(model)
    }
    UpdateWeather(weather) -> {
      let model = GlobalModel(..model, weather: weather, last_updated: now)
      actor.continue(model)
    }
  }
}

/// Get the current global state
pub fn get_model(model_actor: ModelActor) -> GlobalModel {
  process.call(model_actor.data, 10, GetModel)
}
