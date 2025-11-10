import gleam/erlang/process
import gleam/otp/actor
import mist
import model
import routes.{routes}

pub fn main() {
  // Start data management actors. We have one actor that manages the global
  // state, then an additional actor for each data source. The fetch actors push
  // data back to the model actor
  let assert Ok(model_actor) =
    actor.new(model.global_init())
    |> actor.on_message(model.global_update)
    |> actor.start

  // Start HTTP server
  let assert Ok(_) =
    fn(req) { routes(model_actor, req) }
    |> mist.new
    |> mist.port(3000)
    |> mist.start

  process.sleep_forever()
}
