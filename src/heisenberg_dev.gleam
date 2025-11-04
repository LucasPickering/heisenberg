import gleam/erlang/process
import gleam/io
import mist
import radiate
import routes.{routes}

pub fn main() {
  // Hot reloading
  let assert Ok(_) =
    radiate.new()
    // radiate silently fails if you pass any relative path other than `.` on
    // macOS. And no one knows how to get the cwd in gleam!! crazy
    |> radiate.add_dir(".")
    |> radiate.on_reload(fn(_state, path) {
      io.println("Change in " <> path <> ", reloading")
    })
    |> radiate.start()

  // Start the HTTP server
  let assert Ok(_) =
    routes
    |> mist.new
    |> mist.port(3000)
    |> mist.start

  process.sleep_forever()
}
