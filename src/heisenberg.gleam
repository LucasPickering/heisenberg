import gleam/erlang/process
import mist
import routes.{routes}

pub fn main() {
  let assert Ok(_) =
    routes
    |> mist.new
    |> mist.port(3000)
    |> mist.start

  process.sleep_forever()
}
